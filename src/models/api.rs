use super::models::buildings::Building;
use super::models::building_owners::BuildingOwner;
use super::models::users::User;

use diesel::PgConnection;

pub enum APIError {

}

pub fn register_new_organisation(conn: &PgConnection, user: User, organization: BuildingOwner, invites: Vec<User>) -> Result<(), APIError> {
    // validate
    User::insert(&conn, &user);
    
    Ok(())
}

//pub fn create_new_building() -> Building {

//}
