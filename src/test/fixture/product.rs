#[cfg(test)]
use crate::model::Product;

#[cfg(test)]
pub fn product_fixture(id: usize) -> Product {
    Product {
        id: id as i32,
        name: String::from("apple"),
    }
}

#[cfg(test)]
pub fn products_fixture(num: usize) -> Vec<Product> {
    let mut products = vec![];
    for i in 1..num + 1 {
        products.push(product_fixture(i));
    }
    products
}
