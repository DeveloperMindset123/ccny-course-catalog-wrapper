// all relevant data structures is stored here

pub mod course;
pub mod storage_models;

// brings all the relative functions and structs within scope
// so we don't have to specify course::CourseInfo or storage_modes::CompactCourseInfo everytime
// reducing the boilerplate code that needs to be written
pub use self::course::{CourseInfo, CourseComponents};       
pub use self::storage_models::{CompactCourseInfo, CompactCourseComponents};
