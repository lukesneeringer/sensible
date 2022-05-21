#![cfg(test)]

use assert2::check;
use sensible::Default;

#[derive(Default)]
struct Outer {
  a: Inner,
  #[default(Inner { c: 76 })]
  b: Inner,
}

#[derive(Default)]
struct Inner {
  #[default(42)]
  c: u64,
}

#[test]
fn test_nested() {
  let def = Outer::default();
  check!(def.a.c == 42);
  check!(def.b.c == 76);
}
