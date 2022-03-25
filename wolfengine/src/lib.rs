#![allow(clippy::missing_errors_doc)]
#![allow(unused_crate_dependencies)]

use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

//pub mod algorithm;
pub mod chrono;
pub mod compression;
pub mod db;
pub mod net;
pub mod os;
//pub mod script;