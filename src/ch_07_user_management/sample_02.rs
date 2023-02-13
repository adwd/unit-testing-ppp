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
    pub fn change_email(&mut self, new_email: &str, domain: &str, number_of_employees: i64) -> i64 {
        if self.email == new_email {
            return number_of_employees;
        }

        let email_domain = new_email.split('@').nth(0).unwrap();
        let is_email_corporate = email_domain == domain;
        let new_type = if is_email_corporate {
            UserType::Employee
        } else {
            UserType::Cusotmer
        };

        let mut new_number = number_of_employees;
        if self.user_type != new_type {
            let delta = if new_type == UserType::Employee {
                1
            } else {
                -1
            };
            new_number = number_of_employees + delta;
        }

        self.email = new_email.to_owned();
        self.user_type = new_type;

        new_number
    }
}

trait Database {
    fn get_user_by_id(&self, user_id: i64) -> User;
    fn get_company(&self) -> Company;
    fn save_company(&self, number_of_employee: i64);
    fn save_user(&self, user: &User);
}

trait MessageBus {
    fn send_email_changed_message(&self, user_id: i64, new_email: &str);
}

struct UserController<D: Database, M: MessageBus> {
    database: D,
    message_bus: M,
}

impl<D: Database, M: MessageBus> UserController<D, M> {
    pub fn change_email(&self, user_id: i64, new_email: &str) {
        let mut user = self.database.get_user_by_id(user_id);
        let company = self.database.get_company();

        let new_number_of_employees =
            user.change_email(new_email, &company.domain_name, company.number_of_employees);

        self.database.save_company(new_number_of_employees);
        self.database.save_user(&user);
        self.message_bus
            .send_email_changed_message(user_id, new_email);
    }
}
