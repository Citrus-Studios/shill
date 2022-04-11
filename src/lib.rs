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
