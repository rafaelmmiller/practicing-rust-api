use crate::domain::entities::product::Product;

pub trait ProductsRepository {
  fn get_products(&self) -> Vec<Product>;
  fn create_product(&self, product: Product) -> Product;
  fn find_by_id(&self, id: String) -> Product;
  fn update(&self, product: Product) -> Product;
  fn delete(&self, id: String) -> bool;
}