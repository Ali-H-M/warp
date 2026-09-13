use std::iter;
use std::ops::Range;

use arborium::tree_sitter::{Node, Query, QueryCursor, TextProvider, Tree};
use rangemap::RangeMap;
use streaming_iterator::StreamingIterator;
use string_offset::{ByteOffset, CharOffset};
use warp_editor::content::buffer::{Buffer, ToBufferByteOffset, ToBufferCharOffset};
use warp_editor::content::text::Bytes;
use warpui_core::color::ColorU;

/// Color mapping from parsed syntax token name to its corresponding highlighting color.
#[derive(Clone, Copy)]
pub struct ColorMap {
    pub keyword_color: ColorU,
    pub keyword_control_color: ColorU,
    pub function_color: ColorU,
    pub method_color: ColorU,
    pub string_color: ColorU,
    pub escape_color: ColorU,
    pub number_color: ColorU,
    pub boolean_color: ColorU,
    pub constant_color: ColorU,
    pub type_color: ColorU,
    pub type_builtin_color: ColorU,
    pub comment_color: ColorU,
    pub property_color: ColorU,
    pub variable_color: ColorU,
    pub parameter_color: ColorU,
    pub tag_color: ColorU,
    pub attribute_color: ColorU,
    pub punctuation_color: ColorU,
    pub operator_color: ColorU,
    pub namespace_color: ColorU,
    pub label_color: ColorU,
}

/// Query for retrieving syntax highlighting information on the tokens.
pub struct HighlightQuery {
    highlight_map: Vec<Option<ColorU>>,
}

impl HighlightQuery {
    pub fn new(query: &Query, color_map: ColorMap) -> Self {
        let highlight_map = query
            .capture_names()
            .iter()
            .map(|name| convert_capture_name_to_color(name, &color_map))
            .collect();

        Self { highlight_map }
    }

    /// Given the a character range, return its corresponding highlight colors.
    pub fn get_highlighted_chunks(
        &self,
        range: Range<CharOffset>,
        query: &Query,
        buffer: &Buffer,
        tree: &Tree,
    ) -> RangeMap<CharOffset, ColorU> {
        let mut range_map = RangeMap::new();

        let mut cursor = QueryCursor::new();
        let byte_start = range.start.to_buffer_byte_offset(buffer).as_usize();
        let byte_end = range.end.to_buffer_byte_offset(buffer).as_usize();
        cursor.set_byte_range(byte_start..byte_end);
        let mut captures = cursor.captures(query, tree.root_node(), TextBuffer(buffer));

        while let Some(matches) = captures.next() {
            for cap in matches.0.captures {
                let insertion_range = cap.node.byte_range();
                let color = self
                    .highlight_map
                    .get(cap.index as usize)
                    .and_then(|inner| *inner);

                if let Some(color) = color {
                    let char_start =
                        ByteOffset::from(insertion_range.start).to_buffer_char_offset(buffer);
                    let char_end =
                        ByteOffset::from(insertion_range.end).to_buffer_char_offset(buffer);
                    if char_start < char_end {
                        range_map.insert(char_start..char_end, color);
                    }
                }
            }
        }

        range_map
    }
}

fn convert_capture_name_to_color(name: &str, color_map: &ColorMap) -> Option<ColorU> {
    // Multi-segment names checked here take priority over the first-segment fallback below.
    match name {
        "text.title" => return Some(color_map.keyword_color),
        "text.literal" => return Some(color_map.string_color),
        "text.uri" => return Some(color_map.function_color),
        "text.reference" => return Some(color_map.property_color),
        "keyword.control" | "keyword.control.conditional" | "keyword.control.repeat"
        | "keyword.control.return" | "keyword.control.import" | "keyword.control.exception" => {
            return Some(color_map.keyword_control_color);
        }
        "function.method" | "function.method.call" => return Some(color_map.method_color),
        "string.escape" | "escape" => return Some(color_map.escape_color),
        "constant.builtin" | "boolean" | "constant.builtin.boolean" => {
            return Some(color_map.boolean_color);
        }
        "type.builtin" => return Some(color_map.type_builtin_color),
        "variable.parameter" | "parameter" => return Some(color_map.parameter_color),
        "tag.attribute" | "attribute" => return Some(color_map.attribute_color),
        _ => {}
    }
    match name.split('.').next() {
        Some("keyword") | Some("storageclass") | Some("preproc") => Some(color_map.keyword_color),
        Some("conditional") | Some("repeat") | Some("exception") | Some("include")
        | Some("throw") | Some("try") | Some("catch") | Some("finally") => {
            Some(color_map.keyword_control_color)
        }
        Some("function") | Some("constructor") => Some(color_map.function_color),
        Some("string") | Some("character") => Some(color_map.string_color),
        Some("type") | Some("interface") | Some("protocol") | Some("implementation") => {
            Some(color_map.type_color)
        }
        Some("number") | Some("float") => Some(color_map.number_color),
        Some("constant") => Some(color_map.constant_color),
        Some("comment") | Some("spell") => Some(color_map.comment_color),
        Some("property") | Some("field") => Some(color_map.property_color),
        Some("variable") => Some(color_map.variable_color),
        Some("tag") | Some("selector") => Some(color_map.tag_color),
        Some("punctuation") | Some("delimiter") => Some(color_map.punctuation_color),
        Some("operator") => Some(color_map.operator_color),
        Some("namespace") | Some("module") => Some(color_map.namespace_color),
        Some("label") => Some(color_map.label_color),
        _ => None,
    }
}

// The default tree-sitter implementation here is unsafe (since the cursor could query invalid ranges outside of content length).
// TODO(kevin): Once we migrate buffer to store ArrayStrings. We should implement the chunks API on buffer directly to avoid collecting
// into a String and then chunking them again for highlighting.
pub struct TextSlice<'a>(pub &'a [u8]);

impl TextSlice<'_> {
    fn get(&self, range: Range<usize>) -> Self {
        Self(self.0.get(range).unwrap_or_default())
    }
}

impl AsRef<[u8]> for TextSlice<'_> {
    fn as_ref(&self) -> &[u8] {
        self.0
    }
}

impl<'a> TextProvider<TextSlice<'a>> for TextSlice<'a> {
    type I = iter::Once<TextSlice<'a>>;

    fn text(&mut self, node: Node) -> Self::I {
        iter::once(self.get(node.byte_range()))
    }
}

pub struct TextBuffer<'a>(pub &'a Buffer);

impl<'a> TextProvider<&'a [u8]> for TextBuffer<'a> {
    type I = Bytes<'a>;

    fn text(&mut self, node: Node) -> Self::I {
        let range = node.range();
        self.0.bytes_in_range(
            ByteOffset::from(range.start_byte),
            ByteOffset::from(range.end_byte),
        )
    }
}

#[cfg(test)]
#[path = "highlight_query_tests.rs"]
mod tests;
