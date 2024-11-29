use crate::repositories::products_repository::ProductsRepository;
use crate::domain::entities::product::Product;

pub struct GetProductsUseCase<R: ProductsRepository> {
    repository: R,
}

impl<R: ProductsRepository> GetProductsUseCase<R> {
    pub fn new(repository: R) -> Self {
        GetProductsUseCase { repository }
    }

    pub fn execute(&self) -> Vec<Product> {
        self.repository.get_products()
    }
}
