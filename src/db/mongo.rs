use crate::db::schemas::OrderRequest;
use mongodb::bson::oid::Error;
use mongodb::{
    bson::{self, doc, oid::ObjectId, Document},
    sync::{Collection, Database},
};

use crate::db::{
    self,
    schemas::{Community, Order, OrderStatus, User},
};

use super::schemas::CommunityRequest;

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

    pub fn get_user(&self, id: &str) -> Result<Option<User>, Error> {
        let id = ObjectId::parse_str(id)?;
        let filter = doc! {"_id": id };
        let col = DBMongo::col::<User>(self, "users");
        let user = col.find_one(filter, None).expect("Error getting user");

        Ok(user)
    }

    pub fn get_community(&self, id: &str) -> Result<Option<Community>, Error> {
        let id = ObjectId::parse_str(id)?;
        let filter = doc! {"_id": id };
        let col = DBMongo::col::<Community>(self, "communities");
        let comm = col.find_one(filter, None).expect("Error getting community");

        Ok(comm)
    }

    pub fn get_communities(&self, params: &CommunityRequest) -> Result<Vec<Community>, Error> {
        let mut filter = Document::new();
        filter.insert("public", true);
        if let Some(id) = &params._id {
            let id = ObjectId::parse_str(id)?;
            filter.insert("_id", id);
        }
        if let Some(code) = &params.currency {
            filter.insert("currencies", code);
        }

        let col = DBMongo::col::<Community>(self, "communities");
        let cursors = col.find(filter, None).expect("Error getting communitites");

        let comms: Vec<Community> = cursors.map(|doc| doc.unwrap()).collect();

        Ok(comms)
    }

    pub fn get_order(&self, id: &str) -> Result<Option<Order>, Error> {
        let id = ObjectId::parse_str(id)?;
        let filter = doc! {"_id": id };
        let col = DBMongo::col::<Order>(self, "orders");
        let order = col.find_one(filter, None).expect("Error getting order");

        Ok(order)
    }

    pub fn get_orders(&self, params: &OrderRequest) -> Result<Vec<Order>, Error> {
        let mut filter = Document::new();
        filter.insert("status", bson::to_bson(&OrderStatus::Pending).unwrap());
        if let Some(id) = &params._id {
            let id = ObjectId::parse_str(id).unwrap();
            filter.insert("_id", id);
        }
        // TODO: This can be uncommented only after having paginator
        // if let Some(status) = &params.status {
        //     filter.insert("status", bson::to_bson(&status).unwrap());
        // }
        if let Some(d) = &params.direction {
            filter.insert("type", d);
        }
        if let Some(fiat) = &params.currency {
            filter.insert("fiat_code", fiat);
        }
        if let Some(community_id) = &params.community_id {
            filter.insert("community_id", community_id);
        }
        let col = DBMongo::col::<Order>(self, "orders");
        let cursors = col.find(filter, None).expect("Error getting orders");

        let orders: Vec<Order> = cursors
            .map(|doc| doc.unwrap())
            .into_iter()
            .filter(|o| o.tg_channel_message1.is_some())
            .collect();

        Ok(orders)
    }
}
