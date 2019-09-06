use crate::schema::building_owners;

use chrono::Utc;

use serde_derive::{Deserialize, Serialize};

use diesel::prelude::*;
use diesel::pg::PgConnection;

use crate::schema::building_owners::dsl;

#[derive(Insertable, Queryable, Identifiable, AsChangeset, Debug, Serialize, Deserialize)]
pub struct BuildingOwner {
    id: uuid::Uuid,
    full_name: String,
    is_manager: bool,
    manager_id: Option<uuid::Uuid>,
    linked_user_id: Option<uuid::Uuid>,
    coordinates_id: Option<uuid::Uuid>,
    deleted: bool,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
    deleted_at: chrono::NaiveDateTime
}

impl PartialEq for BuildingOwner {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id &&
        self.full_name == other.full_name &&
        self.is_manager == other.is_manager &&
        self.manager_id == other.manager_id &&
        self.linked_user_id == other.linked_user_id &&
        self.coordinates_id == other.coordinates_id &&
        self.deleted == other.deleted &&
        self.created_at.timestamp() == other.created_at.timestamp() &&
        self.updated_at.timestamp() == other.updated_at.timestamp() &&
        self.deleted_at.timestamp() == other.deleted_at.timestamp()
    }
}

impl BuildingOwner {
    pub fn new(
        full_name: String,
        is_manager: bool,
        manager_id: Option<uuid::Uuid>,
        linked_user_id: Option<uuid::Uuid>,
        coordinates_id: Option<uuid::Uuid>
    ) -> BuildingOwner {
        BuildingOwner {
            id: uuid::Uuid::new_v4(),
            full_name: full_name,
            is_manager: is_manager,
            manager_id: manager_id,
            linked_user_id: linked_user_id,
            coordinates_id: coordinates_id,
            deleted: false,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
            deleted_at: Utc::now().naive_utc()
        }
    }

    fn insert(conn: &PgConnection, bo: &BuildingOwner) -> BuildingOwner {
        diesel::insert_into(building_owners::table)
            .values(bo)
            .get_result(conn)
            .expect("Error saving building owners")
    }

    fn update(conn: &PgConnection, bo: &BuildingOwner) -> BuildingOwner {
        diesel::update(building_owners::table)
            .set(bo)
            .get_result(conn)
            .expect("Error saving building owner")
    }

    fn get_one_by_id(conn: &PgConnection, bo_id: uuid::Uuid) -> BuildingOwner {
        dsl::building_owners.filter(dsl::id.eq(bo_id)).first(conn)
            .expect("Could not find building owner")
    }
}

#[cfg(test)]
mod tests {
    use super::{BuildingOwner, Connection};
    use crate::db;
    use diesel::result::Error;

    fn create_test_building_owner() -> BuildingOwner {
        BuildingOwner::new(String::from("FULL NAME"), false, None, None, None)
    }

    #[test]
    fn test_create_building_owner() {
        let conn = db::connection::establish_connection();

        conn.test_transaction::<_, Error, _>(|| {
            let bo = create_test_building_owner();
            BuildingOwner::insert(&conn, &bo);
            let stored_bo = BuildingOwner::get_one_by_id(&conn, bo.id);
            assert_eq!(bo, stored_bo);

            Ok(())
        });
    }

    #[test]
    fn test_update_building_owner() {
        let conn = db::connection::establish_connection();

        conn.test_transaction::<_, Error, _>(|| {
            let mut bo = create_test_building_owner();
            
            BuildingOwner::insert(&conn, &bo);
            assert_eq!(bo, BuildingOwner::get_one_by_id(&conn, bo.id));
            bo.coordinates_id = None;
            bo.deleted = true;
            bo.full_name = String::from("NEW FULL NAME");
            bo.is_manager = true;
            bo.linked_user_id = None;
            bo.manager_id = None;
            
            BuildingOwner::update(&conn, &bo);
            let saved_bo = BuildingOwner::get_one_by_id(&conn, bo.id);

            assert_eq!(bo.coordinates_id, saved_bo.coordinates_id);
            assert_eq!(bo.deleted, saved_bo.deleted);
            assert_eq!(bo.full_name, saved_bo.full_name);
            assert_eq!(bo.is_manager, saved_bo.is_manager);
            assert_eq!(bo.linked_user_id, saved_bo.linked_user_id);
            assert_eq!(bo.manager_id, saved_bo.manager_id);

            Ok(())
        });
    }
}
