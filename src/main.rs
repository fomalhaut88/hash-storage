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


#[derive(Serialize, Deserialize)]
pub struct PublicKeyInput {
    pub public_key: String,
}


#[derive(Serialize, Deserialize)]
pub struct GroupInput {
    pub public_key: String,
    pub data_group: String,
}


#[derive(Serialize, Deserialize)]
pub struct GetInput {
    pub public_key: String,
    pub data_group: String,
    pub data_key: String,
}


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


#[derive(Serialize, Deserialize)]
pub struct DeleteInput {
    pub public_key: String,
    pub data_group: String,
    pub data_key: String,
    pub secret_signature: String,
}


#[get("/version")]
fn version() -> JsonValue {
    json!({"version": env!("CARGO_PKG_VERSION")})
}


#[post("/check", format = "application/json", data = "<input>")]
fn check(input: Json<PublicKeyInput>, conn: db::Connection) -> Result<Json<JsonValue>, Status> {
    let public_key = hex_to_point(&input.public_key);
    let exists = Block::check(&conn, &public_key);
    Ok(Json(json!({"exists": exists})))
}


#[post("/groups", format = "application/json", data = "<input>")]
fn groups(input: Json<PublicKeyInput>, conn: db::Connection) -> Result<Json<JsonValue>, Status> {
    let public_key = hex_to_point(&input.public_key);
    let records = Block::groups(&conn, &public_key);
    Ok(Json(json!(records)))
}


#[post("/keys", format = "application/json", data = "<input>")]
fn keys(input: Json<GroupInput>, conn: db::Connection) -> Result<Json<JsonValue>, Status> {
    let public_key = hex_to_point(&input.public_key);
    let data_group = &input.data_group;
    let records = Block::keys(&conn, &public_key, &data_group);
    Ok(Json(json!(records)))
}


#[post("/list", format = "application/json", data = "<input>")]
fn list(input: Json<GroupInput>, conn: db::Connection) -> Result<Json<JsonValue>, Status> {
    let public_key = hex_to_point(&input.public_key);
    let data_group = &input.data_group;
    let records = Block::list(&conn, &public_key, &data_group);
    Ok(Json(json!(records)))
}


#[post("/get", format = "application/json", data = "<input>")]
fn get(input: Json<GetInput>, conn: db::Connection) -> Result<Json<Block>, Status> {
    let public_key = hex_to_point(&input.public_key);
    let data_group = &input.data_group;
    let data_key = &input.data_key;

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


#[post("/delete", format = "application/json", data = "<input>")]
fn delete(input: Json<DeleteInput>, conn: db::Connection) -> Result<Json<JsonValue>, Status> {
    let public_key = hex_to_point(&input.public_key);
    let data_group = &input.data_group;
    let data_key = &input.data_key;
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
            version, check, groups, keys, list, get, save, delete
        ])
        .launch();
}
