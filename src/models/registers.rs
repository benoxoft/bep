use crate::schema::registers;
use crate::schema::registers::dsl;

use chrono::Utc;
use chrono::naive::NaiveDateTime;

use diesel::prelude::*;
use diesel::pg::PgConnection;

use serde_derive::{Deserialize, Serialize};

use std::vec::Vec;

#[derive(Insertable, Queryable, Identifiable, AsChangeset, Debug, Serialize, Deserialize)]
#[changeset_options(treat_none_as_null = "true")]
pub struct Register {
    id: uuid::Uuid,
    name: String,
    building_id: uuid::Uuid,
    deleted: bool,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
    deleted_at: chrono::NaiveDateTime
}

impl PartialEq for Register {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id &&
        self.name == other.name && 
        self.building_id == other.building_id &&
        self.deleted == other.deleted &&
        self.created_at.timestamp() == other.created_at.timestamp() &&
        // self.updated_at.timestamp() == other.updated_at.timestamp() &&
        self.deleted_at.timestamp() == other.deleted_at.timestamp()
    }
}

impl Register {
    pub fn new(
        name: String,
        building_id: uuid::Uuid        
    ) -> Register {
        Register {
            id: uuid::Uuid::new_v4(),
            name,
            building_id,
            deleted: false,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
            deleted_at: NaiveDateTime::from_timestamp(0, 0)
        }
    }

    pub fn id(&self) -> uuid::Uuid {
        self.id
    }

    pub(super) fn insert(conn: &PgConnection, register: &Register) -> Register {
        diesel::insert_into(registers::table)
            .values(register)
            .get_result(conn)
            .expect("Error saving new register")
    }

    fn update(conn: &PgConnection, register: &Register) -> Register {
        diesel::update(registers::table)
            .set(register)
            .get_result(conn)
            .expect("Error saving the register")
    }

    fn get_one_by_id(conn: &PgConnection, register_id: uuid::Uuid) -> Register {
        dsl::registers.filter(dsl::id.eq(register_id)).first(conn)
            .expect("Could not find register")
    }

}

#[cfg(test)]
pub mod test_functions {
    use super::Register;
    use super::super::buildings::{Building, test_functions::*};

    use diesel::PgConnection;

    pub fn create_test_register(conn: &PgConnection) -> Register {
        let test_building = create_test_building1(&conn);
        Building::insert(&conn, &test_building);

        Register::new(String::from("TEST REGISTER"), test_building.id())
    }
}

#[cfg(test)]
mod tests {
    use super::{Register, Connection};
    use super::test_functions::*;
    use crate::db;
    use diesel::result::Error;

    #[test]
    fn test_insert_register() {
        let conn = db::connection::establish_connection();

        conn.test_transaction::<_, Error, _>(|| {
            let register = create_test_register(&conn);
            Register::insert(&conn, &register);
            let stored_register = Register::get_one_by_id(&conn, register.id());
            assert_eq!(register, stored_register);

            Ok(())
        });
    }

    #[test]
    fn test_update_register() {
        let conn = db::connection::establish_connection();

        conn.test_transaction::<_, Error, _>(|| {
            let mut register = create_test_register(&conn);
            Register::insert(&conn, &register);
            assert_eq!(register, Register::get_one_by_id(&conn, register.id()));
            register.name = String::from("CHANGED NAME");
            Register::update(&conn, &register);
            let saved_register = Register::get_one_by_id(&conn, register.id());
            assert_eq!(register, saved_register);

            Ok(())
        });
    }
}

