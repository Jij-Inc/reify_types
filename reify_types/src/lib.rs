pub use reify_types_macro::Reify;
use syn::{DataEnum, DataStruct, DeriveInput};

#[doc(hidden)]
pub use syn;

#[doc(hidden)]
pub use syn::parse_quote;

pub trait Reify {
    fn reify() -> DeriveInput;
}

pub trait ReifyEnum: Reify {
    fn reify_enum() -> DataEnum {
        let syn::Data::Enum(e) = Self::reify().data else {
            unreachable!()
        };
        e
    }
}

pub trait ReifyStruct: Reify {
    fn reify_struct() -> DataStruct {
        let syn::Data::Struct(e) = Self::reify().data else {
            unreachable!()
        };
        e
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_ok() {
        let t = trybuild::TestCases::new();
        t.pass("tests/enums.rs")
    }
}
