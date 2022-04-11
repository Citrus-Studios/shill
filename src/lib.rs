/*!
# Shill
Shill is a tool for shilling easily

# Example
```rs
struct Person {}
struct Linux {}

impl Shill<Linux> for Person {}

let new_person = Person {};
// Person loves Linux, you should use Linux, Linux is the best!
new_person.shill();
```

(if you wish to actually use this for something please contact me on github)
*/


#[test]
fn test_shill_trait() {
    struct Test {}
    struct Beans {}
    impl Shill<Beans> for Test {}

    let test = Test {};
    test.shill();
}

/// Shills a product through implementation of a trait
pub trait Shill<T: Sized> {
    fn shill(&self)
    where
        Self: Sized,
    {
        let product = std::any::type_name::<T>()
            .split("::")
            .collect::<Vec<&str>>();
        let name = std::any::type_name::<Self>()
            .split("::")
            .collect::<Vec<&str>>();
        println!(
            "{name} loves {product}, you should use {product}, {product} is the best!",
            name = name[name.len() - 1],
            product = product[product.len() - 1]
        );
    }
}
