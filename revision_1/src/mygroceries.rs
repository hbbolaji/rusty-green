use std::ops::Add;

struct GroceryItem {
  name: String,
  price: f32
}

struct GroceryBill {
  items: Vec<GroceryItem>,
  tax_rate: f32
}

impl Add<GroceryItem> for GroceryBill {
  type Output = GroceryBill;
  fn add(self, rhs: GroceryItem) -> Self::Output {
      let mut bill = self;
      bill.items.push(rhs);
      return bill;
  }
}

impl GroceryBill {
  fn calculate_total(&self) -> f32 {
    self.items
      .iter()
      .fold(0f32, |acc, item| acc + item.price) * (1f32 + self.tax_rate)
  }
}


pub fn test_groceries() {
  let mut new_bill = GroceryBill {items: Vec::<GroceryItem>::new(), tax_rate: 0.05};
  
  let carrots = GroceryItem { name: "Carrot".into(), price: 4.99};
  let cheese: GroceryItem = GroceryItem { name: "Cottage Cheese".to_string(), price: 9.99 };
  let bill = new_bill + carrots + cheese;
  println!("The total of your grocery bill is: {}", (bill.calculate_total() * 100.0).round() / 100.0);
}