use crate::generator::c::generate_type;
use crate::parser::node_type::Type;
use crate::util::Either;

#[test]
fn test_generate_type_regular_type() {
    let t = generate_type(Either::Right(Some(Type::Int)));
    assert_eq!(t, "int")
}
