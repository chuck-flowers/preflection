use preflect::drop::PartialDrop;
use std::any::type_name;

#[derive(PartialDrop)]
struct Example {
    first: LogDrop<u32>,
    second: LogDrop<f32>,
    third: LogDrop<i32>,
}

struct LogDrop<T>(T);

impl<T> Drop for LogDrop<T> {
    fn drop(&mut self) {
        println!("Dropping {}", type_name::<T>())
    }
}

fn main() {
    let data = Example {
        first: LogDrop(0),
        second: LogDrop(0.0),
        third: LogDrop(0),
    };

    println!("Partial dropping...");
    let mut data = ::core::mem::ManuallyDrop::new(data);
    unsafe {
        data.drop_all_fields_except(&["first"]);
    }

    println!("Manually dropping...");
    unsafe {
        let first: *mut _ = &mut data.first;
        ::core::ptr::drop_in_place(first);
    }

    println!("Done!");
}
