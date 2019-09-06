use crate::schema::building_managers;

use chrono::Utc;
use chrono::naive::NaiveDateTime;

use serde_derive::{Deserialize, Serialize};

use diesel::prelude::*;
use diesel::pg::PgConnection;

use std::vec::Vec;

//use super::coordinates::Coordinate;
//use super::users::User;

use crate::schema::building_managers::dsl;


#[derive(Insertable, Queryable, Identifiable, AsChangeset, Debug, Serialize, Deserialize)]
pub struct BuildingManager {
    id: uuid::Uuid,
    full_name: String,
    profile_picture: Vec<u8>,
    coordinates_id: Option<uuid::Uuid>,
    linked_user_id: Option<uuid::Uuid>,
    deleted: bool,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
    deleted_at: chrono::NaiveDateTime
}

impl PartialEq for BuildingManager {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id &&
        self.full_name == other.full_name &&
        self.profile_picture == other.profile_picture &&
        self.coordinates_id == other.coordinates_id &&
        self.linked_user_id == other.linked_user_id &&
        self.deleted == other.deleted &&
        self.created_at.timestamp() == other.created_at.timestamp() &&
        self.updated_at.timestamp() == other.updated_at.timestamp() &&
        self.deleted_at.timestamp() == other.deleted_at.timestamp()
    }
}

impl BuildingManager {
    pub fn new(
        full_name: String,
        profile_picture: Vec<u8>,
        coordinates_id: Option<uuid::Uuid>,
        linked_user_id: Option<uuid::Uuid>,
    ) -> BuildingManager {
        BuildingManager {
            id: uuid::Uuid::new_v4(),
            full_name: full_name,
            profile_picture: profile_picture,
            coordinates_id: coordinates_id,
            linked_user_id: linked_user_id,
            deleted: false,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
            deleted_at: NaiveDateTime::from_timestamp(0, 0)
        }
    }

    fn insert(conn: &PgConnection, buildm: &BuildingManager) -> BuildingManager {
        diesel::insert_into(building_managers::table)
            .values(buildm)
            .get_result(conn)
            .expect("Error saving new user")
    }

    fn update(conn: &PgConnection, bm: &BuildingManager) -> BuildingManager {
        diesel::update(building_managers::table)
            .set(bm)
            .get_result(conn)
            .expect("Error saving the building manager")
    }

    fn get_one_by_id(conn: &PgConnection, bm_id: uuid::Uuid) -> BuildingManager {
        dsl::building_managers.filter(dsl::id.eq(bm_id)).first(conn)
            .expect("Could not find building manager")
    }
}

#[cfg(test)]
mod tests {
    use super::{BuildingManager, Connection};
    use crate::db;
    use diesel::result::Error;
    use uuid;

    fn create_test_building_manager() -> BuildingManager {
        BuildingManager::new(
            String::from("MANAGER NAME"), 
            String::from("PROFILE PICTURE").into_bytes(), 
            None, 
            None)
    }

    #[test]
    fn test_insert_building_manager() {
        let conn = db::connection::establish_connection();

        conn.test_transaction::<_, Error, _>(|| {
            let bm = create_test_building_manager();
            BuildingManager::insert(&conn, &bm);
            let stored_bm = BuildingManager::get_one_by_id(&conn, bm.id);
            assert_eq!(bm, stored_bm);

            Ok(())
        });
    }

    #[test]
    fn test_update_building_manager() {
        let conn = db::connection::establish_connection();

        conn.test_transaction::<_, Error, _>(|| {
            let mut bm = create_test_building_manager();
            BuildingManager::insert(&conn, &bm);
            assert_eq!(bm, BuildingManager::get_one_by_id(&conn, bm.id));
            bm.coordinates_id = None;
            bm.deleted = true;
            bm.full_name = String::from("NEW FULL NAME");
            bm.linked_user_id = None;
            bm.profile_picture = String::from("NEW PROFILE PICTURE").into_bytes();
            
            BuildingManager::update(&conn, &bm);
            let saved_bm = BuildingManager::get_one_by_id(&conn, bm.id);

            assert_eq!(bm.coordinates_id, saved_bm.coordinates_id);
            assert_eq!(bm.deleted, saved_bm.deleted);
            assert_eq!(bm.full_name, saved_bm.full_name);
            assert_eq!(bm.linked_user_id, saved_bm.linked_user_id);
            assert_eq!(bm.profile_picture, saved_bm.profile_picture);

            Ok(())
        });
    }
}
