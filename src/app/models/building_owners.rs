use crate::schema::building_owners;

use serde_derive::{Deserialize, Serialize};

use diesel::prelude::*;
use diesel::pg::PgConnection;

use std::vec::Vec;
