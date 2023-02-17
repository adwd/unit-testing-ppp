use anyhow::anyhow;
use rusqlite::Connection;

use super::types::*;

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
                    email_changed_events: vec![],
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
    use anyhow::anyhow;
    use mockall::predicate::eq;
    use std::error;

    use super::*;
    use crate::ch_08::{
        test_helper::test_helper::{create_company, create_db, create_user, last_insert_rowid},
        types::UserType,
    };

    fn get_db() -> SQLiteDatabase {
        let mut conn = create_db().unwrap();
        let user = create_user(&mut conn, "example.com", UserType::Cusotmer).unwrap();
        assert_eq!(user.user_id, 1);

        let company = create_company(&mut conn, "my_corp", 0).unwrap();
        assert_eq!(company.id, 1);

        SQLiteDatabase { conn }
    }

    #[test]
    fn get_user_by_id() -> Result<(), Box<dyn error::Error>> {
        let mut db = get_db();
        let user = db.get_user_by_id(1)?;
        assert_eq!(user.map(|u| u.email), Some("example.com".into()));

        let last_id = last_insert_rowid(&db.conn);
        assert_eq!(last_id, 1);

        let user = create_user(&mut db.conn, "example2.com", UserType::Cusotmer)?;
        assert_eq!(user.user_id, 2);
        let mut user = db.get_user_by_id(2)?.unwrap();
        assert_eq!(user.email, "example2.com");

        user.email = "updated@example.com".into();
        db.save_user(&user)?;
        let user = db.get_user_by_id(2)?.unwrap();
        assert_eq!(user.email, "updated@example.com");

        Ok(())
    }

    #[test]
    #[ignore]
    fn changing_email_from_corporate_to_non_corporate() -> Result<(), Box<dyn error::Error>> {
        // Arrange
        let mut db = get_db();
        let user = create_user(&mut db.conn, "user@mycorp.com", UserType::Employee)?;
        let company = create_company(&mut db.conn, "mycorp.com", 1)?;

        let mut message_bus_mock = MockMessageBus::new();
        message_bus_mock
            .expect_send_email_changed_message()
            .with(eq(user.user_id), eq("new@example.com"))
            .times(1)
            .return_once(|_, _| {});

        let sut = UserController::new(db, message_bus_mock);

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
