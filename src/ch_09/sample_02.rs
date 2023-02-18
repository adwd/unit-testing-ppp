use super::types::*;
use rusqlite::Connection;
struct SQLiteDatabase {
    pub conn: Connection,
}

impl Database for SQLiteDatabase {
    type Error = rusqlite::Error;
    fn get_user_by_id(&self, user_id: i64) -> Result<Option<User>, Self::Error> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, email, email_confirmed, user_type FROM user WHERE id = ?1")?;
        let user = stmt
            .query_map([user_id], |row| {
                let user_type: String = row.get(3)?;
                Ok(User {
                    user_id: row.get(0)?,
                    email: row.get(1)?,
                    email_confirmed: row.get(2)?,
                    domain_events: vec![],
                    user_type: UserType::from(user_type),
                })
            })?
            .next()
            .transpose();

        user
    }

    fn get_company(&self) -> Result<Option<Company>, Self::Error> {
        let mut stmt = self.conn.prepare("SELECT * from company limit 1")?;
        let company = stmt
            .query_map([], |row| {
                Ok(Company {
                    id: row.get(0)?,
                    domain_name: row.get(1)?,
                    number_of_employees: row.get(2)?,
                })
            })?
            .next()
            .transpose()?;

        Ok(company)
    }

    fn save_company(&self, company: &Company) {}

    fn save_user(&self, user: &User) -> Result<(), Self::Error> {
        self.conn.execute(
            "UPDATE user SET email = ?1, email_confirmed = ?2, user_type = ?3  WHERE id = ?4",
            (
                &user.email,
                user.email_confirmed,
                &user.user_type.to_string(),
                user.user_id,
            ),
        )?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use mockall::predicate::eq;
    use std::error;

    use crate::ch_09::test_helper::test_helper::{
        create_company, create_db, create_user, last_insert_rowid,
    };

    use super::*;

    fn get_db() -> SQLiteDatabase {
        let mut conn = create_db().unwrap();
        SQLiteDatabase { conn }
    }

    #[test]
    fn changing_email_from_corporate_to_non_corporate() -> Result<(), Box<dyn error::Error>> {
        // Arrange
        let mut db = get_db();
        let user = create_user(&mut db.conn, "user@mycorp.com", UserType::Employee)?;
        let company = create_company(&mut db.conn, "mycorp.com", 2)?;

        let mut bus_mock = MockBus::new();
        bus_mock
            .expect_send()
            .with(eq(
                "Type: USER EMAIL CHANGED; Id: 1; NewEmail: new@example.com",
            ))
            .times(1)
            .return_once(|_| {});
        let message_bus = MessageBus::new(bus_mock);

        let mut domain_logger_mock = MockDomainLogger::new();
        domain_logger_mock
            .expect_user_type_has_changed()
            .with(eq(1), eq(UserType::Employee), eq(UserType::Cusotmer))
            .times(1)
            .returning(|_, _, _| {});

        let sut = UserController::new(db, EventDispatcher::new(message_bus, domain_logger_mock));

        // Act
        let result = sut.change_email(user.user_id, "new@example.com");

        // Assert
        assert!(result.is_ok());

        let user_from_db = sut.database.get_user_by_id(user.user_id).unwrap().unwrap();
        assert_eq!("new@example.com", user_from_db.email);
        assert_eq!(UserType::Cusotmer, user_from_db.user_type);

        Ok(())
    }
}
