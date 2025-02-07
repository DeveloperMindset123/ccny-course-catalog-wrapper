// TODO : write documentations based examples for usage
// NOTE : as long as they are not declared as "pub"
// they will not be exposed in the documentation

// NOTE : exposing something as pub within non lib.rs file
// will help with importing it internally
// declaring a module as pub within lib.rs will expose it to the documentation
mod models;
mod data_structures;
mod api;
mod ccny_course_catalog;
// pub use data_structures::{custom_hashmap, custom_lru_cache};

/// All relevant implementation can be found here.
pub use crate::ccny_course_catalog::CCNYCourseCatalog;
// we use this line if pub use self::course::{CourseInfo, CourseComponents}; 
// is not included within mod.rs
use models::course::{CourseComponents, CourseInfo};