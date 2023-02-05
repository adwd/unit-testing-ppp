#[cfg(test)]
mod tests {
    use chrono::{DateTime, Duration, Utc};
    use std::ops::Add;

    struct Delivery {
        date_time: DateTime<Utc>,
    }

    impl Delivery {
        fn new(date_time: DateTime<Utc>) -> Self {
            Self { date_time }
        }
    }

    trait DeliveryService {
        fn is_delivery_valid(&self, now: &DateTime<Utc>, delivery: &Delivery) -> bool;
    }

    struct DeliveryServiceImpl {}

    impl DeliveryService for DeliveryServiceImpl {
        fn is_delivery_valid(&self, now: &DateTime<Utc>, delivery: &Delivery) -> bool {
            now.add(Duration::days(2)) <= delivery.date_time
        }
    }

    #[test]
    fn delivery_for_a_past_day_is_invalid() {
        let test_cases = vec![(-1, false), (0, false), (1, false), (2, true)];
        let now = Utc::now();

        for (days_from_now, expected) in test_cases.into_iter() {
            let sut = DeliveryServiceImpl {};
            let delivery_date = now.add(Duration::days(days_from_now));
            let delivery = Delivery::new(delivery_date);

            let is_valid = sut.is_delivery_valid(&now, &delivery);

            assert_eq!(
                expected, is_valid,
                "a day after {days_from_now} should be {expected}"
            );
        }
    }
}
