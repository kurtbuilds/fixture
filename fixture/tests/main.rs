use fixture_derive::Fixture;
use pretty_assertions::assert_eq;

#[test]
fn test_main() {
    #[derive(Debug, Fixture)]
    pub struct Foo {
        a: Option<String>,
        b: usize,
    }

    let s = Foo {
        a: Some("hello".to_string()),
        b: 100,
    };
    let t = FooFixture {
        a: "hello",
        b: 100,
    };
    assert_eq!(s, t);
}