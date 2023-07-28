use std::collections::HashMap;

use uuid::Uuid;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UserEntry {
    pub id: Uuid,
    pub email: String,
    pub password: String,
}

#[derive(Clone)]
struct Credentials {
    pub email: String,
    pub password: String,
}

impl Credentials {
    fn into_entry(&self, id: Uuid) -> UserEntry {
        UserEntry {
            id,
            email: self.email.clone(),
            password: self.password.clone(),
        }
    }
}

pub struct Users {
    entries: HashMap<Uuid, Credentials>,
    emails: HashMap<String, Uuid>,
    capacity: usize,
}

impl Users {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            entries: HashMap::with_capacity(capacity),
            emails: HashMap::with_capacity(capacity),
            capacity,
        }
    }

    pub fn create<E, P>(&mut self, email: E, password: P) -> anyhow::Result<UserEntry>
    where
        E: Into<String>,
        P: Into<String>,
    {
        self.ensure_capacity()?;
        let email_string = email.into();
        let id = self.generate_id();
        let entry = Credentials {
            email: email_string.clone(),
            password: password.into(),
        };
        self.entries.insert(id, entry);
        self.emails.insert(email_string, id);
        self.find_by_id(&id)
            .ok_or(anyhow::anyhow!("Failed to create user"))
    }

    pub fn find_by_id(&self, id: &Uuid) -> Option<UserEntry> {
        self.entries
            .get(&id)
            .map(|creds| creds.into_entry(id.clone()))
    }

    pub fn find_by_email<E>(&self, email: E) -> Option<UserEntry>
    where
        E: Into<String>,
    {
        let id = self.emails.get(&email.into())?;
        self.entries
            .get(&id)
            .map(|creds| creds.into_entry(id.clone()))
    }

    fn generate_id(&self) -> Uuid {
        let mut id = Uuid::new_v4();
        while self.entries.get(&id).is_some() {
            id = Uuid::new_v4();
        }
        id
    }

    fn ensure_capacity(&self) -> anyhow::Result<()> {
        if self.entries.len() < self.capacity {
            Ok(())
        } else {
            Err(anyhow::anyhow!("Users database capacity reached"))
        }
    }
}

impl Default for Users {
    fn default() -> Self {
        Self::with_capacity(10)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_user() {
        let email = "email";
        let password = "password";
        let mut users = Users::default();

        let user = users.create(email, password).expect("A user should exists");

        assert_eq!(user.email, email);
        assert_eq!(user.password, password);
    }

    #[test]
    fn find_user_by_id() {
        let mut users = Users::default();
        users
            .create("email", "password")
            .expect("A user 1 should exists");
        let user = users
            .create("email", "password")
            .expect("A user 2 should exists");

        let find = users.find_by_id(&user.id).expect("User 2 should be found");

        assert_eq!(find, user);
    }

    #[test]
    fn find_user_by_email() {
        let mut users = Users::default();
        users
            .create("email1", "password")
            .expect("A user 1 should exists");
        let user = users
            .create("email2", "password")
            .expect("A user 2 should exists");

        let find = users
            .find_by_email(&user.email)
            .expect("User 2 should be found");

        assert_eq!(find, user);
    }

    #[test]
    fn do_not_create_user_when_capacity_reached() {
        let email = "email";
        let password = "password";
        let mut users = Users::with_capacity(2);
        users
            .create(email, password)
            .expect("A user 1 should be added to database");
        users
            .create(email, password)
            .expect("A user 2 should be added to database");

        let mgs = users.create(email, password).err().unwrap().to_string();

        assert_eq!(mgs, "Users database capacity reached");
    }
}
