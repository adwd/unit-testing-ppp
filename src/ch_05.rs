use mockall::automock;

struct Store {}

struct Customer {
    mail: String,
}

impl Customer {
    fn purchase(&self, store: Store, product: Product, quantity: u32) -> bool {
        false
    }
}

#[automock]
trait CustomerRepository {
    fn get_by_id(&self, customer_id: i64) -> Customer;
}

#[derive(Clone)]
struct Product {
    name: String,
}

#[automock]
trait ProductRepository {
    fn get_by_id(&self, product_id: i64) -> Product;
}

#[automock]
trait EmailGateway {
    fn send_receipt(&self, address: &str, name: &str, quantity: u32);
}

struct CustomerController<C: CustomerRepository, P: ProductRepository, E: EmailGateway> {
    customer_repository: C,
    product_repository: P,
    email_gateway: E,
}

impl<C: CustomerRepository, P: ProductRepository, E: EmailGateway> CustomerController<C, P, E> {
    pub fn purchase(&self, customer_id: i64, product_id: i64, quantity: u32) -> bool {
        let customer = self.customer_repository.get_by_id(customer_id);
        let product = self.product_repository.get_by_id(product_id);

        let is_success = customer.purchase(Store {}, product.clone(), quantity);

        if is_success {
            self.email_gateway
                .send_receipt(&customer.mail, &product.name, quantity);
        }

        is_success
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CustomerController, MockCustomerRepository, MockEmailGateway, MockProductRepository,
    };

    // 動作しない。この本の意図としてはrepositoryはプロセス外依存ではあるものの共有依存でないので実装詳細としている。
    // なのでDBにつなぐなどしてRepositoryが動作するようにする必要がある。
    #[test]
    fn successful_purchase() {
        let mut mock = MockEmailGateway::new();
        mock.expect_send_receipt()
            .withf(|address: &str, name: &str, quantity: &u32| {
                address == "customer@email.com" && name == "Shampoo"
            })
            .times(1);
        let customer_repository = MockCustomerRepository::new();
        let product_repository = MockProductRepository::new();
        let sut = CustomerController {
            email_gateway: mock,
            customer_repository,
            product_repository,
        };

        let is_success = sut.purchase(1, 2, 5);

        assert!(is_success);
    }
}
