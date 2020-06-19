use bigi::Bigi;
use bigi_ecc::Point;
use serde_derive::{Serialize, Deserialize};
use diesel;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;

use crate::utils::*;
use crate::schema::block;


#[table_name = "block"]
#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
pub struct Block {
    pub id: u32,
    pub public_key: String,
    pub data_group: String,
    pub data_key: String,
    pub data_block: String,
    pub data_version: String,
    pub signature: String,
    pub secret: String,
}


impl Block {
    pub fn check(conn: &MysqlConnection, public_key: &Point) -> bool {
        let count: i64 = block::table.filter(
            block::public_key.eq(hex_from_point(public_key))
        ).count().get_result(conn).unwrap();
        count > 0
    }

    pub fn groups(conn: &MysqlConnection, public_key: &Point) -> Vec<String> {
        block::table.filter(
            block::public_key.eq(hex_from_point(public_key))
        ).select(block::data_group).load(conn).unwrap()
    }

    pub fn keys(conn: &MysqlConnection, public_key: &Point,
                data_group: &String) -> Vec<String> {
        block::table.filter(block::public_key.eq(hex_from_point(public_key)))
                    .filter(block::data_group.eq(data_group))
                    .select(block::data_key).load(conn).unwrap()
    }

    pub fn list(conn: &MysqlConnection, public_key: &Point,
                data_group: &String) -> Vec<Self> {
        block::table.filter(block::public_key.eq(hex_from_point(public_key)))
                    .filter(block::data_group.eq(data_group))
                    .load(conn).unwrap()
    }

    pub fn get(conn: &MysqlConnection, public_key: &Point,
               data_group: &String, data_key: &String) -> Option<Self> {
        match block::table.filter(block::public_key.eq(hex_from_point(public_key)))
                          .filter(block::data_group.eq(data_group))
                          .filter(block::data_key.eq(data_key))
                          .first(conn) {
            Ok(block) => Some(block),
            Err(_) => None
        }
    }

    pub fn insert(conn: &MysqlConnection, public_key: &Point, data_group: &String,
                  data_key: &String, data_block: &String, data_version: &String,
                  signature: &(Bigi, Bigi), secret: &Vec<u8>) {
        diesel::insert_into(block::table).values((
            block::public_key.eq(hex_from_point(public_key)),
            block::data_group.eq(data_group),
            block::data_key.eq(data_key),
            block::data_block.eq(data_block),
            block::data_version.eq(data_version),
            block::signature.eq(hex_from_bigi_pair(signature)),
            block::secret.eq(hex_from_bytes(secret)),
        )).execute(conn).unwrap();
    }

    pub fn delete(conn: &MysqlConnection, id: u32) {
        diesel::delete(block::table.filter(block::id.eq(id))).execute(conn).unwrap();
    }

    pub fn update(conn: &MysqlConnection, id: u32, data_block: &String,
                  data_version: &String, signature: &(Bigi, Bigi), secret: &Vec<u8>) {
        diesel::update(block::table.filter(block::id.eq(id))).set((
            block::data_block.eq(data_block),
            block::data_version.eq(data_version),
            block::signature.eq(hex_from_bigi_pair(signature)),
            block::secret.eq(hex_from_bytes(secret)),
        )).execute(conn).unwrap();
    }
}
