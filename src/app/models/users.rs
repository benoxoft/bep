use crate::schema::users;

use chrono::Utc;
use chrono::naive::NaiveDateTime;

use serde_derive::{Deserialize, Serialize};

use diesel::prelude::*;
use diesel::pg::PgConnection;

use std::vec::Vec;

use crate::utils::{HASHER, PWD_SCHEME_VERSION};
use crate::schema::users::dsl;

use libreauth::pass::HashBuilder;


#[derive(Insertable, Queryable, Identifiable, AsChangeset, Debug, Serialize, Deserialize)]
pub struct User {
    id: uuid::Uuid,
    permission: i16,
    full_name: String,
    email: String,
    password: String,
    job_title: String,
    profile_picture: String,
    deleted: bool,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
    deleted_at: chrono::NaiveDateTime
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {

        self.id == other.id &&
        self.permission == other.permission &&
        self.full_name == other.full_name &&
        self.email == other.email &&
        self.password == other.password &&
        self.job_title == other.job_title &&
        self.profile_picture == other.profile_picture &&
        self.deleted == other.deleted &&
        self.created_at.timestamp() == other.created_at.timestamp() &&
        self.updated_at.timestamp() == other.updated_at.timestamp() &&
        self.deleted_at.timestamp() == other.deleted_at.timestamp()
    }
}

impl User {
    pub fn new(
        permission: i16,
        full_name: String,
        email: String,
        password: String,
        job_title: String,
        profile_picture: String
    ) -> User {
        User {
            id: uuid::Uuid::new_v4(),
            permission: permission,
            full_name: full_name,
            email: email,
            password: HASHER.hash(&password).unwrap(),
            job_title: job_title,
            profile_picture: profile_picture,
            deleted: false,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
            deleted_at: NaiveDateTime::from_timestamp(0, 0)
        }
    }

    pub fn encrypted_password(&self) -> &String {
        &self.password
    }

    pub fn change_password(&mut self, old_password_raw: String, new_password_raw: String) -> Result<&User, String> {

        fn new_pass(user: &User, old_password_raw: String, new_password_raw: String) -> Result<String, libreauth::pass::ErrorCode> {
            let checker = HashBuilder::from_phc(&user.password)?;

            if checker.is_valid(&old_password_raw) {
                let hashed_password = HASHER.hash(&new_password_raw)?;
                Ok(hashed_password)
            } else {
                Err(libreauth::pass::ErrorCode::InvalidPasswordFormat)
            }

        };

        match new_pass(&self, old_password_raw, new_password_raw) {
            Ok(encrypted_pass) => {
                self.password = encrypted_pass;
                Ok(self)
            },
            Err(_e) => Err(String::from("Wrong password entered"))
        }

    }

    fn insert(conn: &PgConnection, user: &User) -> User {
        diesel::insert_into(users::table)
            .values(user)
            .get_result(conn)
            .expect("Error saving new user")
    }

    fn update(conn: &PgConnection, user: &User) -> User {
        diesel::update(users::table)
            .set(user)
            .get_result(conn)
            .expect("Error saving user")
    }

    fn get_one_by_id(conn: &PgConnection, user_id: uuid::Uuid) -> User {
        dsl::users.filter(dsl::id.eq(user_id)).first(conn)
            .expect("Could not find user")
    }

    fn get_all(conn: &PgConnection) -> Vec<User> {
        users::table.load::<User>(conn)
            .expect("Error loading users")
    }
}

#[cfg(test)]
mod tests {
    use super::{User, Connection};
    use crate::db;
    use diesel::result::Error;

    fn create_test_user() -> User {
        let permission = 1;
        let full_name = String::from("Lucien Lachance");
        let email = String::from("llachance@gmail.com");
        let password = String::from("supersecretpassword");
        let job_title = String::from("Gestionnaire");
        let profile_picture = String::from("test.png");
        
        User::new(permission, full_name, email, password, job_title, profile_picture)

    }

    #[test]
    fn test_create_new_user() {
        let conn = db::connection::establish_connection();

        conn.test_transaction::<_, Error, _>(|| {
            let user = create_test_user();
            User::insert(&conn, &user);
            let stored_user = User::get_one_by_id(&conn, user.id);
            
            assert_eq!(stored_user, user);
            assert_ne!(user.password, "supersecretpassword");

            Ok(())
        });
    }

    #[test]
    fn test_update_user_info() {
        let conn = db::connection::establish_connection();

        conn.test_transaction::<_, Error, _>(|| {
            let mut user = create_test_user();
            User::insert(&conn, &user);
            user.full_name = String::from("Bernard Landry");
            user.permission = 1;
            user.email = String::from("blandry@gmail.com");
            user.change_password("supersecretpassword".to_owned(), "changedpassword".to_owned()).expect("Should not happen.");
            user.job_title = String::from("Coordinateur");
            user.profile_picture = String::from("new_picture.png");
            user.deleted = true;

            User::update(&conn, &user);
            let saved_user = User::get_one_by_id(&conn, user.id);

            assert_eq!(user.full_name, saved_user.full_name);
            assert_eq!(user.permission, saved_user.permission);
            assert_eq!(user.email, saved_user.email);
            assert_eq!(user.encrypted_password(), saved_user.encrypted_password());
            assert_eq!(user.job_title, saved_user.job_title);
            assert_eq!(user.profile_picture, saved_user.profile_picture);
            assert_eq!(user.deleted, saved_user.deleted);


            Ok(())
        });
    }
}
