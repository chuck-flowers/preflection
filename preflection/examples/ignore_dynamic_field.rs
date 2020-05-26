use preflection::fields::FieldAccessError;
use preflection::fields::FieldAccessResult;
use preflection::fields::HasFields;
use preflection::fields::HasFieldsExt;

#[derive(HasFields)]
struct User {
    id: u32,
    #[allow(dead_code)]
    #[preflection(ignore = true)]
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

fn read_id(has_id: &impl HasFieldsExt) -> FieldAccessResult<&u32> {
    has_id.get_field("id")
}

fn read_name(has_name: &impl HasFieldsExt) -> FieldAccessResult<&String> {
    has_name.get_field("name")
}
