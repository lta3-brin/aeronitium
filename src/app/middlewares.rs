use actix_web::middleware::{normalize, NormalizePath};


pub fn normalize_path() -> NormalizePath {
    normalize::NormalizePath::default()
}
