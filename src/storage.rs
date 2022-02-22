//! # Object storage

use std::path::{Path, PathBuf};
use anyhow::{Result, bail};
use crate::db::*;
use std::fs;

pub struct Store {
    pub base: PathBuf,
    pub db: Db,
}

impl Store {
    /// Try to open a store located at `base`.  If the directory
    /// doesn't exist, it will be created.
    pub fn open(base: &Path) -> Result<Store> {
        if !base.exists() {
            fs::create_dir(base)?;
        } else if !base.is_dir() {
            bail!("Base {:?} is not a directory", base);
        }
        let db = Db::open(base)?;
        Ok(Store {
            db,
            base: base.to_owned(),
        })
    }

    /// Add a new object into this store.
    pub fn add(&mut self, path: &Path) -> Result<ObjectId> {
        let obj_id = self.generate_fresh_id();
        let obj_path = path.to_owned();
        self.db.new_object(&obj_id, &obj_path);
        self.db.flush();
        Ok(obj_id)
    }

    /// Delete an object from this store.
    pub fn delete(&mut self, id: ObjectId) -> Result<()> {
        let obj_path = self.db.get_object_path(&id);
    }
}

impl Store {
    /// Generate a new, unused unique ID.
    fn generate_fresh_id(&self) -> ObjectId {
        ObjectId("a".to_string())
    }
}

/// An object ID is a UTF-8-encoded string, usually reasonably short.
#[derive(Debug, Clone, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct ObjectId(pub String);

impl From<String> for ObjectId {
    fn from(s: String) -> ObjectId {
        ObjectId(s)
    }
}

impl From<ObjectId> for String {
    fn from(ObjectId(s): ObjectId) -> String {
        s
    }
}

impl ObjectId {
    /// Create a new object ID from its string representation.
    pub fn from_str(s: &str) -> ObjectId {
        ObjectId(s.to_string())
    }
}
