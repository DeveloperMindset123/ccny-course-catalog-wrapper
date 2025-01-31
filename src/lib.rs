//! Course Management Library
//! 
//! This library provides functionality for efficient course data management and retrieval.
//! It implements optimized data structures for storing and accessing course information
//! from the Coursedog API.
//!
//! # Features
//! - Optimized course data storage using custom data structures
//! - Efficient search and retrieval operations
//! - API integration with Coursedog
//! - String matching for course and department names
//!
//! # Examples
//! ```rust
//! use course_lib::{CourseStorage, CourseInfo};
//! 
//! #[tokio::main]
//! async fn main() {
//!     // Initialize storage
//!     let mut storage = CourseStorage::new(1000);
//!     
//!     // Fetch and store courses
//!     let courses = fetch_courses_by_department("computer science").await.unwrap();
//!     for course in courses {
//!         storage.insert(course);
//!     }
//!     
//!     // Retrieve course by code
//!     let course = storage.get_by_code("CSC 10300").unwrap();
//! }
//! ```
mod models;
mod storage;
mod api;
mod utils;

pub use models::{CourseInfo, CourseComponents};
pub use storage::CourseStorage;
pub use api::{fetch_courses_by_department};         // fetch_all_courses isn't being used
pub use utils::StringInterner;

pub fn add(x : i32, y : i32) -> i32 {
    x + y
}