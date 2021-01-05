use preflect::fields::FieldAccessError;
use preflect::fields::FieldAccessResult;
use preflect::fields::HasFields;

#[derive(HasFields)]
struct User {
    id: u32,
    #[allow(dead_code)]
    #[preflect(ignore)]
    name: String,
}

fn main() {
    let user = User {
        id: 1,
        name: "John".into(),
    };

    let id_result = read_id(&user);
    let name_result = read_name(&user);

    assert_eq!(Ok(&1), id_result);
    assert_eq!(Err(FieldAccessError::MissingField), name_result);
}

fn read_id(has_id: &impl HasFields) -> FieldAccessResult<&u32> {
    has_id.get_field("id")
}

fn read_name(has_name: &impl HasFields) -> FieldAccessResult<&String> {
    has_name.get_field("name")
}
