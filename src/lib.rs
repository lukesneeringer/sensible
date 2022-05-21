#![doc = include_str!(concat!(env!("OUT_DIR"), "/README-rustdocified.md"))]

extern crate proc_macro;

use darling::ast;
use darling::FromDeriveInput;
use darling::FromField;
use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use syn::parse_macro_input;
use syn::DeriveInput;

/// Derive a [`std::default::Default`] implementation, using customized values
/// where provided.
///
/// ## Examples
///
/// Literal values may be used:
///
/// ```
/// use sensible::Default;
///
/// #[derive(Default)]
/// struct Foo {
///   a: u64,
///   #[default(42)]
///   b: u64,
/// }
///
/// let foo = Foo::default();
/// assert_eq!(foo.a, 0);
/// assert_eq!(foo.b, 42);
/// ```
///
/// Expressions may also be used where needed:
///
/// ```
/// use sensible::Default;
///
/// #[derive(Default)]
/// struct Foo {
///   a: u64,
///   #[default("bacon".into())]
///   b: String,
/// }
///
/// let foo = Foo::default();
/// assert_eq!(foo.a, 0);
/// assert_eq!(foo.b, "bacon");
/// ```
///
/// Defaults can be nested (regardless of how `Default` is implemented), and
/// `default` attributes persist:
///
/// ```
/// use sensible::Default;
///
/// #[derive(Default)]
/// struct Outer {
///   a: Inner,
///   #[default(Inner{ d: 76 })]
///   b: Inner,
///   #[default(12)]
///   c: u32,
/// }
///
/// #[derive(Default)]
/// struct Inner {
///   #[default(42)]
///   d: u32,
/// }
///
/// let outer = Outer::default();
/// assert_eq!(outer.a.d, 42);
/// assert_eq!(outer.b.d, 76);
/// assert_eq!(outer.c, 12);
/// ```
#[proc_macro_derive(Default, attributes(default))]
pub fn derive_default(in_: TokenStream1) -> TokenStream1 {
  Struct::from_derive_input(&parse_macro_input!(in_ as DeriveInput))
    .map(|receiver| quote!(#receiver))
    .unwrap_or_else(|err| err.write_errors())
    .into()
}

#[derive(FromDeriveInput)]
#[darling(supports(struct_named))]
struct Struct {
  ident: syn::Ident,
  generics: syn::Generics,
  data: ast::Data<(), Field>,
}

impl ToTokens for Struct {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let (impl_gcs, ty_gcs, where_cl) = self.generics.split_for_impl();
    let ident = &self.ident;
    let fields = &self
      .data
      .as_ref()
      .map_struct_fields(|f| quote! {#f})
      .take_struct()
      .unwrap()
      .fields;
    tokens.extend(quote! {
      impl #impl_gcs ::core::default::Default for #ident #ty_gcs #where_cl {
        /// Return a sensible default object.
        fn default() -> Self {
          Self {
            #(#fields)*
          }
        }
      }
    });
  }
}

#[derive(FromField)]
#[darling(forward_attrs(default))]
struct Field {
  ident: Option<syn::Ident>,
  attrs: Vec<syn::Attribute>,
}

impl ToTokens for Field {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let ident = &self.ident;
    if self.attrs.is_empty() {
      tokens.extend(quote! { #ident: ::core::default::Default::default(), });
    }
    else {
      let expr: syn::Expr =
        self.attrs[0].parse_args().expect("Could not parse default.");
      tokens.extend(quote! { #ident: #expr, });
    }
  }
}
