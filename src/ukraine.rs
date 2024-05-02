#[allow(unused)]
use derive_deref::Deref;
use getset::{Getters, Setters};
use ralertsinua_models::*;

pub use std::sync::{Arc, RwLock};

// use geo::algorithm::bounding_rect::BoundingRect;
// use geo::algorithm::simplify_vw::SimplifyVw;

#[derive(Debug, Default, Getters, Setters)]
pub struct Ukraine {
    #[getset(get = "pub")]
    regions: Box<[Region]>,
}

impl Ukraine {
    pub fn new_arc() -> Arc<RwLock<Ukraine>> {
        Arc::new(RwLock::new(Ukraine::default()))
    }

    pub fn new(regions: Box<[Region]>) -> Self {
        Self { regions }
    }
}
