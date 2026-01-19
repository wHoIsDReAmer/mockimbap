#[mockimbap::mockable(foo = 1)]
trait Foo {
    fn foo(&self) -> i32;
}

#[test]
fn mock_returns_value() {
    let mock = MockFoo;
    assert_eq!(mock.foo(), 1);
}
