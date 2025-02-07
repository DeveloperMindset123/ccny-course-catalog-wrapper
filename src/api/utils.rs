// stores utility functions that handles the saving and retrival logic of files
use crate::api::department::{fetch_courses_by_department};
use crate::api::course_finder;
use crate::models::CourseInfo;
use std::path::PathBuf;
use std::collections::HashMap;
use std::{fs, io::Write};
use anyhow::Result;

// function that isolates and returns list of courses relevant to a particular department
pub async fn get_course_list_by_department(department_name : &str) -> Result<Vec<String>, anyhow::Error> {
    let course_list : Vec<String> = Vec::new();
    // call on the function to retrieve the course
    let department_courses : Vec<CourseInfo> = fetch_courses_by_department(department_name).await?;

    Ok(course_list)
}

// function to print the type
pub fn print_type_of<T>(_ : &T) {
    println!("{}", std::any::type_name::<T>());
}

// Helper function to save response to file
pub fn save_to_file(data: &serde_json::Value, filename: &str) -> Result<PathBuf> {
    let file = fs::File::create(filename)?;

    // to_writer_pretty() : serialize the given data strucuture as a pretty-printed JSON into the I/O stream.
    serde_json::to_writer_pretty(file, data)?;
    Ok(PathBuf::from(filename))
}

// prints out the keys and values of an hashmap
pub fn print_hashmap_keys(hashmap_input : HashMap<String, String>) {
    for (key,value) in hashmap_input.into_iter() {
        println!("current department : {key:?}");
    }
}