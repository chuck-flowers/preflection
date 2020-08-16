use preflect::fields::HasFields;
use preflect::fields::HasFieldsExt;

#[derive(Debug, HasFields)]
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

fn change_name_to_bob(obj: &mut impl HasFieldsExt) {
    let name: &mut String = obj.get_field_mut("name").unwrap();
    name.clear();
    name.push_str("Bob");
}
