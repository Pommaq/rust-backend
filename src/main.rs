pub mod models;
pub mod schema;
use self::schema::flags::dsl::*;
use crate::schema::challenges::dsl::*;

#[macro_use]
extern crate rocket;
extern crate diesel;
extern crate serde;

use rocket::response;
use rocket::serde::json;
use diesel::prelude::*;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Flag {
    name: String,
}

#[derive(response::Responder)]
enum FlagResponse {
    #[response(status = 404)]
    False(String),

    #[response(status = 200)]
    True(String),
}

#[post("/flag", format = "json", data = "<flag>")]
fn verify_flag(flag: json::Json<Flag>) -> FlagResponse {
    if flag.name.contains("false") {
        FlagResponse::False(String::from("The flag was incorrect... 😞"))
    } else {
        FlagResponse::True(String::from("The flag exists! 😄"))
    }
}

/// ## Cool
/// This is a string that should be markdown?
/// ```python
/// if True:
///     pass
/// ```
/// like that
#[get("/challenge")]
fn get_challenges() -> String {
    serde_json::to_string(&vec!["apsoppa", "snopp", "chall2", "buffer overflow"]).unwrap()
}


fn establish_connection(database_url: &str) -> ConnectionType {
    ConnectionType::establish(database_url)
        .unwrap_or_else(|err| panic!("Error! Failed connecting to {}: {}", database_url, err))
}

type ConnectionType = SqliteConnection;

fn prepare_database(db_con: &mut ConnectionType) {
    diesel::insert_into(challenges).values(models::NewChallenge{name: "apsoppa"}).execute(db_con).expect("Error saving post");
}

#[launch]
fn rocket() -> _ {
    // Example of how to serialize a struct
    // println!("{}", serde_json::to_string(&Flag{name: String::from("aaaa")}).unwrap());
    let db_con = &mut establish_connection("./database.db");
    prepare_database(db_con);
    rocket::build().mount("/", routes![verify_flag, get_challenges])
}
