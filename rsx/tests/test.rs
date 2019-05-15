extern crate rsx;

use rsx::component;

#[derive(Debug, PartialEq)]
struct TestComponent;

impl TestComponent {
    fn build() -> TestComponentBuilder {
        TestComponentBuilder::default()
    }
}

#[derive(Default, Debug)]
struct TestComponentBuilder;

impl TestComponentBuilder {
    fn create(&mut self) -> TestComponent {
        TestComponent
    }
}

#[test]
fn component_should_create_a_simple_instance() {
    let expected = TestComponent::build().create();

    let result = component!(TestComponent);


    assert_eq!(result, expected);
}