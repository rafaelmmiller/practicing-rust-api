// use rocket::serde::{Serialize, Deserialize};

// #[derive(Serialize, Deserialize)]
// struct Product {
//     id: String,
//     name: String,
// }

#[get("/users")]
pub fn get_users() -> &'static str {
    todo!()
}

#[post("/users")]
pub fn create_user() -> &'static str {
    todo!()
}

#[get("/users/<id>")]
pub fn get_user(id: String) -> String {
    todo!()
}

#[put("/users/<id>")]
pub fn update_user(id: String) -> String {
  todo!()
}

#[delete("/users/<id>")]
pub fn delete_user(id: String) -> String {
  todo!()
}