//#[macro_use]
extern crate diesel;

mod schema;
mod models;

use std::io;

use diesel::prelude::*;
use diesel::{Connection, PgConnection};
use models::{NewPost, Post};
use schema::posts::is_published;

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
        // self.delete_post();
        // self.publish_post();
        self.add_new_post();
        // self.display_unpublished_posts();
    }

    fn display_all_posts(&mut self) {
        use schema::posts::dsl::*;

        let all_posts = posts
            .load::<Post>(&mut self.database_connection)
            .expect("Error getting all posts");

        println!("Displaying all the posts...");

        for post in all_posts {
            println!("{} | {} | {}", post.id, post.title.trim(), post.body.trim());
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

        let title = title.trim();
        let body = body.trim();

        let new_post = NewPost::new(title.to_string(), body.to_string());

        diesel::insert_into(posts::table)
            .values(&new_post)
            .get_result::<Post>(&mut self.database_connection)
            .expect("Error adding a new post");
    }

    fn publish_post(&mut self) {
        use schema::posts::dsl::*;

        let mut post_id = String::new();
        println!("Enter the post_id which you want to publish: ");
        io::stdin().read_line(&mut post_id).unwrap();

        let post_id = post_id.trim().parse::<i32>().unwrap();

        diesel::update(posts.find(post_id))
            .set(is_published.eq(true))
            .get_result::<Post>(&mut self.database_connection)
            .expect("Error publishing post");
    }

    fn delete_post(&mut self) {
        use schema::posts::dsl::*;

        let mut post_id = String::new();
        println!("Enter the post_id, you want to delete: ");
        io::stdin().read_line(&mut post_id).unwrap();

        let post_id = post_id.trim().parse::<i32>().unwrap();

        diesel::delete(posts.find(post_id))
            .execute(&mut self.database_connection)
            .expect("Error while deleting a post");
    }

    fn display_unpublished_posts(&mut self) {
        use schema::posts::dsl::*;

        let unpub_posts = posts
            .filter(is_published.eq(false))
            .load::<Post>(&mut self.database_connection)
            .expect("Error getting all unpublished posts");

        println!("Displaying UNPUBLISHED posts...");
        for post in unpub_posts {
            println!("{}: {}", post.id, post.title);
        }
    }
}