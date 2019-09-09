use crate::schema::entities_history;
use crate::schema::entities_history::dsl;

use chrono::Utc;
use chrono::naive::NaiveDateTime;

use diesel::prelude::*;
use diesel::pg::PgConnection;

use serde_derive::{Deserialize, Serialize};

#[derive(Insertable, Queryable, Identifiable, AsChangeset, Debug, Serialize, Deserialize)]
#[table_name = "entities_history"]
pub struct EntitiesHistory {
    id: uuid::Uuid,
    entity_id: uuid::Uuid,
    action_id: i16,
    file_id: uuid::Uuid,
    user_id: uuid::Uuid,
    deleted: bool,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
    deleted_at: chrono::NaiveDateTime
}

impl PartialEq for EntitiesHistory {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id &&
        self.entity_id == other.entity_id &&
        self.action_id == other.action_id &&
        self.file_id == other.file_id &&
        self.user_id == other.user_id &&
        self.deleted == other.deleted &&
        self.created_at.timestamp() == other.created_at.timestamp() &&
        self.updated_at.timestamp() == other.updated_at.timestamp() &&
        self.deleted_at.timestamp() == other.deleted_at.timestamp()
    }
}

impl EntitiesHistory {
    pub fn new(
        entity_id: uuid::Uuid,
        action_id: i16,
        file_id: uuid::Uuid,
        user_id: uuid::Uuid        
    ) -> EntitiesHistory {
        EntitiesHistory {
            id: uuid::Uuid::new_v4(),
            entity_id,
            action_id,
            file_id,
            user_id,
            deleted: false,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
            deleted_at: NaiveDateTime::from_timestamp(0, 0)
        }
    }

    pub fn id(&self) -> uuid::Uuid {
        self.id
    }

    pub(super) fn insert(conn: &PgConnection, enthist: &EntitiesHistory) -> EntitiesHistory {
        diesel::insert_into(entities_history::table)
            .values(enthist)
            .get_result(conn)
            .expect("Error saving new entities history")
    }

    fn get_one_by_id(conn: &PgConnection, enthist_id: uuid::Uuid) -> EntitiesHistory {
        dsl::entities_history.filter(dsl::id.eq(enthist_id)).first(conn)
            .expect("Could not find entities history")
    }

}

#[cfg(test)]
pub mod test_functions {
    use super::EntitiesHistory;
    use super::super::files::{File, test_functions::*};
    use super::super::users::{User, test_functions::*};
    use super::super::registers::{Register, test_functions::*};

    use diesel::PgConnection;

    pub fn create_test_entities_histories(conn: &PgConnection) -> EntitiesHistory {
        let test_file = create_test_file(&conn);
        File::insert(&conn, &test_file);

        let test_user = create_test_user(&conn, String::from("ENTITY HISTORY"));
        User::insert(&conn, &test_user);

        let test_register = create_test_register(&conn);
        Register::insert(&conn, &test_register);

        EntitiesHistory::new(
            test_register.id(), 
            1, 
            test_file.id(), 
            test_user.id()
        )
    }

}

#[cfg(test)]
mod tests {
    use super::{EntitiesHistory, Connection};
    use super::test_functions::*;
    use crate::db;
    use diesel::result::Error;

    #[test]
    fn test_insert_entities_history() {
        let conn = db::connection::establish_connection();

        conn.test_transaction::<_, Error, _>(|| {
            let enthist = create_test_entities_histories(&conn);
            EntitiesHistory::insert(&conn, &enthist);
            let stored_enthist = EntitiesHistory::get_one_by_id(&conn, enthist.id());
            assert_eq!(enthist, stored_enthist);

            Ok(())
        });
    }
}