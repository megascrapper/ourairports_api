use proc_macro::TokenStream;

use quote::quote;
use syn::DeriveInput;

#[proc_macro_derive(ContainsLocation)]
pub fn contains_location_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_contains_location(&ast)
}

fn impl_contains_location(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl ContainsLocation for #name {
            fn latitude_deg(&self) -> Latitude {
                self.latitude_deg
            }
            fn longitude_deg(&self) -> Longitude {
                self.longitude_deg
            }
            fn elevation_ft(&self) -> Elevation {
                self.elevation_ft
            }
        }
    };
    gen.into()
}