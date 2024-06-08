extern crate dotenv;
extern crate diesel_with_postgres;

use std::env;
use diesel_with_postgres::DieselDemo;
use dotenv::dotenv;

fn main() {
    dotenv().ok();
    
    let database_url = env::var("DATABASE_URL").expect("Failed loading DB_URL");
    let mut diesel_demo = DieselDemo::new(database_url);
    diesel_demo.run();
}
