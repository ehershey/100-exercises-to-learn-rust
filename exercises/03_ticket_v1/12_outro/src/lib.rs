// TODO: Define a new `Order` type.
pub struct Order {
    product_name: String,
    quantity: u16,
    unit_price: u16,
}

impl Order {
    pub fn new(product_name: String, quantity: u16, unit_price: u16) -> Order {
        println!("in constructor");
        if quantity == 0 {
            panic!("Quantity cannot be zero");
        }
        if unit_price == 0 {
            panic!("Unit price cannot be zero");
        }
        if product_name == "" {
            panic!("Product name cannot be empty");
        }
        if product_name.len() > 300 {
            panic!("Product name cannot be longer than 300 characters");
        }
        Order {
            product_name: product_name,
            quantity: quantity,
            unit_price: unit_price,
        }
    }
    pub fn total(&self) -> u16 {
        self.quantity * self.unit_price
    }
    pub fn unit_price(&self) -> &u16 {
        &self.unit_price
    }
    pub fn quantity(&self) -> &u16 {
        &self.quantity
    }
    pub fn product_name(&self) -> &String {
        &self.product_name
    }
    pub fn set_unit_price(&mut self, unit_price: u16) {
        if unit_price == 0 {
            panic!("Unit price cannot be zero");
        }
        self.unit_price = unit_price
    }
    pub fn set_quantity(&mut self, quantity: u16) {
        if quantity == 0 {
            panic!("Quantity cannot be zero");
        }
        self.quantity = quantity
    }
    pub fn set_product_name(&mut self, product_name: String) {
        if product_name == "" {
            panic!("Product name cannot be empty");
        }
        if product_name.len() > 300 {
            panic!("Product name cannot be longer than 300 characters");
        }
        self.product_name = product_name;
    }
}
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.
