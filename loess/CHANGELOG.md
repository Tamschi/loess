# Loess Changelog

## next-major

TODO: Date

## Breaking changes

- Signature change: [`Input::peek`](https://docs.rs/loess/0.2/loess/struct.Input.html#method.peek)

  The callback now is given a `vec_deque::Iter` as second argument to examine further tokens if needed.

- Signature change: [`Input::pop_or_replace`](https://docs.rs/loess/0.2/loess/struct.Input.html#method.peek)

  The callback now is given the `&mut Input` as second parameter to examine or consume further tokens if needed.

- Removed: `impl ToTokens for Identifier`

  I had forgotten to remove this before publishing the first version.

  `quote` is now an optional dependency with `default-features = false`, only required by the `"opaque_rust_grammar"` feature.

## Revisions

- [`rust_grammar::DotDot`](https://docs.rs/loess/0.2/loess/rust_grammar/struct.DotDot.html) is now parsed more accurately: It must either be spaced or not followed by `.` or `=`.

- Small documentation revision.

## 0.1.1

2025-04-21

### Features

- Added: [`Input::peek(&self, f)`](https://docs.rs/loess/0.1/loess/struct.Input.html#method.peek)

- Added: [`impl PeekFrom for RArrow`](https://docs.rs/loess/0.1/loess/rust_grammar/struct.RArrow.html#impl-PeekFrom-for-RArrow) (`->`)

- Added: [`Eager<T>(pub T)`](https://docs.rs/loess/0.1/loess/struct.Eager.html)

  This struct can be wrapped around `T` that are `IntoIterator<Item: PeekFrom + PopFrom>` and also `FromIterator` regarding that same item type. It parses repeated values eagerly but stops without error when it detects that the value doesn't repeat.

  (Note that delimited groups still independently raise errors for unconsumed tokens when parsed directly.)

- Added: [`quote_into_mixed_site!`](https://docs.rs/loess/0.1/loess/macro.quote_into_mixed_site.html) (recommended), [`quote_into_with_exact_span!`](https://docs.rs/loess/0.1/loess/macro.quote_into_with_exact_span.html) and [`quote_into_call_site!`](https://docs.rs/loess/0.1/loess/macro.quote_into_call_site.html)

  These statement macros take `span`, `root`, `tokens` and a bracketed `[…]` group as input, separated by commas.

  Inside the bracketed group, most tokens are translated directly into the output, but you can directives that paste [`IntoTokens`](https://docs.rs/loess/0.1/loess/trait.IntoTokens.html) values into the output or expand to control flow statements. You can find more information in the [`quote_into_mixed_site!`](https://docs.rs/loess/0.1/loess/macro.quote_into_mixed_site.html) documentation.

- Added: [`raw_quote_into_mixed_site!`](https://docs.rs/loess/0.1/loess/macro.raw_quote_into_mixed_site.html) (recommended), [`raw_quote_into_with_exact_span!`](https://docs.rs/loess/0.1/loess/macro.raw_quote_into_with_exact_span.html) and [`raw_quote_into_call_site!`](https://docs.rs/loess/0.1/loess/macro.raw_quote_into_call_site.html)

  These statement macros quote tokens without processing directives, and as such don't accept a `root` parameter. Use them to efficiently emit static code. (Note that the `{#raw … }` directive has the same effect within other `quote_into…` macros.)

### Revisions

Various small documentation additions.

## 0.1.0

2025-04-16

Initial release.
