[![github]](https://github.com/usagi/eles)&ensp;[![crates-io]](https://crates.io/crates/eles)&ensp;[![docs-rs]](https://docs.rs/eles)<br>
[![Build Status](https://travis-ci.org/usagi/eles.svg?branch=master)](https://travis-ci.org/usagi/eles)

[github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K

# ELES; Extensible Logical Expression Solver

If you have an existing parser that outputs a single boolean value and you want to add support for AND, OR, XOR, and parentheses, ELES can be of great help. For example, if you already have a system that interprets string syntax to determine/search data for a single item, and you want to support AND/OR searches for multiple items or complex conditions assembled using parentheses, ELES is the usefull library for you.

## Example

```rust
// The simplest example
fn main()
{
 let input = "true && (( false ^^ true && !! false )) || false";
 let literal_parser = |literal: &str| Ok(literal.parse::<bool>().unwrap_or_default());
 let r = eles::solve(&input, literal_parser);
 println!("{:?}", r);
}
```

More examples are here:

- [examples/](examples/)
  - [practical_use_case.rs](examples/practical_use_case.rs)
  - [change_logical_operation_tokens.rs](examples/change_logical_operation_tokens.rs)
  - [basic.rs](examples/basic.rs)
  - [simplest.rs](examples/simplest.rs)

And these tests might be usefull that study usage:

- [tests/](tests/)
  - [basics.rs](tests/basics.rs)

## LICENSE

- [MIT](LICENSE.md)

## Author

- USAGI.NETWORK / Usagi Ito <https://github.com/usagi/>
