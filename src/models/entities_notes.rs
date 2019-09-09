use crate::schema::entities_notes;
use crate::schema::entities_notes::dsl;

use chrono::Utc;
use chrono::naive::NaiveDateTime;

use diesel::prelude::*;
use diesel::pg::PgConnection;

use serde_derive::{Deserialize, Serialize};

use std::vec::Vec;

#[derive(Insertable, Queryable, Identifiable, AsChangeset, Debug, Serialize, Deserialize)]
#[changeset_options(treat_none_as_null = "true")]
pub struct EntitiesNote {
    id: uuid::Uuid,
    entity_id: uuid::Uuid,
    user_id: uuid::Uuid,
    note: String,
    deleted: bool,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
    deleted_at: chrono::NaiveDateTime
}

impl PartialEq for EntitiesNote {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id &&
        self.entity_id == other.entity_id &&
        self.user_id == other.user_id &&
        self.note == other.note &&
        self.deleted == other.deleted &&
        self.created_at.timestamp() == other.created_at.timestamp() &&
        // self.updated_at.timestamp() == other.updated_at.timestamp() &&
        self.deleted_at.timestamp() == other.deleted_at.timestamp()
    }
}

impl EntitiesNote {
    pub fn new(
        entity_id: uuid::Uuid,
        user_id: uuid::Uuid,
        note: String,
    ) -> EntitiesNote {
        EntitiesNote {
            id: uuid::Uuid::new_v4(),
            entity_id,
            user_id,
            note,
            deleted: false,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
            deleted_at: NaiveDateTime::from_timestamp(0, 0)
        }
    }

    pub fn id(&self) -> uuid::Uuid {
        self.id
    }

    fn insert(conn: &PgConnection, entnote: &EntitiesNote) -> EntitiesNote {
        diesel::insert_into(entities_notes::table)
            .values(entnote)
            .get_result(conn)
            .expect("Error saving new note")
    }

    fn update(conn: &PgConnection, entnote: &EntitiesNote) -> EntitiesNote {
        diesel::update(entities_notes::table)
            .set(entnote)
            .get_result(conn)
            .expect("Error saving note")
    }

    fn get_one_by_id(conn: &PgConnection, entnote_id: uuid::Uuid) -> EntitiesNote {
        dsl::entities_notes.filter(dsl::id.eq(entnote_id)).first(conn)
            .expect("Could not find entities note")
    }
}

#[cfg(test)]
pub mod test_functions {
    use super::EntitiesNote;
    use super::super::buildings::{Building, test_functions::*};
    use super::super::users::{User, test_functions::*};

    use diesel::PgConnection;

    pub fn create_test_note(conn: &PgConnection) -> EntitiesNote {
        let test_building = create_test_building1(&conn);
        Building::insert(&conn, &test_building);

        let test_user = create_test_user(&conn, String::from("ENTITIES NOTES"));
        User::insert(&conn, &test_user);

        EntitiesNote::new(
            test_building.id(), 
            test_user.id(), 
            String::from("THIS IS A NOTE")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{EntitiesNote, Connection};
    use super::test_functions::*;
    use crate::db;
    use diesel::result::Error;

    #[test]
    fn test_insert_entity_note() {
        let conn = db::connection::establish_connection();

        conn.test_transaction::<_, Error, _>(|| {
            let entnote = create_test_note(&conn);
            EntitiesNote::insert(&conn, &entnote);
            let stored_entnote = EntitiesNote::get_one_by_id(&conn, entnote.id);
            assert_eq!(entnote, stored_entnote);

            Ok(())
        });
    }

    #[test]
    fn test_update_entity_note() {
        let conn = db::connection::establish_connection();

        conn.test_transaction::<_, Error, _>(|| {
            let mut entnote = create_test_note(&conn);
            EntitiesNote::insert(&conn, &entnote);
            assert_eq!(entnote, EntitiesNote::get_one_by_id(&conn, entnote.id()));
            entnote.note = String::from("THIS IS A NEW NOTE");

            EntitiesNote::update(&conn, &entnote);
            let stored_note = EntitiesNote::get_one_by_id(&conn, entnote.id());
            assert_eq!(entnote, stored_note);

            Ok(())
        });
    }
}