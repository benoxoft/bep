use crate::schema::buildings;

use chrono::Utc;
use chrono::naive::NaiveDateTime;

use serde_derive::{Deserialize, Serialize};

use diesel::prelude::*;
use diesel::pg::PgConnection;

use std::vec::Vec;

use crate::schema::buildings::dsl;

#[derive(Insertable, Queryable, Identifiable, AsChangeset, Debug, Serialize, Deserialize)]
pub struct Building {
    id: uuid::Uuid,
    owner_id: uuid::Uuid,
    manager_id: uuid::Uuid,
    respondant_id: uuid::Uuid,
    name: String,
    address: String,
    deleted: bool,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
    deleted_at: chrono::NaiveDateTime
}

impl PartialEq for Building {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id &&
        self.owner_id == other.owner_id &&
        self.manager_id == other.manager_id &&
        self.respondant_id == other.respondant_id &&
        self.name == other.name &&
        self.address == other.address &&
        self.deleted == other.deleted &&
        self.created_at.timestamp() == other.created_at.timestamp() &&
        // self.updated_at.timestamp() == other.updated_at.timestamp() &&
        self.deleted_at.timestamp() == other.deleted_at.timestamp()
    }
}

impl Building {
    pub fn new(
        owner_id: uuid::Uuid,
        manager_id: uuid::Uuid,
        respondant_id: uuid::Uuid,
        name: String, 
        address: String,
    ) -> Building {
        Building {
            id: uuid::Uuid::new_v4(),
            owner_id: owner_id,
            manager_id: manager_id,
            respondant_id: respondant_id,
            name: name,
            address: address,
            deleted: false,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
            deleted_at: NaiveDateTime::from_timestamp(0, 0)
        }
    }

    fn insert(conn: &PgConnection, building: &Building) -> Building {
        diesel::insert_into(buildings::table)
            .values(building)
            .get_result(conn)
            .expect("Error saving new building")
    }

    fn update(conn: &PgConnection, building: &Building) -> Building {
        diesel::update(buildings::table)
            .set(building)
            .get_result(conn)
            .expect("Error saving building")
    }

    fn get_one_by_id(conn: &PgConnection, bid: uuid::Uuid) -> Building {
        dsl::buildings.filter(dsl::id.eq(bid)).first(conn)
            .expect("Could not load building")
    }

    fn get_all(conn: &PgConnection) -> Vec<Building> {
        buildings::table.load::<Building>(conn)
            .expect("Error loading buildings")
    }

}

#[cfg(test)]
pub mod test_functions {
    use super::{Building, Connection};
    use diesel::pg::PgConnection;
    
    use super::super::building_managers::{BuildingManager, test_functions::*};
    use super::super::building_owners::{BuildingOwner, test_functions::*};
    use super::super::users::{User, test_functions::*};

    pub fn create_test_building1(conn: &PgConnection) -> Building {
        let test_owner = create_test_building_owner1(&conn);
        BuildingOwner::insert(conn, &test_owner);

        let test_manager = create_test_building_manager1(&conn, String::from("MANAGER BUILDING1"));
        BuildingManager::insert(conn, &test_manager);

        let test_respondant = create_test_user(String::from("RESPONDANT BUILDING1"));
        User::insert(conn, &test_respondant);

        Building::new(
            test_owner.id(), 
            test_manager.id(), 
            test_respondant.id(), 
            String::from("BUILDING NAME #1"), 
            String::from("BUILDING ADDRESS #1"))
    }

    pub fn create_test_building2(conn: &PgConnection) -> Building {
        let test_owner = create_test_building_owner2(&conn);
        BuildingOwner::insert(conn, &test_owner);

        let test_manager = create_test_building_manager2(&conn, String::from("MANAGER BUILDING2"));
        BuildingManager::insert(conn, &test_manager);

        let test_respondant = create_test_user(String::from("RESPONDANT BUILDING2"));
        User::insert(conn, &test_respondant);

        Building::new(
            test_owner.id(), 
            test_manager.id(), 
            test_respondant.id(), 
            String::from("BUILDING NAME #2"), 
            String::from("BUILDING ADDRESS #2"))
    }

}

#[cfg(test)]
mod tests {
    use super::{Building, Connection};
    use crate::db;
    use diesel::result::Error;
    use super::test_functions::*;

    #[test]
    fn test_create_new_building() {

        let conn = db::connection::establish_connection();
        
        conn.test_transaction::<_, Error, _>(|| {
            let b1 = create_test_building1(&conn);
            let b2 = create_test_building2(&conn);

            Building::insert(&conn, &b1);
            Building::insert(&conn, &b2);

            let buildings = Building::get_all(&conn);

            assert_eq!(buildings[0], b1);
            assert_eq!(buildings[1], b2);

            Ok(())
        });
    }
    
    #[test]
    fn test_update_building_info() {
        let conn = db::connection::establish_connection();
        
        conn.test_transaction::<_, Error, _>(|| {
            let mut b1 = create_test_building2(&conn);

            Building::insert(&conn, &b1);
            b1.name = String::from("Réjean Tremblay");

            Building::update(&conn, &b1);

            let stored_b = Building::get_one_by_id(&conn, b1.id);

            assert_eq!(stored_b, b1);

            Ok(())
        });

    }
}
