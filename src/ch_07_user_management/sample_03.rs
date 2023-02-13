struct User {
    user_id: i64,
    email: String,
    user_type: UserType,
}

#[derive(PartialEq, Debug)]
enum UserType {
    Cusotmer,
    Employee,
}

struct Company {
    domain_name: String,
    number_of_employees: i64,
}

impl Company {
    fn change_number_of_employees(&mut self, delta: i64) {
        assert!(
            self.number_of_employees + delta >= 0,
            "Number of employees must be greater than zero"
        );

        self.number_of_employees += delta;
    }

    fn is_email_corporate(&self, email: &str) -> bool {
        let email_domain = email
            .split('@')
            .nth(1)
            .expect(&format!("email not contains '@', value: {}", email));

        email_domain == self.domain_name
    }
}

impl User {
    pub fn change_email(&mut self, new_email: &str, company: &mut Company) {
        if self.email == new_email {
            return;
        }

        let new_type = if company.is_email_corporate(new_email) {
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
            company.change_number_of_employees(delta);
        }

        self.email = new_email.to_owned();
        self.user_type = new_type;
    }
}

trait Database {
    fn get_user_by_id(&self, user_id: i64) -> User;
    fn get_company(&self) -> Company;
    fn save_company(&self, company: &Company);
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
        let mut company = self.database.get_company();

        user.change_email(new_email, &mut company);

        self.database.save_company(&company);
        self.database.save_user(&user);
        self.message_bus
            .send_email_changed_message(user_id, new_email);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn changing_email_from_non_corporate_to_corporate() {
        let mut company = Company {
            domain_name: "mycorp.com".to_owned(),
            number_of_employees: 1,
        };
        let mut sut = User {
            email: "user@example.com".to_owned(),
            user_id: 1,
            user_type: UserType::Cusotmer,
        };

        sut.change_email("new@mycorp.com", &mut company);

        assert_eq!(2, company.number_of_employees);
        assert_eq!("new@mycorp.com", sut.email);
        assert_eq!(UserType::Employee, sut.user_type);
    }

    #[test]
    fn changing_email_from_corporate_to_non_corporate() {}

    #[test]
    fn changing_email_without_changing_user_type() {}

    #[test]
    fn changing_email_to_same_one() {}
}
