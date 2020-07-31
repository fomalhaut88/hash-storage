#![feature(test)]
#![feature(proc_macro_hygiene, decl_macro)]

extern crate test;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;

use serde_derive::{Serialize, Deserialize};
use rocket::http::Status;
use rocket_contrib::json::{Json, JsonValue};

const HASH_STORAGE_BITS: usize = 256;

mod utils;
mod db;
mod schema;
mod block;
mod crypto;

use utils::*;
use crypto::*;
use block::Block;


/* Data structures */

#[derive(Serialize, Deserialize)]
pub struct SaveInput {
    pub public_key: String,
    pub data_group: String,
    pub data_key: String,
    pub data_block: String,
    pub data_version: String,
    pub signature: String,
    pub secret_signature: String,
}


/* API methods */

#[derive(Serialize, Deserialize)]
pub struct DeleteInput {
    pub secret_signature: String,
}


#[get("/version")]
fn version() -> JsonValue {
    json!({"version": env!("CARGO_PKG_VERSION")})
}


#[get("/check/<public_key_hex>")]
fn check(public_key_hex: String, conn: db::Connection) -> Result<Json<JsonValue>, Status> {
    let public_key = hex_to_point(&public_key_hex);
    let exists = Block::check(&conn, &public_key);
    Ok(Json(json!({"exists": exists})))
}


#[get("/groups/<public_key_hex>")]
fn groups(public_key_hex: String, conn: db::Connection) -> Result<Json<JsonValue>, Status> {
    let public_key = hex_to_point(&public_key_hex);
    let records = Block::groups(&conn, &public_key);
    Ok(Json(json!(records)))
}


#[get("/keys/<public_key_hex>/<data_group>")]
fn keys(public_key_hex: String, data_group: String, conn: db::Connection) -> Result<Json<JsonValue>, Status> {
    let public_key = hex_to_point(&public_key_hex);
    let records = Block::keys(&conn, &public_key, &data_group);
    Ok(Json(json!(records)))
}


#[get("/list/<public_key_hex>/<data_group>")]
fn list(public_key_hex: String, data_group: String, conn: db::Connection) -> Result<Json<JsonValue>, Status> {
    let public_key = hex_to_point(&public_key_hex);
    let records = Block::list(&conn, &public_key, &data_group);
    Ok(Json(json!(records)))
}


#[get("/get/<public_key_hex>/<data_group>/<data_key>")]
fn get(public_key_hex: String, data_group: String, data_key: String, conn: db::Connection) -> Result<Json<Block>, Status> {
    let public_key = hex_to_point(&public_key_hex);
    match Block::get(&conn, &public_key, &data_group, &data_key) {
        Some(record) => Ok(Json(record)),
        None => Err(Status::NotFound)
    }
}


#[post("/save", format = "application/json", data = "<input>")]
fn save(input: Json<SaveInput>, conn: db::Connection) -> Result<Json<Block>, Status> {
    let public_key = hex_to_point(&input.public_key);
    let data_group = &input.data_group;
    let data_key = &input.data_key;
    let data_block = &input.data_block;
    let data_version = &input.data_version;
    let signature = hex_to_bigi_pair(&input.signature);

    let secret_signature_option = if !input.secret_signature.is_empty() {
        Some(hex_to_bigi_pair(&input.secret_signature))
    } else {
        None
    };

    if check_data_block_size(&data_block) {
        if check_data_signature(&public_key, &data_group, &data_key, &data_block, &data_version, &signature) {
            match Block::get(&conn, &public_key, &data_group, &data_key) {
                Some(record) => {
                    match secret_signature_option {
                        Some(secret_signature) => {
                            let secret = hex_to_bytes(&record.secret);
                            if check_secret_signature(&public_key, &secret, &secret_signature) {
                                let secret = generate_secret();
                                Block::update(&conn, record.id, &data_block, &data_version, &signature, &secret);
                                let new_record = Block::get(&conn, &public_key, &data_group, &data_key).unwrap();
                                Ok(Json(new_record))
                            } else {
                                Err(Status::Forbidden)
                            }
                        },
                        None => {
                            Err(Status::Forbidden)
                        }
                    }
                },
                None => {
                    let secret = generate_secret();
                    Block::insert(&conn, &public_key, &data_group, &data_key, &data_block, &data_version, &signature, &secret);
                    let new_record = Block::get(&conn, &public_key, &data_group, &data_key).unwrap();
                    Ok(Json(new_record))
                }
            }
        } else {
            Err(Status::Forbidden)
        }
    } else {
        Err(Status::BadRequest)
    }
}


#[post("/delete/<public_key_hex>/<data_group>/<data_key>", format = "application/json", data = "<input>")]
fn delete(public_key_hex: String, data_group: String, data_key: String, input: Json<DeleteInput>, conn: db::Connection) -> Result<Json<JsonValue>, Status> {
    let public_key = hex_to_point(&public_key_hex);
    let secret_signature = hex_to_bigi_pair(&input.secret_signature);

    match Block::get(&conn, &public_key, &data_group, &data_key) {
        Some(record) => {
            let secret = hex_to_bytes(&record.secret);
            if check_secret_signature(&public_key, &secret, &secret_signature) {
                Block::delete(&conn, record.id);
                Ok(Json(json!({"success": true})))
            } else {
                Err(Status::Forbidden)
            }
        },
        None => Err(Status::NotFound)
    }
}


fn main() {
    rocket::ignite()
        .manage(db::connect())
        .mount("/", routes![
            version, check, groups, keys, list, get, save, delete,
        ])
        .launch();
}
