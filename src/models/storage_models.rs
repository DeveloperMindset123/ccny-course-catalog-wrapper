//! Optimized internal storage models

/// Compact representation of course components
/// This component is part of CompactCourseInfo struct
#[derive(Clone,Copy)]
pub struct CompactCourseComponents {
    // pack multiple fields into a single u32
    pub data : u32,     // contains course_type, attendance_type, instruction_mode as bit flags
    pub weekly_hours : u8,      // most courses are under 255 hours, cannot be negative
    pub class_size : u16,        // supports upt to 65535 students
    pub exam_spacing : u8       // small values for spacing
}

/// Compact course information for storage
#[derive(Clone)]
pub struct CompactCourseInfo {
    pub unique_id : String,     // maintained as string due to the alphanumerical values
    pub name_id : u32,          // reference to interned string
    pub course_code : u32,      // packed department and number
    pub components : CompactCourseComponents,       // inherited from CompactCourseComponents above
    pub start_date : u32,        // packed date
    pub group_id : u32,
    pub department_id : u32,    // interned department 
    pub credits : u8
}

