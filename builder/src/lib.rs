mod context;

use proc_macro::TokenStream;
use quote::quote;
use syn::{spanned::Spanned, DeriveInput};

use crate::context::BuildContext;

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);
    let context = BuildContext::from(ast);
    context.render().into()
}

