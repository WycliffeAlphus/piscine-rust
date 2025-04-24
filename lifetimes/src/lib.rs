#[derive(Debug, Clone, PartialEq)]
pub struct Person<'a>{
	pub name: &'a str,
	pub age: u8,
}

#[allow(elided_named_lifetimes)]
impl Person<'_> {
	pub fn new<'a>(name: &'a str) -> Person{
        Person {
            name: name,
            age: 0,
        }
	}
}
