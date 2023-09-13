# loess

[![Lib.rs](https://img.shields.io/badge/Lib.rs-*-84f)](https://lib.rs/crates/loess)
[![Crates.io](https://img.shields.io/crates/v/loess)](https://crates.io/crates/loess)
[![Docs.rs](https://docs.rs/loess/badge.svg)](https://docs.rs/loess)

![Rust 1.56](https://img.shields.io/static/v1?logo=Rust&label=&message=1.56&color=grey)
[![CI](https://github.com/Tamschi/loess/workflows/CI/badge.svg?branch=develop)](https://github.com/Tamschi/loess/actions?query=workflow%3ACI+branch%3Adevelop)
![Crates.io - License](https://img.shields.io/crates/l/loess/0.0.1)

[![GitHub](https://img.shields.io/static/v1?logo=GitHub&label=&message=%20&color=grey)](https://github.com/Tamschi/loess)
[![open issues](https://img.shields.io/github/issues-raw/Tamschi/loess)](https://github.com/Tamschi/loess/issues)
[![open pull requests](https://img.shields.io/github/issues-pr-raw/Tamschi/loess)](https://github.com/Tamschi/loess/pulls)
[![good first issues](https://img.shields.io/github/issues-raw/Tamschi/loess/good%20first%20issue?label=good+first+issues)](https://github.com/Tamschi/loess/contribute)

[![crev reviews](https://web.crev.dev/rust-reviews/badge/crev_count/loess.svg)](https://web.crev.dev/rust-reviews/crate/loess/)
[![Zulip Chat](https://img.shields.io/endpoint?label=chat&url=https%3A%2F%2Fiteration-square-automation.schichler.dev%2F.netlify%2Ffunctions%2Fstream_subscribers_shield%3Fstream%3Dproject%252Floess)](https://iteration-square.schichler.dev/#narrow/stream/project.2Floess)

A group-lazy defaulting speculative Rust parser.

The crate structure mirrors roughtly [The Rust Reference](https://doc.rust-lang.org/stable/reference/)'s.

## Installation

Please use [cargo-edit](https://crates.io/crates/cargo-edit) to always add the latest version of this library:

```cmd
cargo add loess
```

## Example

```rust
// TODO_EXAMPLE
```

## See Also

- [matklad - Resilient LL Parsing Tutorial](https://matklad.github.io/2023/05/21/resilient-ll-parsing-tutorial.html)

  A friend pointed this out, and it's likely a better explanation of resilient parsing than what you'd get from looking through my code.

  I'm just winging this crate, but in terms of this article, I'm going with the "errors are side-effects of the parsing process" approach here to save myself a bunch of typing without macros.

## License

Licensed under either of

- Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

See [CONTRIBUTING](CONTRIBUTING.md) for more information.

## [Code of Conduct](CODE_OF_CONDUCT.md)

## [Changelog](CHANGELOG.md)

## Versioning

`loess` strictly follows [Semantic Versioning 2.0.0](https://semver.org/spec/v2.0.0.html) with the following exceptions:

- The minor version will not reset to 0 on major version changes (except for v1).  
Consider it the global feature level.
- The patch version will not reset to 0 on major or minor version changes (except for v0.1 and v1).  
Consider it the global patch level.

This includes the Rust version requirement specified above.  
Earlier Rust versions may be compatible, but this can change with minor or patch releases.

Which versions are affected by features and patches can be determined from the respective headings in [CHANGELOG.md](CHANGELOG.md).

Note that dependencies of this crate may have a more lenient MSRV policy!
Please use `cargo +nightly update -Z minimal-versions` in your automation if you don't generate Cargo.lock manually (or as necessary) and require support for a compiler older than current stable.
