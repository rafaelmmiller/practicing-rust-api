#[macro_use] extern crate rocket;

pub mod server;
pub mod domain;
pub mod repositories;

use crate::server::controllers::products::{
  get_products,
  create_product,
  get_product,
  update_product,
  delete_product
};

use crate::server::controllers::users::{
  get_users,
  create_user,
  get_user,
  update_user,
  delete_user
};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![
      index,
      get_products,
      create_product,
      get_product,
      update_product,
      delete_product,
      get_users,
      create_user,
      get_user,
      update_user,
      delete_user
    ])
}
