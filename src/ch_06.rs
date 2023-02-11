#[cfg(test)]
mod output_based_tests {
    use rust_decimal::Decimal;
    use rust_decimal_macros::dec;

    struct Product {
        name: String,
    }

    struct PriceEngine {}

    impl PriceEngine {
        fn calculate_discount(&self, products: &[Product]) -> Decimal {
            let len = Decimal::new(products.len() as i64, 0);
            let discount: Decimal = len * dec!(0.01);

            discount.min(dec!(0.2))
        }
    }

    #[test]
    fn discount_of_two_products() {
        let p1 = Product {
            name: "Hand wash".to_owned(),
        };
        let p2 = Product {
            name: "Shampoo".to_owned(),
        };
        let sut = PriceEngine {};

        let discount = sut.calculate_discount(&vec![p1, p2]);

        assert_eq!(dec!(0.02), discount);
    }
}

#[cfg(test)]
mod state_based_tests {
    #[derive(Clone, PartialEq, Eq, Debug)]
    struct Product {
        name: String,
    }

    struct Order {
        products: Vec<Product>,
    }

    impl Order {
        pub fn add_product(&mut self, product: Product) {
            self.products.push(product);
        }
    }

    #[test]
    fn adding_a_product_to_an_order() {
        let product = Product {
            name: "Hand wash".to_owned(),
        };
        let mut sut = Order { products: vec![] };

        sut.add_product(product.clone());

        assert_eq!(1, sut.products.len());
        assert_eq!(product, sut.products[0]);
    }
}

#[cfg(test)]
mod communication_based_tests {
    use mockall::automock;

    #[automock]
    trait EmailGateway {
        fn send_greetings_email(&self, address: &str);
    }

    struct Controller<E: EmailGateway> {
        email_gateway: E,
    }

    impl<E: EmailGateway> Controller<E> {
        fn greet_user(&self, address: &str) {
            self.email_gateway.send_greetings_email(address);
        }
    }

    #[test]
    fn sending_a_greetings_email() {
        let mut email_gateway_mock = MockEmailGateway::new();
        email_gateway_mock
            .expect_send_greetings_email()
            .withf(|address: &str| address == "user@email.com")
            .times(1)
            .return_once(|_| {});
        let sut = Controller {
            email_gateway: email_gateway_mock,
        };

        sut.greet_user("user@email.com");
    }
}
