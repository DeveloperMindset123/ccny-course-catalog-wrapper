// TODO : write documentations based examples for usage
mod models;
mod data_structures;
mod api;
mod ccny_course_catalog;
pub use data_structures::{custom_hashmap, custom_lru_cache};
pub use crate::ccny_course_catalog::CCNYCourseCatalog;
// we use this line if pub use self::course::{CourseInfo, CourseComponents}; 
// is not included within mod.rs
pub use models::course::{CourseComponents, CourseInfo};

// pub fn add(x : i32, y : i32) -> i32 {
//     x + y
// }