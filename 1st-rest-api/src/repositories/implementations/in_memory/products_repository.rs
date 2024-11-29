use crate::repositories::products_repository::ProductsRepository;
use crate::domain::entities::product::Product;

pub struct InMemoryProductsRepository {
  products: Vec<Product>
}

impl InMemoryProductsRepository {
    pub fn new() -> Self {
      InMemoryProductsRepository {
          products: vec![
            Product {
              id: String::from("1"),
              name: String::from("Product 1"),
              description: String::from("Description 1"),
              price: 10.0
            }, Product {
              id: String::from("2"),
              name: String::from("Product 2"),
              description: String::from("Description 2"),
              price: 20.0
            }, Product {
              id: String::from("3"),
              name: String::from("Product 3"),
              description: String::from("Description 3"),
              price: 30.0
          }]
      }
    }
}

impl ProductsRepository for InMemoryProductsRepository {
    fn get_products(&self) -> Vec<Product> {
       self.products.clone()
    }

    fn create_product(&self, product: Product) -> Product {
        todo!()
        // self.products.push(product);
        // product
    }

    fn find_by_id(&self, id: String) -> Product {
        todo!()
        // self.products.find(|product| product.id == id).unwrap()
    }

    fn update(&self, product: Product) -> Product {
        todo!()
        // self.products.find(|p| p.id == product.id).unwrap() = product;
    }

    fn delete(&self, id: String) -> bool {
        todo!()
        // self.products.remove(
    }
}
