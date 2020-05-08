#![feature(proc_macro_hygiene, decl_macro)]

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
pub struct ListInput {
    pub public_key: String,
}


#[derive(Serialize, Deserialize)]
pub struct GetInput {
    pub public_key: String,
    pub data_key: String,
}


#[derive(Serialize, Deserialize)]
pub struct SaveInput {
    pub public_key: String,
    pub data_key: String,
    pub data_block: String,
    pub signature: String,
    pub secret_signature: String,
}


#[derive(Serialize, Deserialize)]
pub struct DeleteInput {
    pub public_key: String,
    pub data_key: String,
    pub secret_signature: String,
}


#[post("/list", format = "application/json", data = "<input>")]
fn list(input: Json<ListInput>, conn: db::Connection) -> Result<Json<JsonValue>, Status> {
    let public_key = hex_to_point(&input.public_key);
    let records = Block::list(&conn, &public_key);
    Ok(Json(json!(records)))
}


#[post("/get", format = "application/json", data = "<input>")]
fn get(input: Json<GetInput>, conn: db::Connection) -> Result<Json<Block>, Status> {
    let public_key = hex_to_point(&input.public_key);
    let data_key = hex_to_bytes(&input.data_key);

    match Block::get(&conn, &public_key, &data_key) {
        Some(record) => Ok(Json(record)),
        None => Err(Status::NotFound)
    }
}


#[post("/save", format = "application/json", data = "<input>")]
fn save(input: Json<SaveInput>, conn: db::Connection) -> Result<Json<Block>, Status> {
    let public_key = hex_to_point(&input.public_key);
    let data_key = hex_to_bytes(&input.data_key);
    let data_block = hex_to_bytes(&input.data_block);
    let signature = hex_to_bigi_pair(&input.signature);

    let secret_signature_option = if !input.secret_signature.is_empty() {
        Some(hex_to_bigi_pair(&input.secret_signature))
    } else {
        None
    };

    if check_data_block_size(&data_block) {
        if check_data_signature(&public_key, &data_key, &data_block, &signature) {
            match Block::get(&conn, &public_key, &data_key) {
                Some(record) => {
                    match secret_signature_option {
                        Some(secret_signature) => {
                            let secret = hex_to_bytes(&record.secret);
                            if check_secret_signature(&public_key, &secret, &secret_signature) {
                                let secret = generate_secret();
                                Block::update(&conn, record.id, &data_block, &signature, &secret);
                                let new_record = Block::get(&conn, &public_key, &data_key).unwrap();
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
                    Block::insert(&conn, &public_key, &data_key, &data_block, &signature, &secret);
                    let new_record = Block::get(&conn, &public_key, &data_key).unwrap();
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
    let data_key = hex_to_bytes(&input.data_key);
    let secret_signature = hex_to_bigi_pair(&input.secret_signature);

    match Block::get(&conn, &public_key, &data_key) {
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
        .mount("/api", routes![list, get, save, delete])
        .launch();
}
