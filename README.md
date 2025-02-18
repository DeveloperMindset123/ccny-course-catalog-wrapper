<!--https://docs.rs/ccny-course-catalog/0.1.1/ccny_course_catalog/-->
# ccny-course-catalog
## Overview
A library that fetches information related to list of courses and particular course information from CUNY City College:
- **Retrieve List of Courses** : Get information about list of courses that are offered given a department name available within CCNY.

- **Retrieve Course Information** : Retrieve information about a particular course provided as part of constructor parameter, this will retrieve data in regards to seat sizing, instruction mode, class sizes, attendance requirement, semesters during which the class is offered, etc.

[API Documentation](https://docs.rs/ccny-course-catalog/0.1.4/ccny_course_catalog/) : To get better understanding of implementation details.

#### Installation:
- While not all of the dependancies are needed for the methods module, if you do wish to use the features module, some additional dependancies such as serde and reqwest will be needed.
- The list below covers all the dependancies needed.
```toml
[dependancies]
ccny-course-catalog = "0.1.1"
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.12.12", features = ["json", "gzip", "deflate", "stream", "blocking"] }
anyhow = "1.0.95"
tokio-macros = "2.5.0"
serde = { version = "1.0.217", features = ["derive"] }
serde_json = "1.0.135"
filepath="0.2.0"
async-compression={version="0.4.18", features=["futures-io"] }
closestMatch="=0.1.2"
lru="0.13.0"
```

## Code Examples
### Constructor Example:
```rust
use ccny_course_catalog::CCNYCourseCatalog;
use tokio;
use anyhow::Result;
     
#[tokio::main]
async fn main() -> Result<()> {
    let mut course_instance = CCNYCourseCatalog::new(String::from("computer science"), Some(String::from("data structures")));
    Ok(())
}
```

### Example to retrieve list of courses:
```rust
#[tokio::main]
async fn main() -> Result<()> {
    let mut course_instance = CCNYCourseCatalog::new(String::from("computer science"), Some(String::from("data structures")));
    // return type of list_of_courses
    // Result<Vec<CourseInfo>, anyhow::Error>
    let mut list_of_courses = course_instance.get_courses_list().await;
    println!("{list_of_courses:#?}");
    Ok(())
 
// Sample output:
// Result(
//     [
//         CourseInfo {
//             unique_id: "0455781-1901-01-01",
//             course_name: "Systems Simulation",
//             career: "Undergraduate",
//             course_code: "CSC 44200",
//             course_components: CourseComponents {
//             course_type: "LEC",
//             attendance_type: "Class Meeting",
//             weekly_hours: 3,
//             class_size: 35,
//             final_exam: "Yes",
//             exam_seat_spacing: 1,
//             instruction_mode: "In Person",
//         },
//         effective_start_date: "1901-01-01",
//         effective_end_date: "unknown",
//         course_group_id: 455781,
//         course_number: 44200,
//         department: [
//             "CSC-CTY",
//         ],
//         subject_code: "CSC",
//         credits: 3,
//     },
//        
//        
//         CourseInfo {
//             unique_id: "1366591-2021-03-15",
//             course_name: "Topics in Software Engineering",
//             career: "Undergraduate",
//             course_code: "CSC 45600",
//             course_components: CourseComponents {
//                 course_type: "LEC",
//                 attendance_type: "Class Meeting",
//                 weekly_hours: 3,
//                 class_size: 25,
//                 final_exam: "Yes",
//                 exam_seat_spacing: 1,
//                 instruction_mode: "In Person",
//             },
//             effective_start_date: "2021-03-15",
//             effective_end_date: "unknown",
//             course_group_id: 1366591,
//             course_number: 45600,
//             department: [
//                 "CSC-CTY",
//             ],
//             subject_code: "CSC",
//             credits: 3,
//         },
//         CourseInfo {
//             unique_id: "1267861-2021-03-15",
//             course_name: "Visualization",
//             career: "Undergraduate",
//             course_code: "CSC 47400",
//             course_components: CourseComponents {
//                 course_type: "LEC",
//                 attendance_type: "Class Meeting",
//                 weekly_hours: 3,
//                 class_size: 30,
//                 final_exam: "Yes",
//                 exam_seat_spacing: 1,
//                 instruction_mode: "In Person",
//             },
//             effective_start_date: "2021-03-15",
//             effective_end_date: "unknown",
//             course_group_id: 1267861,
//             course_number: 47400,
//             department: [
//                 "CSC-CTY",
//             ],
//             subject_code: "CSC",
//             credits: 3,
//         },
//         
//         CourseInfo {
//             unique_id: "0455871-1901-01-01",
//             course_name: "Web Site Design",
//             career: "Undergraduate",
//             course_code: "CSC 47300",
//             course_components: CourseComponents {
//                 course_type: "LEC",
//                 attendance_type: "Class Meeting",
//                 weekly_hours: 3,
//                 class_size: 35,
//                 final_exam: "Yes",
//                 exam_seat_spacing: 1,
//                 instruction_mode: "In Person",
//             },
//             effective_start_date: "1901-01-01",
//             effective_end_date: "unknown",
//             course_group_id: 455871,
//             course_number: 47300,
//             department: [
//                 "CSC-CTY",
//             ],
//             subject_code: "CSC",
//             credits: 3,
//         },
//         ...additional courses list continued
//     ]
// )
}

```

### Example to retrieve information about particular course:
```rust
#[tokio::main]
async fn main() -> Result<()> {
    let mut course_instance = CCNYCourseCatalog::new(String::from("computer science"), Some(String::from("data structures")));
    // return tyep of list_of_courses
    // Result<Vec<CourseInfo>, anyhow::Error>
    let mut list_of_courses = course_instance.get_courses_list().await;
    let mut course_info = course_instance.get_course_info().await;
    println!("{course_info:#?}");
    Ok(())
 
// Resulting Output:
//     Ok(
//     Object {
//         "data": Array [
//             Object {
//                 "_id": String("0455351-1901-01-01"),
//                 "career": String("Undergraduate"),
//                 "code": String("CSC 21200"),
//                 "college": String("ENGR - Grove School of Engineering"),
//                 "components": Array [
//                     Object {
//                         "attendanceContactUse": Bool(true),
//                         "attendanceGenerate": Bool(false),
//                         "attendanceLeftUse": Bool(true),
//                         "attendancePresentUse": Bool(true),
//                         "attendanceReasonUse": Bool(true),
//                         "attendanceTardyUse": Bool(true),
//                         "attendanceTemplateOverride": Bool(false),
//                         "attendanceTimeUse": Bool(true),
//                         "attendanceType": String("Class Meeting"),
//                         "autoCreate": Bool(false),
//                         "code": String("LEC"),
//                         "contactHours": Number(4),
//                         "customFields": Object {},
//                         "defaultSectionSize": Number(35),
//                         "deliveryMode": Array [],
//                         "examSeatSpacing": Number(1),
//                         "finalExamType": String("Yes"),
//                         "id": String("LEC"),
//                         "includeInDynamicDate": Bool(false),
//                         "instructionMode": String("In Person"),
//                         "lmsFileType": String("Blackboard CourseInfo 4"),
//                         "name": String("Lecture"),
//                         "oeeWorkloadHours": Number(0),
//                         "optionalComponent": Bool(false),
//                         "preferredRoomFeatures": Array [
//                             String("Academic Scheduling"),
//                         ],
//                         "providerForAuthentication": String(""),
//                         "workloadHours": Number(4),
//                     },
//                 ],
//                 "consent": String("No Special Consent Required"),
//                 "courseEquivalencies": Array [],
//                 "courseGroupId": String("0455351"),
//                 "courseNumber": String("21200"),
//                 "courseTypicallyOffered": String("Fall, Spring"),
//                 "credits": Object {
//                     "academicProgressHours": Object {
//                         "operator": String(""),
//                         "value": Number(3),
//                     },
//                     "contactHours": Object {
//                         "operator": String(""),
//                         "value": Number(4),
//                     },
//                     "courseCount": Number(1),
//                     "creditHours": Object {
//                         "max": Number(3),
//                         "min": Number(3),
//                         "operator": String(""),
//                     },
//                     "financialAidHours": Object {
//                         "operator": String(""),
//                         "value": Number(3),
//                     },
//                     "numberOfCredits": Number(3),
//                     "numberOfRepeats": Number(1),
//                     "repeatable": Bool(false),
//                 },
//                 "customFields": Object {
//                     "catalogAttributes": Array [],
//                     "cuLibartsFlag": Bool(false),
//                     "rawCourseId": String("045535"),
//                 },
//                 "departmentOwnership": Array [],
//                 "departments": Array [
//                     Object {
//                         "_id": String("CSC-CTY"),
//                         "campus": String("MAIN"),
//                         "chair": Array [],
//                         "displayName": String("Computer Science"),
//                         "effectiveEndDate": String(""),
//                         "effectiveStartDate": String(""),
//                         "id": String("CSC-CTY"),
//                         "institution": String("CTY01"),
//                         "lastSyncErrorRecommendations": Array [],
//                         "lastSyncErrors": Array [],
//                         "lastSyncMergeReportId": String("gztrPMIGtByV2OgAAo2a"),
//                         "lastSyncStatus": String("success"),
//                         "lastSyncedAt": Number(1738733625029),
//                         "name": String("Computer Science"),
//                         "scheduleStatus": Object {
//                             "2023": Object {},
//                         },
//                         "status": String("Active"),
//                     },
//                 ],
//                 "description": String("Extension of the knowledge of algorithm design and programming gained in CSC 10300 with continued emphasis on the logic underlying the transition from specification to program. Particular attention is paid to issues arising in the implementation of larger programs: introduction of data structures and data abstraction; the basics of object-oriented programming. Introduction of recursion as a design tool. Introduction of complexity analysis"),
//                 "effectiveEndDate": Null,
//                 "effectiveStartDate": String("1901-01-01"),
//                 "id": String("0455351-1901-01-01"),
//                 "institution": String("CTY01"),
//                 "institutionId": String("045535"),
//                 "longName": String("Data Structures"),
//                 "name": String("Data Structures"),
//                 "orderByKeyForCode": String("CSC0000021200"),
//                 "requirementGroup": String("019393"),
//                 "status": String("Active"),
//                 "subjectCode": String("CSC"),
//                 "topics": Array [],
//             },
//         ],
//         "limit": Number(50),
//         "listLength": Number(1),
//         "skip": Number(0),
//     },
// )
}
```

### Available setter methods:
```rust
course_instance.set_department_name("electrical engineering");
course_instance.set_course_name("statistics");
```

### Getting Help
- If the [API Documentation](https://docs.rs/ccny-course-catalog/0.1.4/ccny_course_catalog/) doesn't help and you happen to be stuck on something, there's also examples within the **bin** folder containing executable code.

- However, if that doesn't work or solve your problem, or if there's a feature request you would like to make, you can reach out to me in the following:

**Gmail**: dasa60196@gmail.com

**Discord** : the1sand0s (Just send a friend request and message me)
### License
This project is licensed under the MIT License.