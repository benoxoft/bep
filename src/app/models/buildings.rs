use crate::schema::buildings;

use serde_derive::{Deserialize, Serialize};

use diesel::prelude::*;
use diesel::pg::PgConnection;

use std::vec::Vec;

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub full_name: String,
    pub job_title: String,
    pub email: String,
    pub profile_picture: String,

}

#[derive(Insertable, Queryable, Identifiable, AsChangeset, Debug, Serialize, Deserialize)]
pub struct Building {
    pub id: uuid::Uuid,
    pub name: String,
    pub address: String,
}

impl PartialEq for Building {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id &&
        self.name == other.name &&
        self.address == other.address
    }
}

impl Building {
    pub fn new(name: String, address: String) -> Building {
        Building {
            id: uuid::Uuid::new_v4(),
            name: name,
            address: address
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
            .expect("Error saving new building")
    }

    fn get_all(conn: &PgConnection) -> Vec<Building> {
        buildings::table.load::<Building>(conn)
            .expect("Error loading buildings")
    }

}

#[cfg(test)]
mod tests {
    use super::{Building, Connection};
    use crate::db;
    use diesel::result::Error;

    #[test]
    fn test_create_new_building() {

        let conn = db::establish_connection();
        
        conn.test_transaction::<_, Error, _>(|| {
            let name1 = String::from("Guy Laliberté");
            let address1 = String::from("test haha 123");
            let b1 = Building::new(name1, address1);

            let name2 = String::from("Réjean Laliberté");
            let address2 = String::from("test haha 456");
            let b2 = Building::new(name2, address2);

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
        let conn = db::establish_connection();
        
        conn.test_transaction::<_, Error, _>(|| {
            let name1 = String::from("Guy Laliberté");
            let address1 = String::from("test haha 123");
            let mut b1 = Building::new(name1, address1);

            Building::insert(&conn, &b1);
            b1.name = String::from("Réjean Tremblay");
            Building::update(&conn, &b1);

            let buildings = Building::get_all(&conn);

            assert_eq!(buildings[0], b1);

            Ok(())
        });

    }
}
