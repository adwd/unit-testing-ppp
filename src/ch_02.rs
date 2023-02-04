#[cfg(test)]
mod clasic_school_tests {
    use std::collections::HashMap;

    #[derive(PartialEq, Eq, Hash)]
    enum Product {
        Shampoo,
    }

    type Quantity = usize;

    struct Store {
        inventory: HashMap<Product, Quantity>,
    }

    impl Store {
        pub fn new() -> Self {
            Self {
                inventory: HashMap::new(),
            }
        }

        fn add_inventry(&mut self, product: Product, quantity: Quantity) {
            self.inventory.insert(product, quantity);
        }

        fn get(&self, product: Product) -> Quantity {
            *self.inventory.get(&product).unwrap_or(&0)
        }
    }

    struct Customer {}

    impl Customer {
        pub fn new() -> Self {
            Self {}
        }

        fn purchase(self, store: &mut Store, product: Product, quantity: Quantity) -> bool {
            for (p, q) in store.inventory.iter_mut() {
                if *p == product {
                    if *q >= quantity {
                        *q -= quantity;
                        return true;
                    } else {
                        return false;
                    }
                }
            }
            false
        }
    }

    #[test]
    fn purchase_succeeds_when_enough_inventry() {
        // Arrange
        let mut store = Store::new();
        store.add_inventry(Product::Shampoo, 10);
        let customer = Customer::new();

        // Act
        let success = customer.purchase(&mut store, Product::Shampoo, 5);

        // Assert
        assert!(success);
        assert_eq!(5, store.get(Product::Shampoo));
    }

    #[test]
    fn purchase_fails_when_not_enough_inventry() {
        // Arrange
        let mut store = Store::new();
        store.add_inventry(Product::Shampoo, 10);
        let customer = Customer::new();

        // Act
        let success = customer.purchase(&mut store, Product::Shampoo, 15);

        // Assert
        assert!(!success);
        assert_eq!(10, store.get(Product::Shampoo));
    }
}

#[cfg(test)]
mod london_school_tests {
    use mockall::{automock, predicate::eq};
    #[derive(PartialEq, Eq, Debug, Clone, Copy)]
    enum Product {
        Shampoo,
    }

    type Quantity = usize;

    #[automock]
    trait Store {
        fn has_enough_inventory(&self, product: Product, quantity: Quantity) -> bool;
        fn remove_inventory(&mut self, product: Product, quantity: Quantity);
    }

    struct Customer {}

    impl Customer {
        pub fn new() -> Self {
            Self {}
        }

        fn purchase(self, store: &mut impl Store, product: Product, quantity: Quantity) -> bool {
            if store.has_enough_inventory(product, quantity) {
                store.remove_inventory(product, quantity);
                return true;
            }
            false
        }
    }

    #[test]
    fn purchase_succeeds_when_enough_inventry() {
        // Arrange
        let mut store = MockStore::new();
        store
            .expect_has_enough_inventory()
            .with(eq(Product::Shampoo), eq(5))
            .times(1)
            .return_once(|_, _| true);
        store
            .expect_remove_inventory()
            .with(eq(Product::Shampoo), eq(5))
            .times(1)
            .returning(|_, _| {});
        let customer = Customer::new();

        // Act
        let success = customer.purchase(&mut store, Product::Shampoo, 5);

        // Assert
        assert!(success);
    }

    #[test]
    fn purchase_fails_when_not_enough_inventry() {
        // Arrange
        let mut store = MockStore::new();
        store
            .expect_has_enough_inventory()
            .with(eq(Product::Shampoo), eq(5))
            .times(1)
            .return_once(|_, _| false);
        store.expect_remove_inventory().times(0);
        let customer = Customer::new();

        // Act
        let success = customer.purchase(&mut store, Product::Shampoo, 5);

        // Assert
        assert!(!success);
    }
}
