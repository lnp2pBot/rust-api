use mongodb::{
    bson::{doc, oid::ObjectId},
    sync::{Collection, Database},
};
use std::io::Error;

use crate::db::{self, schemas::Community};

pub struct DBMongo {
    db: Database,
}

impl DBMongo {
    pub fn init() -> Self {
        let db = db::connect::get_database();

        DBMongo { db }
    }

    pub fn col<T>(data_source: &Self, collection_name: &str) -> Collection<T> {
        data_source.db.collection(collection_name)
    }

    pub fn get_community(&self, id: &str) -> Result<Community, Error> {
        let id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": id };
        let col = DBMongo::col::<Community>(&self, "communities");
        let comm = col
            .find_one(filter, None)
            .ok()
            .expect("Error getting community");

        Ok(comm.unwrap())
    }

    pub fn get_communities(&self) -> Result<Vec<Community>, Error> {
        let col = DBMongo::col::<Community>(&self, "communities");
        let cursors = col
            .find(None, None)
            .ok()
            .expect("Error getting communitites");

        let comms: Vec<Community> = cursors.map(|doc| doc.unwrap()).collect();

        Ok(comms)
    }
}
