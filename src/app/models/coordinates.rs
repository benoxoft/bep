use crate::schema::coordinates;

use chrono::Utc;
use chrono::naive::NaiveDateTime;

use serde_derive::{Deserialize, Serialize};

use diesel::prelude::*;
use diesel::pg::PgConnection;

use crate::schema::coordinates::dsl;

#[derive(Insertable, Queryable, Identifiable, AsChangeset, Debug, Serialize, Deserialize)]
pub struct Coordinate {
    id: uuid::Uuid,
    address: String,
    telephone_no: String,
    fax: String,
    cellphone_no: String,
    email: String,
    company_name: String,
    company_number: String,
    deleted: bool,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
    deleted_at: chrono::NaiveDateTime
}

impl PartialEq for Coordinate {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id &&
        self.address == other.address &&
        self.telephone_no == other.telephone_no &&
        self.fax == other.fax &&
        self.cellphone_no == other.cellphone_no &&
        self.email == other.email &&
        self.company_name == other.company_name &&
        self.company_number == other.company_number &&
        self.deleted == other.deleted &&
        self.created_at.timestamp() == other.created_at.timestamp() &&
        self.updated_at.timestamp() == other.updated_at.timestamp() &&
        self.deleted_at.timestamp() == other.deleted_at.timestamp()
    }
}

impl Coordinate {
    pub fn new(
        address: String,
        telephone_no: String,
        fax: String,
        cellphone_no: String,
        email: String,
        company_name: String,
        company_number: String        
    ) -> Coordinate {
        Coordinate {
            id: uuid::Uuid::new_v4(),
            address: address,
            telephone_no: telephone_no,
            fax: fax,
            cellphone_no: cellphone_no,
            email: email,
            company_name: company_name,
            company_number: company_number,
            deleted: false,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
            deleted_at: Utc::now().naive_utc()
        }
    }

    fn insert(conn: &PgConnection, coordinate: &Coordinate) -> Coordinate {
        diesel::insert_into(coordinates::table)
            .values(coordinate)
            .get_result(conn)
            .expect("Error saving coordinate")
    }

    fn update(conn: &PgConnection, coordinate: &Coordinate) -> Coordinate {
        diesel::update(coordinates::table)
            .set(coordinate)
            .get_result(conn)
            .expect("Error saving coordinate")
    }

    fn get_one_by_id(conn: &PgConnection, coord_id: uuid::Uuid) -> Coordinate {
        dsl::coordinates.filter(dsl::id.eq(coord_id)).first(conn)
            .expect("Could not find")
    }
    
}

#[cfg(test)]
mod tests {
    use super::{Coordinate, Connection};
    use crate::db;
    use diesel::result::Error;

    fn create_test_coordinate() -> Coordinate {
        Coordinate::new(String::from("123 rue du Sablier"),
                        String::from("TELEPHONE NUMBER"),
                        String::from("FAX NUMBER"),
                        String::from("CELLPHONE NUMBER"),
                        String::from("email@gmail.com"),
                        String::from("COMPANY NAME"),
                        String::from("COMPANY NUMBER"))
    }

    #[test]
    fn test_create_coordinate() {
        let conn = db::connection::establish_connection();

        conn.test_transaction::<_, Error, _>(|| {
            let coord = create_test_coordinate();
            Coordinate::insert(&conn, &coord);
            let stored_coord = Coordinate::get_one_by_id(&conn, coord.id);
            assert_eq!(stored_coord, coord);

            Ok(())
        });

    }

    #[test]
    fn test_update_coordinate() {
        let conn = db::connection::establish_connection();

        conn.test_transaction::<_, Error, _>(|| {
            let mut coord = create_test_coordinate();
            Coordinate::insert(&conn, &coord);
            assert_eq!(coord, Coordinate::get_one_by_id(&conn, coord.id));
            coord.address = String::from("NEW TEST ADDRESS");
            coord.cellphone_no = String::from("CHANGED CELLPHONE NO");
            coord.company_name = String::from("CHANGED COMPANY NAME");
            coord.company_number = String::from("CHANGED COMPANY NUMBER");
            coord.deleted = true;
            coord.email = String::from("CHANGED EMAIL");
            coord.fax = String::from("CHANGED FAX");
            coord.telephone_no = String::from("CHANGED TELEPHONE NO");
            
            Coordinate::update(&conn, &coord);
            let saved_coord = Coordinate::get_one_by_id(&conn, coord.id);

            assert_eq!(coord.address, saved_coord.address);
            assert_eq!(coord.cellphone_no, saved_coord.cellphone_no);
            assert_eq!(coord.company_name, saved_coord.company_name);
            assert_eq!(coord.company_number, saved_coord.company_number);
            assert_eq!(coord.deleted, saved_coord.deleted);
            assert_eq!(coord.email, saved_coord.email);
            assert_eq!(coord.fax, saved_coord.fax);
            assert_eq!(coord.telephone_no, saved_coord.telephone_no);
            assert_ne!(coord.updated_at, saved_coord.updated_at);

            Ok(())
        });
    }

}