use preflect::fields::HasField;

#[derive(HasField)]
struct User {
    id: u32,
    #[preflect(ignore = true)]
    #[allow(dead_code)]
    name: String,
}

fn main() {
    let user = User {
        id: 1,
        name: "John".into(),
    };

    let id = read_id(&user);

    // Would fail to compile:
    // let name = read_name(&user);

    println!("id = {}", id);
}

fn read_id(has_id: &dyn HasField<"id", FieldType = u32>) -> &u32 {
    has_id.get_field()
}

#[allow(dead_code)]
fn read_name(has_name: &dyn HasField<"name", FieldType = String>) -> &String {
    has_name.get_field()
}
