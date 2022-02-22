//! # Metadata database

use std::{path::{Path, PathBuf}, collections::HashMap};
use anyhow::Result;
use crate::storage::ObjectId;

pub struct Db {
    object_path_map: HashMap<ObjectId, PathBuf>,
}

impl Db {
    pub fn open(path: &Path) -> Result<Db> {
        todo!()
    }

    pub fn new_object(&mut self, id: &ObjectId, path: &Path) -> Result<()> {
        todo!()
    }

    pub fn get_object_path(&self, id: &ObjectId) -> Option<PathBuf> {
        self.object_path_map.get(id).cloned()
    }

    pub fn flush(&mut self) -> Result<()> {
        todo!()
    }
}
