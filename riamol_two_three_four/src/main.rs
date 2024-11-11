use rand::Rng;
use std::fmt;

#[derive(Debug)]
struct warehouse {
    id : u64,
    items_in_stock : u32,
    location : String,
    products : Option<Vec<Products>>,
    repair_products : Option<Vec<Products>>,
    digital_products : Option<Vec<Products>>,
    house_products : Option<Vec<Products>>,
}

impl fmt::Display for warehouse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl warehouse {
  
    fn create_warehouse() -> Self {
        let random_15 : u64;
        fn generate_random_15_digit_number() -> u64 {
            let mut rng = rand::thread_rng();
            rng.gen_range(1_000_000_000_000_000..10_000_000_000_000_000) // Range to ensure 15 digits
        }

        let random_1 : usize;
        let countries  = [String::from("France"), String::from("China"), String::from("India"), String::from("Brazil"), String::from("Costa Rica"), String::from("Sweden"), String::from("Australia"), String::from("Bostwana"), String::from("Guinea")];
        fn generate_random_1_digit_number() -> usize {
            let mut rng = rand::thread_rng();
            rng.gen_range(0..9)
        }
        
        random_15 = generate_random_15_digit_number();
        random_1 = generate_random_1_digit_number();

        Self {
            id : random_15,
            items_in_stock : 0,
            location : countries[random_1].clone(),
            products : None,
            repair_products : None,
            digital_products : None,
            house_products : None,
        }
    }
}

#[derive(Debug)]
enum Categories {
    Repair, 
    Digital, 
    House,
}
#[derive(Debug)]
struct Products {
    product_id : u32,
    name : String,
    price : f32, 
    category : Categories,
    warehouse_id : u32,
    stocks : u32,
    discount : u32,
    country_made_in : String,
}
// function for generating a random number for product ID
// function to put items in a vector (the one of the warehouse)
// function to apply discount based on category
// function to put the item in the warehouse that is located the closest from its country where it has been made


fn main () {
    let mut warehouse_one = warehouse::create_warehouse();
    println!("{}", warehouse_one);
}