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

fn read_id<'a>(has_id: &'a dyn HasField<u32, "id">) -> &'a u32 {
    has_id.get_field()
}

#[allow(dead_code)]
fn read_name<'a>(has_name: &'a dyn HasField<String, "name">) -> &'a String {
    has_name.get_field()
}
