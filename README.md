# `reify_types` - Use the definition of your types after its definition

This crate provides a machinery to allow your types "reified" later;
i.e. any external crate can use the definition of the type!

This consists of two major parts:

- `Reify` trait - provides `T::reify()` static method that returns the definition of type `T` as `DeriveInput`.
  + It also comes with `ReifyStruct` and `ReifyEnum` that is specialized to structs and enums.
- `Reify` derive macro - automatically derives `Reify` and related specialized types.

With this crate, you can mimic the reification mechanism in TemplateHaskell.
Current limitations:

- Only types with `Reify` impl can be reified.
- It uses `ToTokens` and `parse_quote!` under the hood, so the performance can be poor for large types.
