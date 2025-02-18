use ccny_course_catalog::CCNYCourseCatalog;
use tokio;
use anyhow::Result;

// #[tokio::main]
// async fn main() -> Result<()> {
//     let mut course_instance = CCNYCourseCatalog::new(String::from("computer science"), Some(String::from("computer sec")));

//     println!("{:#?}", course_instance.clone());
//     // tested : worked
//     // println!("{:#?}",course_instance.get_courses_list().await);

//     // tested : worked
//     // println!("{:#?}", course_instance.get_course_info().await);

//     // test setter methods
//     course_instance.set_department_name("electrical engineering");
//     course_instance.set_course_name("statistics");
//     println!("{:#?}", course_instance.clone());

//     Ok(())

// }

#[tokio::main]
async fn main() -> Result<()> {
    let mut course_instance = CCNYCourseCatalog::new(String::from("computer science"), Some(String::from("algorithms")));

    // return tyep of list_of_courses
    // Result<Vec<CourseInfo>, anyhow::Error>
    let mut list_of_courses = course_instance.get_courses_list().await;
    let mut course_info = course_instance.get_course_info().await;
    // println!("{course_info:#?}");

    let department_list = CCNYCourseCatalog::get_department_list();
    println!("{department_list:#?}");
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