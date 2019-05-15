extern crate proc_macro;

//use proc_macro2::TokenStream;
use quote::quote;
use proc_macro::TokenStream;

#[proc_macro]
pub fn component(input: TokenStream) -> TokenStream {
    let result = quote! {
        TestComponent::build().create()
    };
    result.into()
}