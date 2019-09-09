use crate::schema::files;
use crate::schema::files::dsl;

use chrono::Utc;
use chrono::naive::NaiveDateTime;

use diesel::prelude::*;
use diesel::pg::PgConnection;

use serde_derive::{Deserialize, Serialize};

use std::vec::Vec;

#[derive(Insertable, Queryable, Identifiable, AsChangeset, Debug, Serialize, Deserialize)]
pub struct File {
    id: uuid::Uuid,
    filename: String,
    url: String,
    content: String,
    deleted: bool,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
    deleted_at: chrono::NaiveDateTime
}

impl PartialEq for File {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id &&
        self.filename == other.filename &&
        self.url == other.url &&
        self.content == other.content &&
        self.deleted == other.deleted &&
        self.created_at.timestamp() == other.created_at.timestamp() &&
        // self.updated_at.timestamp() == other.updated_at.timestamp() &&
        self.deleted_at.timestamp() == other.deleted_at.timestamp()
    }
}

impl File {
    pub fn new(
        filename: String,
        url: String,
        content: String
    ) -> File {
        File {
            id: uuid::Uuid::new_v4(),
            filename,
            url,
            content,
            deleted: false,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
            deleted_at: NaiveDateTime::from_timestamp(0, 0)
        }
    }

    pub fn id(&self) -> uuid::Uuid {
        self.id
    }

    pub(super) fn insert(conn: &PgConnection, file: &File) -> File {
        diesel::insert_into(files::table)
            .values(file)
            .get_result(conn)
            .expect("Error saving file")
    }

    fn update(conn: &PgConnection, file: &File) -> File {
        diesel::update(files::table)
            .set(file)
            .get_result(conn)
            .expect("Error saving file")
    }

    fn get_one_by_id(conn: &PgConnection, file_id: uuid::Uuid) -> File {
        dsl::files.filter(dsl::id.eq(file_id)).first(conn)
            .expect("Could not find specified file")
    }

}

#[cfg(test)]
pub mod test_functions {
    use super::File;
    use diesel::PgConnection;

    pub fn create_test_file(conn: &PgConnection) -> File {
        File::new(
            String::from("FILENAME.TXT"), 
            String::from("URL OF FILE"), 
            String::from("FILE CONTENT")
        )
    }
}

#[cfg(test)]
pub mod tests {
    use super::{File, Connection};
    use super::test_functions::*;
    use crate::db;
    use diesel::result::Error;

    #[test]
    fn test_create_new_file() {
        let conn = db::connection::establish_connection();

        conn.test_transaction::<_, Error, _>(|| {
            let file = create_test_file(&conn);
            File::insert(&conn, &file);
            let stored_file = File::get_one_by_id(&conn, file.id());
            assert_eq!(file, stored_file);

            Ok(())
        });
    }

    #[test]
    fn test_update_file() {
        let conn = db::connection::establish_connection();

        conn.test_transaction::<_, Error, _>(|| {
            let mut file = create_test_file(&conn);
            File::insert(&conn, &file);
            assert_eq!(file, File::get_one_by_id(&conn, file.id()));
            file.content = String::from("NEW CONTENT");
            file.deleted = true;
            file.filename = String::from("NEW FILE NAME");
            file.url = String::from("NEW URL");

            File::update(&conn, &file);
            let stored_file = File::get_one_by_id(&conn, file.id());

            assert_eq!(file, stored_file);

            Ok(())
        });
    }

}
