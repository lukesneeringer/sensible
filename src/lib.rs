#![doc = include_str!(concat!(env!("OUT_DIR"), "/README-rustdocified.md"))]

use proc_macro::TokenStream as TokenStream1;
use darling::FromDeriveInput;
use syn::Ident;
use syn::Generics;

/// Derive a [`std::default::Default`] implementation, using customized values where provided.
#[proc_macro_derive(Default, attributes(default))]
pub fn derive_default(in_: TokenStream1) -> TokenStream1 {

}

#[derive(FromDeriveInput)]
#[darling(supports(struct_named))]
struct Receiver {
  ident: Ident,
  generics: Generics,
  data: ast::Data<(), ReceiverField>,
}
