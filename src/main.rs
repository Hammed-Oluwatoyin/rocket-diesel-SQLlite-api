#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket_sync_db_pools; 

mod auth;
mod models;
mod schema;

   use diesel::prelude::*;
   use auth::BasicAuth;
   use models::{Rustacean, NewRustacean};
   use schema::rustaceans; 
   use rocket::serde::json::{Value, json, Json};
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
async fn view_rustacean(id: i32, _auth: BasicAuth, db: DbConn) -> Value {
     db.run(move |c| {
      let result = rustaceans::table.find(id).get_result::<Rustacean>(c).expect("failed retrieving rustacean row");
      json!(result)
     }).await
   
   
}

#[post("/rustaceans", format="json", data="<new_rustacean>")]
async fn create_rustaceans(_auth: BasicAuth , db: DbConn, new_rustacean: Json<NewRustacean>) -> Value {
db.run(|c| {
      let result = diesel::insert_into(rustaceans::table).values(new_rustacean.into_inner()).execute(c).expect("failed inserting new rusteacean entry");
      json!(result);
   }).await.into()
}
 
#[put("/rustaceans/<id>", format="json", data = "<rustacean>")] 
async fn update_rustacean(id: i32, _auth: BasicAuth , db:DbConn, rustacean: Json<Rustacean>) -> Value {
   db.run(move |c| {
      let result = diesel::update(rustaceans::table.find(id))
                  .set((
                        rustaceans::email.eq(rustacean.email.to_owned()),
                        rustaceans::name.eq(rustacean.name.to_owned())
                  )).execute(c).expect("failed updating rustacean entry");
                  json!(result)
   }).await
}

#[delete("/rustaceans/<id>")]
async fn delete_rustacean(id: i32, _auth: BasicAuth, db:DbConn) -> status::NoContent {
   db.run(move |c| {
      diesel::delete(rustaceans::table.find(id)).execute(c).expect("failed deleitng rustacean entry");
        status::NoContent
   }).await
   
 
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





 