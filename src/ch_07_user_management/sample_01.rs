struct User {
    user_id: i64,
    email: String,
    user_type: UserType,
}

#[derive(PartialEq)]
enum UserType {
    Cusotmer,
    Employee,
}

struct Company {
    domain_name: String,
    number_of_employees: i64,
}

impl User {
    pub fn change_email(&mut self, user_id: i64, new_email: &str) {
        let user = Database::get_user_by_id(user_id);
        self.user_id = user_id;
        self.email = user.email;
        self.user_type = user.user_type;

        if self.email == new_email {
            return;
        }

        let company = Database::get_company();

        let email_domain = new_email.split('@').nth(0).unwrap();
        let is_email_corporate = email_domain == company.domain_name;
        let new_type = if is_email_corporate {
            UserType::Employee
        } else {
            UserType::Cusotmer
        };

        if self.user_type != new_type {
            let delta = if new_type == UserType::Employee {
                1
            } else {
                -1
            };
            let new_number = company.number_of_employees + delta;
            Database::save_company(new_number);
        }

        self.email = new_email.to_owned();
        self.user_type = new_type;

        Database::save_user(&self);
        MessageBus::send_email_changed_message(self.user_id, &self.email);
    }
}

struct Database;

impl Database {
    pub fn get_user_by_id(user_id: i64) -> User {
        User {
            user_id,
            email: "todo".to_owned(),
            user_type: UserType::Cusotmer,
        }
    }

    pub fn get_company() -> Company {
        Company {
            domain_name: "todo".to_owned(),
            number_of_employees: 1000,
        }
    }

    pub fn save_company(number_of_employee: i64) {}

    pub fn save_user(user: &User) {}
}

struct MessageBus;

impl MessageBus {
    pub fn send_email_changed_message(user_id: i64, new_email: &str) {}
}
