#[cfg(test)]
pub mod test_helper {
    use crate::ch_08::types::{Company, User, UserType};
    use rusqlite::{Connection, Result};

    pub fn create_db() -> Result<Connection> {
        let conn = Connection::open_in_memory()?;

        conn.execute(
            "CREATE TABLE user (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                email TEXT NOT NULL,
                email_confirmed INT NOT NULL DEFAULT TRUE,
                user_type TEXT collate BINARY NOT NULL,
                CHECK (user_type = 'CUSTOMER' OR user_type = 'EMPLOYEE')
            )",
            (),
        )?;

        conn.execute(
            "CREATE TABLE company (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                domain TEXT NOT NULL,
                number_of_employees INTEGER NOT NULl
            )",
            (),
        )?;

        println!("{}", rusqlite::version());

        Ok(conn)
    }

    pub fn create_user(
        conn: &mut Connection,
        email: impl Into<String>,
        user_type: UserType,
    ) -> Result<User> {
        let mut user = User {
            email: email.into(),
            email_confirmed: true,
            email_changed_events: vec![],
            user_id: 0,
            user_type,
        };

        let tx = conn.transaction()?;

        tx.execute(
            "INSERT INTO user (email, user_type) VALUES (?1, ?2)",
            (&user.email, &user.user_type.to_string()),
        )?;

        user.user_id = tx.last_insert_rowid();

        tx.commit()?;
        Ok(user)
    }

    pub fn create_company(
        conn: &mut Connection,
        domain: impl Into<String>,
        number_of_employees: i64,
    ) -> Result<Company> {
        let mut company = Company {
            id: 0,
            domain_name: domain.into(),
            number_of_employees,
        };

        let tx = conn.transaction()?;
        tx.execute(
            "
        INSERT INTO company (domain, number_of_employees) VALUES (?1, ?2)",
            (&company.domain_name, number_of_employees),
        )?;

        company.id = tx.last_insert_rowid();
        tx.commit()?;

        Ok(company)
    }

    pub fn last_insert_rowid(conn: &Connection) -> i64 {
        conn.last_insert_rowid()
    }
}
