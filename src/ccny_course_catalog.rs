// This is the file that will handle the implementation logic
// This will be the user facing file
use anyhow::Result;
use crate::api::department::{self, fetch_courses_by_department};
use crate::models::CourseInfo;
use crate::api::course_finder::retrieve_specific_course_info;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CCNYCourseCatalog {
        pub department_name : String,       
        pub course_name : String
    }

impl CCNYCourseCatalog {

    /// # constructor function
    /// - Example Usage
    /// ```rust
    /// use ccny_course_catalog::CCNYCourseCatalog;
    /// use tokio;
    /// use anyhow::Result;
    /// 
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let mut course_instance = CCNYCourseCatalog::new(String::from("computer science"), Some(String::from("data structures")));
    ///     Ok(())
    /// }
    /// ```
    pub fn new(department_name : String, course_name : Option<String>) ->   CCNYCourseCatalog {
        CCNYCourseCatalog {
            department_name : department_name,
            course_name : course_name.unwrap_or(String::from(""))
        }
    }

    /// # Description
    /// - Retrieves list of courses 
    /// - Based on name of department
    /// 
    /// Example Usage:
    /// /// # constructor function
    /// - Example Usage
    /// ```rust
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let mut course_instance = CCNYCourseCatalog::new(String::from("computer science"), Some(String::from("data structures")));

    ///     // return type of list_of_courses
    ///     // Result<Vec<CourseInfo>, anyhow::Error>
    ///     let mut list_of_courses = course_instance.get_courses_list().await;
    ///     println!("{list_of_courses:#?}");
    ///     Ok(())
    /// 
    /// // Sample output:
    /// // Result(
    /// //     [
    /// //         CourseInfo {
    /// //             unique_id: "0455781-1901-01-01",
    /// //             course_name: "Systems Simulation",
    /// //             career: "Undergraduate",
    /// //             course_code: "CSC 44200",
    /// //             course_components: CourseComponents {
    /// //             course_type: "LEC",
    /// //             attendance_type: "Class Meeting",
    /// //             weekly_hours: 3,
    /// //             class_size: 35,
    /// //             final_exam: "Yes",
    /// //             exam_seat_spacing: 1,
    /// //             instruction_mode: "In Person",
    /// //         },
    /// //         effective_start_date: "1901-01-01",
    /// //         effective_end_date: "unknown",
    /// //         course_group_id: 455781,
    /// //         course_number: 44200,
    /// //         department: [
    /// //             "CSC-CTY",
    /// //         ],
    /// //         subject_code: "CSC",
    /// //         credits: 3,
    /// //     },
    /// //        
    /// //        
    /// //         CourseInfo {
    /// //             unique_id: "1366591-2021-03-15",
    /// //             course_name: "Topics in Software Engineering",
    /// //             career: "Undergraduate",
    /// //             course_code: "CSC 45600",
    /// //             course_components: CourseComponents {
    /// //                 course_type: "LEC",
    /// //                 attendance_type: "Class Meeting",
    /// //                 weekly_hours: 3,
    /// //                 class_size: 25,
    /// //                 final_exam: "Yes",
    /// //                 exam_seat_spacing: 1,
    /// //                 instruction_mode: "In Person",
    /// //             },
    /// //             effective_start_date: "2021-03-15",
    /// //             effective_end_date: "unknown",
    /// //             course_group_id: 1366591,
    /// //             course_number: 45600,
    /// //             department: [
    /// //                 "CSC-CTY",
    /// //             ],
    /// //             subject_code: "CSC",
    /// //             credits: 3,
    /// //         },
    /// //         CourseInfo {
    /// //             unique_id: "1267861-2021-03-15",
    /// //             course_name: "Visualization",
    /// //             career: "Undergraduate",
    /// //             course_code: "CSC 47400",
    /// //             course_components: CourseComponents {
    /// //                 course_type: "LEC",
    /// //                 attendance_type: "Class Meeting",
    /// //                 weekly_hours: 3,
    /// //                 class_size: 30,
    /// //                 final_exam: "Yes",
    /// //                 exam_seat_spacing: 1,
    /// //                 instruction_mode: "In Person",
    /// //             },
    /// //             effective_start_date: "2021-03-15",
    /// //             effective_end_date: "unknown",
    /// //             course_group_id: 1267861,
    /// //             course_number: 47400,
    /// //             department: [
    /// //                 "CSC-CTY",
    /// //             ],
    /// //             subject_code: "CSC",
    /// //             credits: 3,
    /// //         },
    /// //         
    /// //         CourseInfo {
    /// //             unique_id: "0455871-1901-01-01",
    /// //             course_name: "Web Site Design",
    /// //             career: "Undergraduate",
    /// //             course_code: "CSC 47300",
    /// //             course_components: CourseComponents {
    /// //                 course_type: "LEC",
    /// //                 attendance_type: "Class Meeting",
    /// //                 weekly_hours: 3,
    /// //                 class_size: 35,
    /// //                 final_exam: "Yes",
    /// //                 exam_seat_spacing: 1,
    /// //                 instruction_mode: "In Person",
    /// //             },
    /// //             effective_start_date: "1901-01-01",
    /// //             effective_end_date: "unknown",
    /// //             course_group_id: 455871,
    /// //             course_number: 47300,
    /// //             department: [
    /// //                 "CSC-CTY",
    /// //             ],
    /// //             subject_code: "CSC",
    /// //             credits: 3,
    /// //         },
    /// //         ...additional courses list continued
    /// //     ]
    /// // )
 
