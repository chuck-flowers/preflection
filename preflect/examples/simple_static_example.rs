use preflect::fields::HasField;

#[derive(Debug, HasField)]
struct User {
    id: u32,
    name: String,
    age: usize,
}

fn main() {
    let mut user = User {
        id: 1,
        name: "Jim".into(),
        age: 18,
    };

    println!("Before name change: {:?}", user);
    change_name_to_bob(&mut user);
    println!("After name change: {:?}", user);
}

fn change_name_to_bob(obj: &mut impl HasField<String, "name">) {
    let name: &mut String = obj.get_field_mut();
    name.clear();
    name.push_str("Bob");
}
