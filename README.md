# sensible

This is a crate for a more configurable derive macro to derive an
implementation for [`Default`][]. Rust's [`std::default`][] module provides a
derive implementation, but for structs, it requires that:

- Every member on the struct implement `Default`.
- Every member on the struct must use the `Default` implementation.

This is fine for many cases, but consider the case of a large struct where just
one or two fields should default to something other than their type's `Default`
implementation: now you have to implement the whole thing.

`sensible` provides a configurable derive macro that allows giving certain
fields alternate default values.

## Example

```rs
use sensible::Default;

#[derive(Default)]
struct Foo {
  a: u32,  // default value: 0

  #[default(42)]
  b: u32,  // default value: 42
}
```

[`default`]: https://doc.rust-lang.org/std/default/trait.Default.html
[`std::default`]: https://doc.rust-lang.org/std/default/index.html
