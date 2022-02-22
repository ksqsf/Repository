//! Repository is a personal object storage with the following design
//! goals:
//!
//! - **Open**: designed to work with other applications.
//! - **Extensible**: an object can have arbitrary attributes.
//! - **Versatile**: comes with an full-featured query language.
//! - **Scalable** up to millions of files.
//! - **Safety**: set up replication easily.

pub mod cli;
pub mod storage;
pub mod db;
pub mod query;
