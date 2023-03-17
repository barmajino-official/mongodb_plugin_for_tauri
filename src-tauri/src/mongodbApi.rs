use mongodb::{Client, Collection};
use serde::{Deserialize, Serialize};
use tauri::plugin::Plugin;
use tauri::Result;

#[derive(Deserialize, Serialize)]
struct DBInfo {
    server: String,
    database: String,
}

#[derive(Deserialize)]
struct FindArgs {
    collection: String,
    query: String,
}

#[derive(Deserialize)]
struct InsertOneArgs {
    collection: String,
    data: String,
}

#[derive(Deserialize)]
struct InsertManyArgs {
    collection: String,
    data: String,
}

#[derive(Deserialize)]
struct AggregateArgs {
    collection: String,
    pipeline: String,
}

pub struct MongoPlugin;

use std::process::Command;
use tauri::Runtime;
impl<R: Runtime> Plugin<R> for MongoPlugin {
    fn name(&self) -> &'static str {
        "mongo"
    }

fn extend_api(&mut self, message: Command<'_, R>) {
        tauri::generate_handler!(message, 
            "connectDBServer" => |_ctx, payload: DBInfo| async move {
                let client = match Client::with_uri_str(&payload.server) {
                    Ok(client) => client,
                    Err(e) => {
                        return Err(format!("Failed to connect: {}", e));
                    }
                };
                let db = client.database(&payload.database);
                Ok(db)
            },

            "accessDB" => |_ctx, db| async move {
                Ok(db)
            },

            "find" => |_ctx, db, args: FindArgs| async move {
                let coll = db.collection(&args.collection);
                let query = match serde_json::from_str(&args.query) {
                    Ok(query) => query,
                    Err(e) => return Err(format!("Failed to parse query: {}", e)),
                };
                let cursor = match coll.find(query, None).await {
                    Ok(cursor) => cursor,
                    Err(e) => return Err(format!("Failed to execute query: {}", e)),
                };
                let results = match cursor.into_vec().await {
                    Ok(results) => results,
                    Err(e) => return Err(format!("Failed to read results: {}", e)),
                };
                Ok(serde_json::to_value(results).unwrap())
            },

            "findOne" => |_ctx, db, args: FindArgs| async move {
                let coll = db.collection(&args.collection);
                let query = match serde_json::from_str(&args.query) {
                    Ok(query) => query,
                    Err(e) => return Err(format!("Failed to parse query: {}", e)),
                };
                let result = match coll.find_one(query, None).await {
                    Ok(result) => result,
                    Err(e) => return Err(format!("Failed to execute query: {}", e)),
                };
                Ok(serde_json::to_value(result).unwrap())
            },

            "insertOne" => |_ctx, db, args: InsertOneArgs| async move {
                let coll = db.collection(&args.collection);
                let doc = match serde_json::from_str(&args.data) {
                    Ok(doc) => doc,
                    Err(e) => return Err(format!("Failed to parse document: {}", e)),
                };
                match coll.insert_one(doc, None).await {
                    Ok(_) => Ok(serde_json::to_value("success").unwrap()),
                    Err(e) => Err(format!("Failed to insert document: {}", e)),
                }
            },

            "insertMany" => |_ctx, db, args: InsertManyArgs| async move {
                let coll = db.collection(&args.collection);
                let docs = match serde_json::from_str(&args.data) {
                    Ok(docs) => docs,
                    Err(e) => return Err(format!("Failed to parse documents: {}", e)),
                };
                match coll.insert_many(docs, None).await {
                    Ok(_) => Ok(serde_json::to_value("success").unwrap()),
                    Err(e) => Err(format!("Failed to insert documents: {}", e)),
                }
            },

            "aggregate" => |_ctx, db, args: AggregateArgs| async move {
                let coll = db.collection(&args.collection);
                let pipeline = match serde_json::from_str(&args.pipeline) {
                    Ok(pipeline) => pipeline,
                    Err(e) => return Err(format!("Failed to parse pipeline: {}", e)),
                };
                let cursor = match coll.aggregate(pipeline, None).await {
                    Ok(cursor) => cursor,
                    Err(e) => return Err(format!("Failed to execute aggregation: {}", e)),
                };
                let results = match cursor.into_vec().await {
                    Ok(results) => results,
                    Err(e) => return Err(format!("Failed to read aggregation results: {}", e)),
                };
                Ok(serde_json::to_value(results).unwrap())
            }
    );
}
}
