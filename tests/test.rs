use teleprost::FromProtos;

#[test]
fn tests() {
    #[derive(FromProtos)]
    struct A {
        a: i32,
    }
    struct B {
        b: i32,
    }
    let b = B { b: 42 };
    let a: A = b.into();
    assert_eq!(a.a, 42);
}
