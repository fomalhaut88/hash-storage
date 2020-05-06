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
    pub data_key: String,
    pub data_block: String,
    pub signature: String,
    pub secret: String,
}


impl Block {
    pub fn get(conn: &MysqlConnection, public_key: &Point,
               data_key: &Vec<u8>) -> Option<Self> {
        match block::table.filter(block::public_key.eq(hex_from_point(public_key)))
                          .filter(block::data_key.eq(hex_from_bytes(data_key)))
                          .first(conn) {
            Ok(block) => Some(block),
            Err(_) => None
        }
    }

    pub fn insert(conn: &MysqlConnection, public_key: &Point, data_key: &Vec<u8>,
                  data_block: &Vec<u8>, signature: &(Bigi, Bigi), secret: &Vec<u8>) {
        diesel::insert_into(block::table).values((
            block::public_key.eq(hex_from_point(public_key)),
            block::data_key.eq(hex_from_bytes(data_key)),
            block::data_block.eq(hex_from_bytes(data_block)),
            block::signature.eq(hex_from_bigi_pair(signature)),
            block::secret.eq(hex_from_bytes(secret)),
        )).execute(conn).unwrap();
    }

    pub fn delete(conn: &MysqlConnection, id: u32) {
        diesel::delete(block::table.filter(block::id.eq(id))).execute(conn).unwrap();
    }

    pub fn update(conn: &MysqlConnection, id: u32, data_block: &Vec<u8>,
                  signature: &(Bigi, Bigi), secret: &Vec<u8>) {
        diesel::update(block::table.filter(block::id.eq(id))).set((
            block::data_block.eq(hex_from_bytes(data_block)),
            block::signature.eq(hex_from_bigi_pair(signature)),
            block::secret.eq(hex_from_bytes(secret)),
        )).execute(conn).unwrap();
    }
}
