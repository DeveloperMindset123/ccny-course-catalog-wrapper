//! Course search functionality module
//! 
//! This module handles all course search related operations including:
//! - Course lookup by name
//! - Department matching
//! - String similarity matching
//! 
//! # Examples
//! ```rust
//! use course_lib::search::find_course_by_name_and_dept;
//! 
//! #[tokio::main]
//! async fn main() {
//!     let course = find_course_by_name_and_dept("data structures", "computer science").await?;
//!     println!("Found course: {}", course.course_name);
//! }
//! ```

pub mod matcher;
pub use self::matcher::StringMatcher;
pub use self::course_finder::find_course_by_name_and_dept;

