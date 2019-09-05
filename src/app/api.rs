use crate::db;

use super::models::buildings::Building;

pub fn create_new_building() {
    let conn = db::connection::establish_connection();

}
