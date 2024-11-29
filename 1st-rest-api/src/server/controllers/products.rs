// use rocket::serde::{Serialize, Deserialize};

// #[derive(Serialize, Deserialize)]
// struct Product {
//     id: String,
//     name: String,
// }

use crate::domain::use_cases::products::get_products::GetProductsUseCase;
use crate::repositories::implementations::in_memory::products_repository::InMemoryProductsRepository;
use crate::domain::entities::product::Product;
use rocket::serde::json::Json;
use rocket::http::Status;
use rocket::get;

#[get("/products")]
pub fn get_products() -> Result<Json<Vec<Product>>, Status> {
    let get_products_use_case = GetProductsUseCase::new(InMemoryProductsRepository::new());
    let products = get_products_use_case.execute();
    Ok(Json(products))
}

#[post("/products")]
pub fn create_product() -> &'static str {
    todo!()
}

#[get("/products/<id>")]
pub fn get_product(id: String) -> String {
    todo!()
}

#[put("/products/<id>")]
pub fn update_product(id: String) -> String {
  todo!()
}

#[delete("/products/<id>")]
pub fn delete_product(id: String) -> String {
  todo!()
}