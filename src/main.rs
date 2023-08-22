#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket_sync_db_pools; 

mod auth;
mod models;
mod schema;

   use diesel::prelude::*;
   use auth::BasicAuth;
   use models::Rustacean;
   use schema::rustaceans; 
   use rocket::serde::json::{Value, json};
   use::rocket::response::status;
   
   #[database("sqlite")]
      struct DbConn(diesel::SqliteConnection);
 
#[get("/rustaceans")]
async fn get_rustaceans(_auth: BasicAuth, db: DbConn) -> Value {
   db.run(|c| {
      let result = rustaceans::table.limit(100).load::<Rustacean>(c).expect("failed to read rusteacean entries");
      json!(result)
   }).await 
}

#[get("/rustaceans/<id>")]
fn view_rustacean(id: i32, _auth: BasicAuth) -> Value {
   json!({"id":id, "name":"John Doe", "email":"john@doe.com" })
}

#[post("/rustaceans", format="json")]
fn create_rustaceans(_auth: BasicAuth) -> Value {
  json!({"id":3, "name":"John Doe", "email":"john@doe.com" })
}

#[put("/rustaceans/<id>", format="json")]
fn update_rustacean(id: i32, _auth: BasicAuth) -> Value {
   json!({"id":id, "name":"John Doe", "email":"john@doe.com" })
}

#[delete("/rustaceans/<_id>")]
fn delete_rustacean(_id: i32, _auth: BasicAuth) -> status::NoContent {
   status::NoContent
}



#[catch(404)]
fn not_found() -> Value {
   json!("Not found!")
}


#[rocket::main]
async fn main () {
    let _= rocket::build()
    .mount("/", routes![
        get_rustaceans,
        view_rustacean,
        create_rustaceans,
        update_rustacean,
        delete_rustacean
        
        ])
        .register("/", catchers![not_found])
        .attach(DbConn::fairing())
        .launch()
        .await;
}





 