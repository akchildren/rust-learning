use std::collections::HashMap;

struct Product {
    name : String,
    price : f32,
    stock : u16
}

impl Product {
    fn get_name(self) -> String {
        self.name
    }
    fn get_price(&self) -> f32 {
        self.price
    }
    fn in_stock(&self) -> bool {
        self.stock > 0
    }
}

struct Cart {
    products : HashMap<Product, i32>
}

impl Cart {
    fn add_product() {

    }

    fn remove_product(&self, product: Product) {
        
    }

    fn get_total(&self) {
        
    }

    fn clear(&self) {
        
    }
}

fn main() {
    println!("Hello, world!");
}

fn add_product_to_cart() {

}

fn remove_product_from_cart() {

}