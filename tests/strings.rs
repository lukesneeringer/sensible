#![cfg(test)]

use assert2::check;
use sensible::Default;

#[derive(Default)]
struct Foo {
  a: String,
  #[default("foo".into())]
  b: String,
}

#[test]
fn test_strings() {
  let def = Foo::default();
  check!(def.a == "");
  check!(def.b == "foo");
}
