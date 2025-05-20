#![feature(
    // Language
    decl_macro,
    never_type,
    unboxed_closures,
    // Standard Library,
    fn_traits,
    tuple_trait,
    // Documentation
    doc_cfg
)]


mod internal;
pub use internal::App;

pub mod game;

pub mod time;
pub mod rand;
pub use uuid;

#[cfg(any(doc, feature = "selfhosted"))]
mod selfhosted;


pub mod prelude {
    pub use super::internal::App;

    pub use super::game::prelude::*;

    pub use super::time::{ Duration, DurationExt, Instant };
    pub use super::rand::{ self, GetRandom };
    pub use super::uuid::Uuid;

    #[cfg(any(doc, feature = "selfhosted"))]
    #[doc(cfg(feature = "selfhosted"))]
    pub use super::selfhosted::{
        trace, debug, info, pass,
        warn, error, fatal
    };
}
