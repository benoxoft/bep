use crate::schema::entities_files;
use crate::schema::entities_files::dsl;

use chrono::Utc;
use chrono::naive::NaiveDateTime;

use diesel::prelude::*;
use diesel::pg::PgConnection;

use serde_derive::{Deserialize, Serialize};

use std::vec::Vec;

#[derive(Insertable, Queryable, Identifiable, AsChangeset, Debug, Serialize, Deserialize)]
#[primary_key(file_id, entity_id)]
pub struct EntitiesFile {
    file_id: uuid::Uuid,
    entity_id: uuid::Uuid,
    deleted: bool,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
    deleted_at: chrono::NaiveDateTime
}

impl PartialEq for EntitiesFile {
    fn eq(&self, other: &Self) -> bool {
        self.file_id == other.file_id &&
        self.entity_id == other.entity_id &&
        self.deleted == other.deleted &&
        self.created_at.timestamp() == other.created_at.timestamp() &&
        self.updated_at.timestamp() == other.updated_at.timestamp() &&
        self.deleted_at.timestamp() == other.deleted_at.timestamp()
    }
}

impl EntitiesFile {
    pub fn new(
        file_id: uuid::Uuid,
        entity_id: uuid::Uuid
    ) -> EntitiesFile {
        EntitiesFile{
            file_id,
            entity_id,
            deleted: false,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
            deleted_at: NaiveDateTime::from_timestamp(0, 0)
        }
    }

    pub fn id(&self) -> uuid::Uuid {
        self.file_id 
    }

    pub(super) fn insert(conn: &PgConnection, entfile: &EntitiesFile) -> EntitiesFile {
        diesel::insert_into(entities_files::table)
            .values(entfile)
            .get_result(conn)
            .expect("Error saving entities files")
    }

    fn get_by_file_id(conn: &PgConnection, fileid: uuid::Uuid) -> Vec<EntitiesFile> {
        entities_files::table.load::<EntitiesFile>(conn)
            .expect("Error loading entities files")
    }

}

#[cfg(test)]
pub mod test_functions {
    use super::EntitiesFile;
    use super::super::files::{File, test_functions::*};
    use super::super::registers::{Register, test_functions::*};

    use diesel::PgConnection;

    pub fn create_test_entity_file(conn: &PgConnection) -> EntitiesFile {
        let test_file = create_test_file(&conn);
        File::insert(&conn, &test_file);
        
        let test_register = create_test_register(&conn);
        Register::insert(&conn, &test_register);

        EntitiesFile::new(test_file.id(), test_register.id())
    }

}

#[cfg(test)]
mod tests {
    use super::{EntitiesFile, Connection};
    use super::test_functions::*;
    use crate::db;
    use diesel::result::Error;

    #[test]
    fn test_insert_entities_files() {
        let conn = db::connection::establish_connection();
        
        conn.test_transaction::<_, Error, _>(|| {
            let entfile = create_test_entity_file(&conn);
            EntitiesFile::insert(&conn, &entfile);
            let stored_entfile = EntitiesFile::get_by_file_id(&conn, entfile.file_id);
            assert_eq!(entfile, stored_entfile[0]);
            Ok(())
        });
    }
}