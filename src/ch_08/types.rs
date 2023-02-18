use std::fmt::Debug;

#[derive(Debug)]
pub struct User {
    pub user_id: i64,
    pub email: String,
    pub email_confirmed: bool,
    pub email_changed_events: Vec<EmailChangeEvent>,
    pub user_type: UserType,
}

impl User {
    pub fn can_change_email(&self) -> bool {
        self.email_confirmed
    }

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

#[derive(PartialEq, Debug)]
pub struct EmailChangeEvent {
    pub user_id: i64,
    pub new_email: String,
}

#[derive(PartialEq, Debug, Copy, Clone, derive_more::Display)]
pub enum UserType {
    #[display(fmt = "CUSTOMER")]
    Cusotmer,
    #[display(fmt = "EMPLOYEE")]
    Employee,
}

impl From<String> for UserType {
    fn from(v: String) -> Self {
        match v.as_str() {
            "CUSTOMER" => Self::Cusotmer,
            _ => Self::Employee,
        }
    }
}

pub struct Company {
    pub id: i64,
    pub domain_name: String,
    pub number_of_employees: i64,
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

pub trait Database {
    type Error: std::error::Error + Send + Sync + 'static;
    fn get_user_by_id(&self, user_id: i64) -> Result<Option<User>, Self::Error>;
    fn get_company(&self) -> Result<Option<Company>, Self::Error>;
    fn save_company(&self, company: &Company);
    fn save_user(&self, user: &User) -> Result<(), Self::Error>;
}

#[mockall::automock]
pub trait MessageBus {
    fn send_email_changed_message(&self, user_id: i64, new_email: &str);
}

#[derive(derive_more::Constructor)]
pub struct UserController<D: Database, M: MessageBus> {
    pub database: D,
    pub message_bus: M,
}

impl<D: Database, M: MessageBus> UserController<D, M> {
    pub fn change_email(&self, user_id: i64, new_email: &str) -> anyhow::Result<()> {
        let mut user = match self.database.get_user_by_id(user_id) {
            Ok(result) => match result {
                Some(user) => user,
                None => return Err(anyhow::anyhow!("err")),
            },
            Err(e) => return Err(e.into()),
        };

        if !user.can_change_email() {
            return Err(anyhow::anyhow!("Cannot change email"));
        }

        let mut company = self.database.get_company()?.unwrap();

        user.change_email(new_email, &mut company);

        self.database.save_company(&company);
        self.database.save_user(&user)?;
        user.email_changed_events.iter().for_each(|ev| {
            self.message_bus
                .send_email_changed_message(ev.user_id, &ev.new_email);
        });

        Ok(())
    }
}
