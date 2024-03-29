use diesel::result::QueryResult;
use diesel::prelude::*;
use super::models::*;
use super::schema::*;

pub struct RustaceanRepository;

impl RustaceanRepository {
    pub fn load_all( c: &mut SqliteConnection, limit: i64) -> QueryResult<Vec<Rustacean>> {
        rustaceans::table.limit(limit).load::<Rustacean>(c)
    }

    pub fn find_one(c: &mut SqliteConnection, id: i32) -> QueryResult<Rustacean> {
        rustaceans::table.find(id).get_result::<Rustacean>(c)
    }

    pub fn create(c: &mut SqliteConnection, new_rustacean: NewRustacean) -> QueryResult<Rustacean> {
        diesel::insert_into(rustaceans::table)
            .values(new_rustacean)
            .execute(c)?;

        let last_id = Self::last_id(c)?;

        Self::find_one(c, last_id)
    }

    pub fn save(c: &mut SqliteConnection, rustacean: Rustacean) -> QueryResult<Rustacean> {
        diesel::update(rustaceans::table.find(rustacean.id))
            .set((
                rustaceans::name.eq(rustacean.name.to_owned()),
                rustaceans::email.eq(rustacean.email.to_owned()),
            ))
            .execute(c)?;

        Self::find_one(c, rustacean.id)
    }

    pub fn delete(c: &mut SqliteConnection, id: i32) -> QueryResult<usize>  {
        diesel::delete(rustaceans::table.find(id)).execute(c)
    }

    fn last_id(c: &mut SqliteConnection) -> QueryResult<i32> {
        rustaceans::table
            .select(rustaceans::id)
            .order(rustaceans::id.desc())
            .first(c)
    }
}