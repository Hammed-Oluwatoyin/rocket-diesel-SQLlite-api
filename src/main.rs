#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket_sync_db_pools;


 
mod auth;
mod models;
mod schema;
mod repositories;

   
   use auth::BasicAuth;
   use models::{Rustacean, NewRustacean};
   use repositories::RustaceanRepository;
   use rocket::serde::json::{Value, json, Json};
   use rocket::http::Status;
   use rocket::response::status::{self, Custom};
 
   

   
   #[database("sqlite")]
      struct DbConn(diesel::SqliteConnection);

      

 
#[get("/rustaceans")]
async fn get_rustaceans(_auth: BasicAuth, db: DbConn) -> Result<Value, Custom<Value>> {
   db.run(|c| {
          RustaceanRepository::load_all(c, 100).map(|rustaceans| json!(rustaceans))
            .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))   
   }).await

}

#[get("/rustaceans/<id>")]
async fn view_rustacean(id: i32, _auth: BasicAuth, db: DbConn) ->  Result<Value, Custom<Value>>  {
     db.run(move |c| {
       RustaceanRepository::find_one(c, id).map(|rustaceans| json!(rustaceans))
      .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
   }).await 
   
   
}

#[post("/rustaceans", format="json", data="<new_rustacean>")]
async fn create_rustaceans(_auth: BasicAuth, db: DbConn, new_rustacean: Json<NewRustacean>) -> Result<Value, Custom<Value>>  {
 db.run(|c| {
        RustaceanRepository::create(c, new_rustacean.into_inner())
            .map(|rustaceans| json!(rustaceans))
            .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
    }).await
}
 
#[put("/rustaceans/<_id>", format="json", data = "<rustacean>")] 
async fn update_rustacean(_id: i32, _auth: BasicAuth , db:DbConn, rustacean: Json<Rustacean>) -> Result<Value, Custom<Value>> {
 db.run(move |c| {
        RustaceanRepository::save(c, rustacean.into_inner())
            .map(|rustaceans| json!(rustaceans))
            .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
    }).await
}

#[delete("/rustaceans/<id>")]
async fn delete_rustacean(id: i32, _auth: BasicAuth, db:DbConn) -> Result<status::NoContent, status::Custom<Value>>   {
   db.run(move |c| {
        RustaceanRepository::delete(c, id)
            .map(|_| status::NoContent)
            .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
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





 