// all relevant data structures is stored here

pub mod course;

// brings all the relative functions and structs within scope
// so we don't have to specify course::CourseInfo or storage_modes::CompactCourseInfo everytime
// reducing the boilerplate code that needs to be written
pub use self::course::{CourseInfo, CourseComponents};       
