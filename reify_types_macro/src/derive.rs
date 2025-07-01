use proc_macro2::*;
use quote::quote;
use syn::*;

pub(crate) fn derive(input: DeriveInput) -> Result<TokenStream> {
    let mut tokens = quote! {};
    let ty_ = &input.ident;
    let (impl_gen, ty_gen, wheres) = input.generics.split_for_impl();
    tokens.extend(quote! {
        impl #impl_gen ::reify_types::Reify for #ty_ #ty_gen #wheres {
            fn reify() -> ::reify_types::syn::DeriveInput {
                ::reify_types::parse_quote! {
                    #input
                }
            }
        }
    });
    match input.data {
        Data::Struct(_) => tokens.extend(quote! {
            impl #impl_gen  ::reify_types::ReifyStruct for #ty_ #ty_gen #wheres {}
        }),
        Data::Enum(_) => tokens.extend(quote! {
            impl #impl_gen ::reify_types::ReifyEnum for #ty_ #ty_gen #wheres {}
        }),
        Data::Union(_) => (),
    }
    Ok(tokens)
}
