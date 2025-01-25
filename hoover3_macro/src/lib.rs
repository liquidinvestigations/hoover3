use proc_macro::TokenStream;


#[proc_macro_attribute]
pub fn activity(_attr: TokenStream, item: TokenStream) -> TokenStream {
    hoover3_macro2::activity(_attr.into(), item.into()).into()
}

#[proc_macro_attribute]
pub fn workflow(_attr: TokenStream, item: TokenStream) -> TokenStream {
    hoover3_macro2::workflow(_attr.into(), item.into()).into()
}