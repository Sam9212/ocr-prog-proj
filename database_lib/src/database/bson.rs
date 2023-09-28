use super::{DatabaseAdapter, DatabaseError, DatabaseErrorKind};
use serde::{Serialize, Deserialize};
use bson::{from_reader, to_vec};
use std::fs::{File, write};

pub struct BsonAdapter {
    pub fp: String,
}

impl DatabaseAdapter for BsonAdapter {
    fn read_db(&self) -> Result<Vec<User>, DatabaseError> {
        let db_file = match File::open(self.fp.clone()) {
            Ok(file) => file,
            Err(_) => {
                return Err(DatabaseError { kind: DatabaseErrorKind::FileNotFound })
            }
        };
        let db: Vec<User> = match from_reader(db_file) {
            Ok(db) => db,
            Err(_) => {
                return Err(DatabaseError { kind: DatabaseErrorKind::BsonDeserializationError })
            }
        };

        Ok(db)
    }

    fn write_db(&self, db: Vec<User>) -> Result<(), DatabaseError> {
        let db_ser = match to_vec(&db) {
            Ok(db) => db,
            Err(_) => {
                return Err(DatabaseError { kind: DatabaseErrorKind::BsonSerializationError})
            }
        };

        match write(self.fp.clone(), db_ser) {
            Ok(_) => {},
            Err(_) => {
                return Err(DatabaseError { kind: DatabaseErrorKind::WriteError})
            }
        };

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    password_hash: String,
    incorrect_strikes: u8,
    access: AccessLevel,
    forename: String,
    surname: String,
    age: u8,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum AccessLevel {
    USER = 0,
    TEACHER = 1,
    ADMIN = 2,
}

impl User {
    pub fn new_user(forename: String, surname: String, age: u8, password_hash: String) -> Self {
        User { forename, surname, age, password_hash, access: AccessLevel::USER, incorrect_strikes: 0 }
    }

    pub fn new_teacher(forename: String, surname: String, age: u8, password_hash: String) -> Self {
        User { forename, surname, age, password_hash, access: AccessLevel::TEACHER, incorrect_strikes: 0 }
    }

    pub fn new_admin(forename: String, surname: String, age: u8, password_hash: String) -> Self {
        User { forename, surname, age, password_hash, access: AccessLevel::ADMIN, incorrect_strikes: 0 }
    }
}