use rand::Rng;
use std::fmt;
use std::io;

#[derive(Debug)]
struct Warehouse {
    id : u64,
    items_in_stock : u32,
    location : String,
    products : Option<Vec<Product>>,
    repair_products : Option<Vec<Product>>,
    digital_products : Option<Vec<Product>>,
    house_products : Option<Vec<Product>>,
}

impl fmt::Display for Warehouse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Warehouse {
  
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
struct Product {
    product_id : u32,
    name : String,
    price : Option<u32>, 
    category : Option<Categories>,
    warehouse_id : Option<u32>,
    stocks : Option<u32>,
    discount : Option<u32>,
    country_made_in : Option<String>,
}

impl fmt::Display for Product {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// function to put items in a vector (the one of the warehouse)
// function to apply discount based on category
// function to put the item in the warehouse that is located the closest from its country where it has been made

impl Product {
    fn new_product() -> Self {
        
        println!("Please input a product name.");
        let mut name = String::new();
        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read line");

        fn generate_random_6_digit_number() -> u32 {
            let mut rng = rand::thread_rng();
            rng.gen_range(100_000..1_000_000)
        }

            let random_6 = generate_random_6_digit_number();

            Self {
                product_id : random_6,
                name : name,
                price : None,
                category : None, 
                warehouse_id : None, 
                stocks : None, 
                discount : None, 
                country_made_in : None, 
            }
        }
    }

fn main () {
    let mut warehouse_one = Warehouse::create_warehouse();
    println!("{}", warehouse_one);
    let mut product_one = Product::new_product();
    println!("{}", product_one);
}