    /// }
    /// ```
    pub async fn get_courses_list(&self) -> Result<Vec<CourseInfo>, anyhow::Error> {
        fetch_courses_by_department(&self.department_name).await
    }

    /// Retrieves information about the current course.
    /// ```rust
    /// #[tokio::main]
/// async fn main() -> Result<()> {
///     let mut course_instance = CCNYCourseCatalog::new(String::from("computer science"), Some(String::from("data structures")));

///     // return tyep of list_of_courses
///     // Result<Vec<CourseInfo>, anyhow::Error>
///     let mut list_of_courses = course_instance.get_courses_list().await;
///     let mut course_info = course_instance.get_course_info().await;
///     println!("{course_info:#?}");
///     Ok(())
/// 
/// // Resulting Output:
/// //     Ok(
/// //     Object {
/// //         "data": Array [
/// //             Object {
/// //                 "_id": String("0455351-1901-01-01"),
/// //                 "career": String("Undergraduate"),
/// //                 "code": String("CSC 21200"),
/// //                 "college": String("ENGR - Grove School of Engineering"),
/// //                 "components": Array [
/// //                     Object {
/// //                         "attendanceContactUse": Bool(true),
/// //                         "attendanceGenerate": Bool(false),
/// //                         "attendanceLeftUse": Bool(true),
/// //                         "attendancePresentUse": Bool(true),
/// //                         "attendanceReasonUse": Bool(true),
/// //                         "attendanceTardyUse": Bool(true),
/// //                         "attendanceTemplateOverride": Bool(false),
/// //                         "attendanceTimeUse": Bool(true),
/// //                         "attendanceType": String("Class Meeting"),
/// //                         "autoCreate": Bool(false),
/// //                         "code": String("LEC"),
/// //                         "contactHours": Number(4),
/// //                         "customFields": Object {},
/// //                         "defaultSectionSize": Number(35),
/// //                         "deliveryMode": Array [],
/// //                         "examSeatSpacing": Number(1),
/// //                         "finalExamType": String("Yes"),
/// //                         "id": String("LEC"),
/// //                         "includeInDynamicDate": Bool(false),
/// //                         "instructionMode": String("In Person"),
/// //                         "lmsFileType": String("Blackboard CourseInfo 4"),
/// //                         "name": String("Lecture"),
/// //                         "oeeWorkloadHours": Number(0),
/// //                         "optionalComponent": Bool(false),
/// //                         "preferredRoomFeatures": Array [
/// //                             String("Academic Scheduling"),
/// //                         ],
/// //                         "providerForAuthentication": String(""),
/// //                         "workloadHours": Number(4),
/// //                     },
/// //                 ],
/// //                 "consent": String("No Special Consent Required"),
/// //                 "courseEquivalencies": Array [],
/// //                 "courseGroupId": String("0455351"),
/// //                 "courseNumber": String("21200"),
/// //                 "courseTypicallyOffered": String("Fall, Spring"),
/// //                 "credits": Object {
/// //                     "academicProgressHours": Object {
/// //                         "operator": String(""),
/// //                         "value": Number(3),
/// //                     },
/// //                     "contactHours": Object {
/// //                         "operator": String(""),
/// //                         "value": Number(4),
/// //                     },
/// //                     "courseCount": Number(1),
/// //                     "creditHours": Object {
/// //                         "max": Number(3),
/// //                         "min": Number(3),
/// //                         "operator": String(""),
/// //                     },
/// //                     "financialAidHours": Object {
/// //                         "operator": String(""),
/// //                         "value": Number(3),
/// //                     },
/// //                     "numberOfCredits": Number(3),
/// //                     "numberOfRepeats": Number(1),
/// //                     "repeatable": Bool(false),
/// //                 },
/// //                 "customFields": Object {
/// //                     "catalogAttributes": Array [],
/// //                     "cuLibartsFlag": Bool(false),
/// //                     "rawCourseId": String("045535"),
/// //                 },
/// //                 "departmentOwnership": Array [],
/// //                 "departments": Array [
/// //                     Object {
/// //                         "_id": String("CSC-CTY"),
/// //                         "campus": String("MAIN"),
/// //                         "chair": Array [],
/// //                         "displayName": String("Computer Science"),
/// //                         "effectiveEndDate": String(""),
/// //                         "effectiveStartDate": String(""),
/// //                         "id": String("CSC-CTY"),
/// //                         "institution": String("CTY01"),
/// //                         "lastSyncErrorRecommendations": Array [],
/// //                         "lastSyncErrors": Array [],
/// //                         "lastSyncMergeReportId": String("gztrPMIGtByV2OgAAo2a"),
/// //                         "lastSyncStatus": String("success"),
/// //                         "lastSyncedAt": Number(1738733625029),
/// //                         "name": String("Computer Science"),
/// //                         "scheduleStatus": Object {
/// //                             "2023": Object {},
/// //                         },
/// //                         "status": String("Active"),
/// //                     },
/// //                 ],
/// //                 "description": String("Extension of the knowledge of algorithm design and programming gained in CSC 10300 with continued emphasis on the logic underlying the transition from specification to program. Particular attention is paid to issues arising in the implementation of larger programs: introduction of data structures and data abstraction; the basics of object-oriented programming. Introduction of recursion as a design tool. Introduction of complexity analysis"),
/// //                 "effectiveEndDate": Null,
/// //                 "effectiveStartDate": String("1901-01-01"),
/// //                 "id": String("0455351-1901-01-01"),
/// //                 "institution": String("CTY01"),
/// //                 "institutionId": String("045535"),
/// //                 "longName": String("Data Structures"),
/// //                 "name": String("Data Structures"),
/// //                 "orderByKeyForCode": String("CSC0000021200"),
/// //                 "requirementGroup": String("019393"),
/// //                 "status": String("Active"),
/// //                 "subjectCode": String("CSC"),
/// //                 "topics": Array [],
/// //             },
/// //         ],
/// //         "limit": Number(50),
/// //         "listLength": Number(1),
/// //         "skip": Number(0),
/// //     },
/// // )
 
/// }
/// ```
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
    
    /// Returns list of departments available within CUNY City College of New York.
    /// ```rust
    /// // example usage
    /// // this code snippet should be within a function
    /// let department_list = CCNYCourseCatalog::get_department_list();
    /// ```
    pub fn get_department_list() -> Vec<String> {
        department::get_department_list()
    }

    /// Setter methods allows modification of department and courses.
    /// Setter logic examples
    /// ```
    /// course_instance.set_department_name("electrical engineering");
    /// course_instance.set_course_name("statistics");
    /// ```

    pub fn set_department_name(&mut self, new_department_name : &str) {
        self.department_name = String::from(new_department_name);
    }

    pub fn set_course_name(&mut self, new_course_name : &str) {
        self.course_name = String::from(new_course_name);
    }
}