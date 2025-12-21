pub mod decl;
use std::path::{self, Path};

static mut PROJECT: Option<Project> = None;

pub struct Project {}

pub async fn load(path: impl AsRef<Path>) {
    let path = path.as_ref();
}
