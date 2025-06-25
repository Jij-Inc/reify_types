use reify_types::*;

#[derive(Eq, PartialEq, Debug, Reify)]
pub enum Operator {
    Add,
    Mul,
}

fn main() {
    use syn::*;

    let reified = Operator::reify();
    let DeriveInput {
        attrs,
        vis,
        ident,
        generics,
        data,
    } = reified;
    assert_eq!(ident, "Operator");
    assert!(attrs.is_empty());
    assert!(generics.params.is_empty());
    assert!(matches!(vis, Visibility::Public(_)));
    let Data::Enum(DataEnum { variants, .. }) = data else {
        panic!("Expected an enum");
    };
    assert_eq!(variants.len(), 2);
    assert_eq!(variants[0].ident, "Add");
    assert_eq!(variants[1].ident, "Mul");

    let enm = Operator::reify_enum();
    assert_eq!(enm.variants.len(), 2);
}
