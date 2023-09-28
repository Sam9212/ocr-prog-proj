pub mod bson;

use std::fmt::Display;
use bson::{BsonAdapter, User};
use serde::{Serialize, Deserialize};
use std::error::Error;

#[derive(Debug)]
pub struct DatabaseError {
    kind: DatabaseErrorKind
}

#[derive(Debug)]
pub enum DatabaseErrorKind {
    BsonDeserializationError,
    BsonSerializationError,
    SqlDeserializationError,
    SqlSerializationError,
    FileNotFound,
    WriteError,
}

impl Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.kind)
    }
}

impl Error for DatabaseError {}

pub struct DatabaseConnection<T> {
    database_adapter: T,
    database_cache: Option<Vec<User>>,
}

pub trait DatabaseAdapter {
    fn read_db(&self) -> Result<Vec<User>, DatabaseError>;
    fn write_db(&self, db: Vec<User>) -> Result<(), DatabaseError>;
}

impl DatabaseConnection<BsonAdapter> {
    pub fn new_bson(fp: String) -> DatabaseConnection<BsonAdapter> {
        DatabaseConnection { database_adapter: BsonAdapter { fp }, database_cache: None }
    }
}

impl <T: DatabaseAdapter> DatabaseConnection<T> {
    pub fn read_db(&mut self) -> Result<Vec<User>, DatabaseError> {
        if self.database_cache.is_none() {
            match self.database_adapter.read_db() {
                Ok(db) =>  {
                    self.database_cache = Some(db.clone());
                    Ok(db)
                }
                Err(why) => Err(why),
            }
        } else {
            Ok(self.database_cache.clone().unwrap())
        }
    }

    pub fn write_db(&mut self, db: Vec<User>) -> Result<(), DatabaseError> {
        self.database_cache = None;
        self.database_adapter.write_db(db)
    }
}