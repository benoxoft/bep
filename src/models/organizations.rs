use crate::schema::organizations;
use crate::schema::organizations::dsl;

use chrono::Utc;
use chrono::naive::NaiveDateTime;

use diesel::prelude::*;
use diesel::pg::PgConnection;

use serde_derive::{Deserialize, Serialize};

use std::vec::Vec;

#[derive(Insertable, Queryable, Identifiable, AsChangeset, Debug, Serialize, Deserialize)]
#[changeset_options(treat_none_as_null = "true")]
pub struct Organization {
    id: uuid::Uuid,
    org_name: String,
    profile_picture: Vec<u8>,
    coordinates_id: Option<uuid::Uuid>,
    deleted: bool,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
    deleted_at: chrono::NaiveDateTime
}

impl PartialEq for Organization {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id &&
        self.org_name == other.org_name &&
        self.profile_picture == other.profile_picture &&
        self.coordinates_id == other.coordinates_id &&
        self.deleted == other.deleted &&
        self.created_at.timestamp() == other.created_at.timestamp() &&
        self.updated_at.timestamp() == other.updated_at.timestamp() &&
        self.deleted_at.timestamp() == other.deleted_at.timestamp()
    }
}

impl Organization {
    pub fn new(
        org_name: String,
        profile_picture: Vec<u8>,
        coordinates_id: Option<uuid::Uuid>,
    ) -> Organization {
        Organization {
            id: uuid::Uuid::new_v4(),
            org_name,
            profile_picture,
            coordinates_id,
            deleted: false,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
            deleted_at: NaiveDateTime::from_timestamp(0, 0)
        }
    }

    pub fn id(&self) -> uuid::Uuid {
        self.id 
    }

    pub(super) fn insert(conn: &PgConnection, buildm: &Organization) -> Organization {
        diesel::insert_into(organizations::table)
            .values(buildm)
            .get_result(conn)
            .expect("Error saving new user")
    }

    fn update(conn: &PgConnection, org: &Organization) -> Organization {
        diesel::update(organizations::table)
            .set(org)
            .get_result(conn)
            .expect("Error saving the building manager")
    }

    fn get_one_by_id(conn: &PgConnection, org_id: uuid::Uuid) -> Organization {
        dsl::organizations.filter(dsl::id.eq(org_id)).first(conn)
            .expect("Could not find building manager")
    }
}

#[cfg(test)]
pub mod test_functions {
    use super::Organization;
    use super::super::coordinates::{Coordinate, test_functions::*};

    use diesel::PgConnection;

    pub fn create_test_organization1(conn: &PgConnection) -> Organization {
        let test_coord = create_test_coordinate1();
        Coordinate::insert(&conn, &test_coord);

        Organization::new(
            String::from("MANAGER NAME #1"), 
            String::from("PROFILE PICTURE #1").into_bytes(),
            Some(test_coord.id())
        )
    }

    pub fn create_test_organization2(conn: &PgConnection) -> Organization {
        let test_coord = create_test_coordinate1();
        Coordinate::insert(&conn, &test_coord);

        Organization::new(
            String::from("MANAGER NAME #2"), 
            String::from("PROFILE PICTURE #2").into_bytes(),
            Some(test_coord.id())
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{Organization, Connection};
    use super::test_functions::*;
    use crate::db;
    use diesel::result::Error;
    
    #[test]
    fn test_insert_building_manager() {
        let conn = db::connection::establish_connection();

        conn.test_transaction::<_, Error, _>(|| {
            let org = create_test_organization1(&conn);
            Organization::insert(&conn, &org);
            let stored_org = Organization::get_one_by_id(&conn, org.id);
            assert_eq!(org, stored_org);

            Ok(())
        });
    }

    #[test]
    fn test_update_building_manager() {
        let conn = db::connection::establish_connection();

        conn.test_transaction::<_, Error, _>(|| {
            let mut org = create_test_organization2(&conn);
            Organization::insert(&conn, &org);
            assert_eq!(org, Organization::get_one_by_id(&conn, org.id));
            org.coordinates_id = None;
            org.deleted = true;
            org.org_name = String::from("NEW FULL NAME");
            org.profile_picture = String::from("NEW PROFILE PICTURE").into_bytes();
            
            Organization::update(&conn, &org);
            let saved_org = Organization::get_one_by_id(&conn, org.id);

            assert_eq!(org.coordinates_id, saved_org.coordinates_id);
            assert_eq!(org.deleted, saved_org.deleted);
            assert_eq!(org.org_name, saved_org.org_name);
            assert_eq!(org.profile_picture, saved_org.profile_picture);

            Ok(())
        });
    }
}
