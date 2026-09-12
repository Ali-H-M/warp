`src/dark_modern.rs`'s `DARK_MODERN` color values are derived from [Visual Studio Code](https://github.com/microsoft/vscode)'s
built-in "Dark Modern" theme, which inherits its token colors from "Dark+"
(`extensions/theme-defaults/themes/dark_modern.json` and `dark_plus.json`, which in turn extends
`dark_vs.json`), pinned to commit `3addbda66f9e80c3ed1b943822ab823bb6747b02` (2026-09-12).

No files were copied; the hex color values were read from those theme JSON files and mapped onto
this crate's own `ColorMap` fields (which are keyed by tree-sitter capture name, not by VS Code's
TextMate scope names), so the mapping is a manual, semantic translation rather than a mechanical
port.

Visual Studio Code is distributed under the MIT License:

```
MIT License

Copyright (c) 2015 - present Microsoft Corporation

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```
