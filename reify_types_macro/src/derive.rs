use proc_macro2::*;
use quote::quote;
use syn::*;

pub(crate) fn derive(input: DeriveInput) -> Result<TokenStream> {
    let mut tokens = quote! {};
    let ty_ = &input.ident;
    tokens.extend(quote! {
        impl ::reify_types::Reify for #ty_ {
            fn reify() -> ::reify_types::syn::DeriveInput {
                ::reify_types::parse_quote! {
                    #input
                }
            }
        }
    });
    match input.data {
        Data::Struct(_) => tokens.extend(quote! {
            impl ::reify_types::ReifyStruct for #ty_ {}
        }),
        Data::Enum(_) => tokens.extend(quote! {
            impl ::reify_types::ReifyEnum for #ty_ {}
        }),
        Data::Union(_) => (),
    }
    Ok(tokens)
}
