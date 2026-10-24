# Third-party notices and acknowledgments

## CodeMirror 6

Ferrum is a Rust implementation of the architecture and behavior of [CodeMirror 6](https://codemirror.net/), created by Marijn Haverbeke and its contributors. The design of Ferrum's foundation (immutable state, transactions, facets, range sets, decorations, the view model, language infrastructure and editing commands) follows CodeMirror's, and parts of Ferrum are ported from CodeMirror's source code and tests. Those parts are a derived work of CodeMirror and remain subject to its license, reproduced below.

We are grateful to Marijn Haverbeke and to everyone who has contributed to CodeMirror. Its careful design, its readable source and its extensive tests are what made this project possible. If Ferrum is useful to you, consider supporting CodeMirror's development: <https://codemirror.net/>.

Packages Ferrum ports from (all under the same license): `@codemirror/state`, `@codemirror/view`, `@codemirror/language`, `@codemirror/commands`, and, as they are ported, other `@codemirror/*` packages. Their source: <https://code.haverbeke.berlin/codemirror>.

```
MIT License

Copyright (C) 2018-2021 by Marijn Haverbeke <marijn@haverbeke.berlin> and others

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.
```

(Text copied from `LICENSE` in each of those packages; identical in `state`, `view`, `language` and `commands`.)

## Code vendored in this repository

- `patches/xattr-0.2.3/`: a copy of the `xattr` 0.2.3 crate with one local change (`src/sys/mod.rs`: defines `ENOATTR` as `ENODATA` on Linux), used through `[patch.crates-io]` in `Cargo.toml`. It stays under its own licenses (`patches/xattr-0.2.3/LICENSE-MIT`, `patches/xattr-0.2.3/LICENSE-APACHE`).

## Dependencies

Ferrum's crates use third-party Rust crates through Cargo; each keeps its own license, listed in its package metadata. At the time of writing, every dependency of the foundation crates (`ferrum-state`, `ferrum-view`, `ferrum-language`, `ferrum-commands`, `ferrum`) is licensed under MIT, Apache-2.0, or a choice between them. The GPUI surface (`ferrum-view-gpui`) also pulls in crates under MPL-2.0 and crates that offer a choice of licenses; none of them changes the license of Ferrum's own code.

Applications that distribute binaries built with Ferrum must include the license notices of the dependencies they ship. A tool such as [`cargo-about`](https://github.com/EmbarkStudios/cargo-about) can generate that list from `Cargo.lock`.
