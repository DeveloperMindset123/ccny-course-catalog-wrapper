// This is the file that will handle the implementation logic
// This will be the user facing file
use anyhow::Result;
use crate::api::department::fetch_courses_by_department;
use crate::models::CourseInfo;
use crate::api::course_finder::retrieve_specific_course_info;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CCNYCourseCatalog {
        pub department_name : String,
        pub course_name : String
    }

impl CCNYCourseCatalog {

    // constructor definition
    pub fn new(department_name : String, course_name : Option<String>) ->   CCNYCourseCatalog {
        CCNYCourseCatalog {
            department_name : department_name,
            course_name : course_name.unwrap_or(String::from(""))
        }
    }

    // retrieves list of courses 
    // based on name of department
    pub async fn get_courses_list(&self) -> Result<Vec<CourseInfo>, anyhow::Error> {
        fetch_courses_by_department(&self.department_name).await
    }

    pub async fn get_course_info(&self) -> Result<serde_json::Value, anyhow::Error> {
        if self.course_name == "".to_owned() {
            eprintln!("course name is empty, please specify a specific course name to retrieve information about a particular course.\n\n
                
            HINT : use the method .set_course_name('name of course here')");
            return Err(anyhow::Error::msg("method failed to execute."));
        }

        // otherwise, if course_name does exist
        // call on the function
        // retrieve_specific_course_info
        retrieve_specific_course_info(&self.course_name, &self.department_name).await
    }

    pub fn set_department_name(&mut self, new_department_name : &str) {
        self.department_name = String::from(new_department_name);
    }

    pub fn set_course_name(&mut self, new_course_name : &str) {
        self.course_name = String::from(new_course_name);
    }
}
