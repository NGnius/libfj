//! Macro implementation for using #[derive(Parsable)]
extern crate proc_macro;

use proc_macro::{TokenStream};
use syn::{DeriveInput, Data};
use quote::quote;

/// Macro generator
#[proc_macro_derive(Parsable)]
pub fn derive_parsable(struc: TokenStream) -> TokenStream {
    let ast: &DeriveInput = &syn::parse(struc).unwrap();
    let name = &ast.ident;
    if let Data::Struct(data_struct) = &ast.data {
        let mut p_fields_gen = vec![];
        let mut d_fields_gen = vec![];
        for field in &data_struct.fields {
            let field_ident = &field.ident.clone().expect("Expected named field");
            let field_type = &field.ty;
            p_fields_gen.push(
                quote! {
                    #field_ident: <#field_type>::parse(data)?
                }
            );
            d_fields_gen.push(
                quote! {
                    self.#field_ident.dump(data)?;
                }
            );
        }
        let final_gen = quote! {
            impl Parsable for #name {
                fn parse(data: &mut dyn std::io::Read) -> std::io::Result<Self> {
                    Ok(Self{
                        #(#p_fields_gen),*
                    })
                }

                fn dump(&self, data: &mut dyn std::io::Write) -> std::io::Result<usize> {
                    let mut write_count: usize = 0;
                    #(write_count += #d_fields_gen;)*
                    Ok(write_count)
                }
            }
        };
        return final_gen.into();
    } else {
        panic!("Expected Parsable auto-trait to be applied to struct");
    }
}
