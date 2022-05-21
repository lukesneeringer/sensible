#![cfg(test)]

use assert2::check;
use sensible::Default;

#[derive(Default)]
struct Foo {
  a: u64,
  #[default(42)]
  b: u64,
}

#[test]
fn test_integers() {
  let def = Foo::default();
  check!(def.a == 0);
  check!(def.b == 42);
}
