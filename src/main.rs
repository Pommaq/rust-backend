#[macro_use]
extern crate rocket;
extern crate serde;
use rocket::serde::json;
use rocket::response;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Flag {
    name: String
}

#[derive(response::Responder)]
enum FlagResponse {
    #[response(status = 404)]
    False(String),

    #[response(status = 200)]
    True(String),
}

#[post("/flag", format="json", data="<flag>")]
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
    serde_json::to_string(& vec!["apsoppa", "snopp", "chall2", "buffer overflow"]).unwrap()
}

#[launch]
fn rocket() -> _ {
    // Example of how to serialize a struct
    // println!("{}", serde_json::to_string(&Flag{name: String::from("aaaa")}).unwrap());
    rocket::build().mount("/", routes![verify_flag, get_challenges])
}
