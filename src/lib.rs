#[macro_use]
extern crate diesel;

mod schema;
mod models;

use std::io;

use diesel::prelude::*;
use diesel::{Connection, PgConnection};
use models::{NewPost, Post};

pub struct DieselDemo {
    database_connection: PgConnection,
}

impl DieselDemo {
    pub fn new(database_url: String) -> DieselDemo {
        let database_connection = PgConnection::establish(&database_url).expect("Error connecting to the database");
        DieselDemo {
            database_connection,
        }
    }

    pub fn run(&mut self) {
        self.display_all_posts();
        self.add_new_post();
    }

    fn display_all_posts(&mut self) {
        use schema::posts::dsl::*;

        let all_posts = posts
            .load::<Post>(&mut self.database_connection)
            .expect("Error getting all posts");

        println!("Displaying all the posts...");

        for post in all_posts {
            println!("{}", post.title);
            println!("{}", post.body);
            println!("")
        }
    }

    fn add_new_post(&mut self) {
        use schema::posts;
        println!("Creating a new post...");
        
        let mut title = String::new();
        let mut body = String::new();

        println!("Title: ");
        io::stdin().read_line(&mut title).unwrap();

        println!("Body: ");
        io::stdin().read_line(&mut body).unwrap();

        let new_post = NewPost::new(title, body);

        diesel::insert_into(posts::table)
            .values(&new_post)
            .get_result::<Post>(&mut self.database_connection)
            .expect("Error adding a new post");
    }
}