# Loess-Rust-Opaque

`loess-rust-opaque` is a companion crate to [`loess-rust`](https://crates.io/crates/loess-rust) that contains additional *opaque* grammar types not yet implemented natively for [Loess](https://crates.io/crates/loess). In practice, here that usually means Syn-wrappers.

Note that this crate will **accidentally** accept unstable grammar. Narrowing that towards stable Rust grammar only is not considered a breaking change.

When a part of the grammar becomes available through `loess-rust`, the version available through `loess-rust-opaque` is marked deprecated. Where compatible, which should usually be the case, the latter may also be switched over to that implementation as a non-breaking change.

How to read the version number:

After the "+", the most-major version of the (most directly) compatible Loess releases is listed.
