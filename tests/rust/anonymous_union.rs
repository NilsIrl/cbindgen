pub struct MyStruct {
    a_normal_field: i32,
    my_union: name_doesnt_matter_because_it_is_anonymous,
}

pub struct MyStructWithAnonUnion {
    a_normal_field: i32,
    /// cbindgen:anonymous=true
    my_union: name_doesnt_matter_because_it_is_anonymous,
}

union name_doesnt_matter_because_it_is_anonymous {
    a: i32,
    b: f32,
    c: bool,
}

#[no_mangle]
pub extern "C" fn root(x: MyStructWithAnonUnion, y: MyStruct) { }

