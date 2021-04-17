//! Attributes to override the visibility of items.
//!
//! ## Example
//!
//! ```Rust
//! #[visible::StructFields(pub(crate))]
//! pub struct Test {
//!     pub a: i32,
//!     pub b: i64,
//! }
//! ```
//!
//! The struct `Test` will be rewritten as below:
//!
//! ```Rust
//! pub struct Test {
//!     pub(crate) a: i32,
//!     pub(crate) b: i64,
//! }
//! ```

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse_macro_input, Fields, Item, ItemStruct, Visibility};

/// **Override**s the visibility of the annotated struct fields with the one given to
/// this attribute:
///
/// ## Example
///
/// ```rust
/// #[visible::StructFields(pub(crate))]
/// pub struct Test {
///    pub a: i32,
///    pub b: i64,
/// }
/// ```
#[allow(non_snake_case)]
#[proc_macro_attribute]
pub fn StructFields(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let visibility: Visibility = parse_macro_input!(attrs);
    let mut input: Item = parse_macro_input!(input);

    if let Item::Struct(ItemStruct { ref mut fields, .. }) = input {
        match fields {
            Fields::Named(fields) => {
                for field in &mut fields.named {
                    field.vis = visibility.clone();
                }
            }
            Fields::Unnamed(fields) => {
                for field in &mut fields.unnamed {
                    field.vis = visibility.clone();
                }
            }
            Fields::Unit => {}
        }
    }

    input.into_token_stream().into()
}
