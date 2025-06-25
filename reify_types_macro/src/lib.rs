mod derive;

#[proc_macro_derive(Reify)]
pub fn reify(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    derive::derive(input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
