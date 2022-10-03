use crate::db::schemas::{OrderRequest, OrdersStatsRequest};
use bson::datetime::DateTime;
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
        let cursor = col.find(filter, None).expect("Error getting communitites");

        let comms: Vec<Community> = cursor.map(|doc| doc.unwrap()).collect();

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
        let cursor = col.find(filter, None).expect("Error getting orders");

        let orders: Vec<Order> = cursor
            .map(|doc| doc.unwrap())
            .into_iter()
            .filter(|o| o.tg_channel_message1.is_some())
            .collect();

        Ok(orders)
    }

    pub fn get_orders_stats(&self, params: &OrdersStatsRequest) -> Result<Vec<Document>, Error> {
        let mut filter = Document::new();
        let mut match_content = Document::new();
        let mut created_at = Document::new();

        if let Some(status) = &params.status {
            match_content.insert("status", status);
        }
        if let Some(date_from) = &params.date_from {
            let date_from = format!("{date_from}T00:00:00Z");
            let from = DateTime::parse_rfc3339_str(date_from).unwrap_or_else(|_| DateTime::now());
            created_at.insert("$gte", from);
        }
        if let Some(date_to) = &params.date_to {
            let date_to = format!("{date_to}T23:59:59Z");
            let to = DateTime::parse_rfc3339_str(date_to).unwrap_or_else(|_| DateTime::now());
            created_at.insert("$lte", to);
        }
        match_content.insert("created_at", created_at);
        filter.insert("$match", match_content);
        let group = doc! {
            "$group": {
                "_id": "$fiat_code",
                "orders": { "$sum": 1 },
                "amount": { "$sum": "$amount" },
            },
        };
        let sort = doc! {
            "$sort": { "orders": -1 },
        };

        let pipeline = vec![filter, group, sort];
        let col = DBMongo::col::<Order>(self, "orders");
        let cursor = col
            .aggregate(pipeline, None)
            .expect("Error getting order stats");
        let orders: Vec<Document> = cursor.map(|doc| doc.unwrap()).collect();

        Ok(orders)
    }
}
