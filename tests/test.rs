use teleprost::FromProtos;

#[test]
fn tests() {
    #[derive(FromProtos)]
    #[from(proto = "B")]
    struct A {
        same_field: i32,
    }
    struct B {
        same_field: i32,
    }
    let b = B { same_field: 42 };
    let a: A = b.into();
    assert_eq!(a.same_field, 42);
}
