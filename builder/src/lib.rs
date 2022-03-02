use proc_macro::TokenStream;
use quote::quote;
use syn::{spanned::Spanned, DeriveInput};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);
    match expand(&ast) {
        Ok(tokens) => tokens.into(),
        Err(e) => e.to_compile_error().into(),
    }
}
fn expand(ast: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let struct_ident = &ast.ident;

    let builder_name = format!("{}Builder", ast.ident.to_string());
    let buidler_ident = syn::Ident::new(&builder_name, ast.span());
    let tokens = quote! {
        pub struct #buidler_ident{

        }
        impl #struct_ident {
            pub fn builder()-> #buidler_ident{
                #buidler_ident{}
            }
        }
    };
    Ok(tokens)
}
