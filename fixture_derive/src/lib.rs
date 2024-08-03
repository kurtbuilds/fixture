use proc_macro::{self, TokenStream};
use quote::{quote, ToTokens};
use syn::{parse_macro_input, DeriveInput, Data};

enum FieldType {
    Type(syn::Type),
    StaticStr,
}

#[proc_macro_derive(Fixture)]
pub fn derive(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, vis, .. } = parse_macro_input!(input);
    let mut name = ident.to_string();
    name.push_str("Fixture");

    let fixture_ident = syn::Ident::new(&name, ident.span());
    let Data::Struct(d) = data else {
        panic!("Fixture derive only supports structs");
    };
    let fields = d.fields.into_iter().map(|f| {
        let field_name = f.ident.unwrap();
        let ty = f.ty;
        if ty.to_token_stream().to_string() == "Option < String >" {
            (field_name, FieldType::StaticStr)
        } else {
            (field_name, FieldType::Type(ty))
        }
    }).collect::<Vec<_>>();

    let struct_fields = fields.iter().map(|(field_name, ty)| {
        // check if f.ty is Option<String>
        match ty {
            FieldType::Type(ty) => {
                quote! {
                    pub #field_name: #ty
                }
            }
            FieldType::StaticStr => {
                quote! {
                    pub #field_name: &'static str
                }
            }
        }
    });
    let struct_code = quote! {
        #[derive(Debug, Default)]
        #vis struct #fixture_ident {
            #(#struct_fields),*
        }
    };

    let field_comparison = fields.iter().map(|(name, ty)| {
        match ty {
            FieldType::Type(_) => {
                quote! {
                    self.#name == other.#name
                }
            }
            FieldType::StaticStr => {
                quote! {
                    self.#name.as_deref().unwrap_or_default() == other.#name
                }
            }
        }
    });
    let partial_eq_code = quote! {
        impl PartialEq<#fixture_ident> for #ident {
            fn eq(&self, other: &#fixture_ident) -> bool {
                #(#field_comparison) &&*
            }
        }
    };
    let output = quote! {
        #struct_code
        #partial_eq_code
    };
    output.into()
}