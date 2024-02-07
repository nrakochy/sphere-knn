use proc_macro2::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn positionable(
    _metadata: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input: TokenStream = input.into();
    let output = quote! {
        #[derive(Clone, Debug, Default, Copy)]
        #input
    };
    output.into()
}
