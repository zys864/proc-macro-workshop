use quote::quote;
use syn::Field;

struct Fd {
    name: syn::Ident,
    ty: syn::Type,
    optional: bool,
}

pub struct BuildContext {
    name: syn::Ident,
    fields: Vec<Fd>,
}

impl BuildContext {
    pub fn render(self) -> proc_macro2::TokenStream {
        let name = &self.name;

        let builder_name = syn::Ident::new(&format!("{name}Builder"), name.span());
        let option_fields = self.gen_optional_fields();
        let methods = self.gen_methods();
        let assigns = self.gen_assign();

        quote!(
            #[derive(Debug,Default)]
            struct #builder_name {
                #(#option_fields,)*
            }
            impl #builder_name {
                #(#methods)*

                pub fn build(mut self)->Result<#name,&'static str>{
                    Ok(
                        #name{
                            #(#assigns,)*
                        }
                    )
                }
            }
            impl #name {
                fn builder() -> #builder_name {
                    Default::default()
                }
            }
        )
    }
    fn gen_optional_fields<'a>(&'a self) -> impl Iterator<Item = proc_macro2::TokenStream> + 'a {
        self.fields
            .iter()
            .map(|Fd { name, ty, .. }| quote!(#name:std::option::Option<#ty>))
    }

    fn gen_methods<'a>(&'a self) -> impl Iterator<Item = proc_macro2::TokenStream> + 'a {
        self.fields.iter().map(|Fd { name, ty, .. }| {
            quote!(
                pub fn #name(mut self, v: impl Into<#ty>) -> Self{
                    self.#name = Some(v.into());
                    self
                }
            )
        })
    }
    fn gen_assign<'a>(&'a self) -> impl Iterator<Item = proc_macro2::TokenStream> + 'a {
        self.fields.iter().map(|Fd { name, optional, .. }| {
            match &optional {
                true => quote!(#name : self.#name.take()),
                false => quote!(#name : self.#name.take().ok_or(concat!(stringify!(#name), " must be setted"))?),
            }
        })
    }
}

impl From<syn::DeriveInput> for BuildContext {
    fn from(value: syn::DeriveInput) -> Self {
        let name = value.ident;
        let fields = if let syn::Data::Struct(syn::DataStruct { fields, .. }) = value.data {
            fields
        } else {
            panic!("not impl");
        };
        let fds = fields.into_iter().map(Fd::from).collect();
        Self { name, fields: fds }
    }
}

impl From<Field> for Fd {
    fn from(f: Field) -> Self {
        let (optional, ty) = get_option_inner(&f.ty);
        Self {
            name: f.ident.unwrap(),
            ty: ty.clone(),
            optional,
        }
    }
}

fn get_option_inner<'a,'b>(ty: &'b syn::Type) -> (bool, &'a syn::Type)

where 'b:'a {
    if let syn::Type::Path(syn::TypePath {
        path: syn::Path { segments, .. },
        ..
    }) = ty
    {
        if let Some(v) = segments.iter().next() {
            if v.ident == "Option" {
                let type_ = match &v.arguments {
                    syn::PathArguments::AngleBracketed(a) => match a.args.iter().next() {
                        Some(syn::GenericArgument::Type(t)) => t,
                        _ => panic!("not impl"),
                    },
                    _ => panic!("not impl"),
                };
                return (true, type_);
            }
        };
    }
    (false, ty)
}
