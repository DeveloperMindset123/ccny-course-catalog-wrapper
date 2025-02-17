// Stores all relevant structs here

use serde::{Deserialize, Serialize};

// redefining CourseComponents here, could throw potential error
/// Represents the components of a course such as lecture, lab, etc.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CourseComponents {
    pub course_type : String,
    pub attendance_type : String,
    pub weekly_hours : i32,
    pub class_size : i32,
    pub final_exam : String,
    pub exam_seat_spacing : i32,
    pub instruction_mode : String
}

/// Main course information structure
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CourseInfo {
    pub unique_id : String,
    pub course_name : String,
    pub career : String,
    pub course_code : String,
    pub course_components : CourseComponents,
    pub effective_start_date : String,
    pub effective_end_date : String,
    pub course_group_id : String,       // originally i32
    pub course_number : i64,
    pub department : Vec<String>,
    pub subject_code : String,
    pub credits : String
}