// NOTE : upon retrieving the list of courses, additional information about the course can be retrieved using the courseGroupId

// GET Method --> https://app.coursedog.com/api/v1/cty01/general/terms : general api to retrieve information regarding all the terms

use anyhow::Result;
use reqwest;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, CONTENT_TYPE, ORIGIN, REFERER};
use serde_json::json;
use std::{fs, io::Write};
use std::path::PathBuf;
use std::collections::HashMap;
use std::borrow::Borrow;

// This struct is inherited within CourseInfo struct
pub struct CourseComponents {
    pub course_type : String,       // (i.e. "LEC")
    pub weekly_hours : i32,         // (i.e. 3 hour per week)
    pub class_size : i32,
    pub final_exam : String,
    pub attendance_type : String,
    pub exam_seat_spacing : i32   
}

pub struct CourseInfo {
    pub unique_id : String,
    pub course_name : String,       // course LongName
    pub career : String,
    pub course_code : String,
    pub course_components : CourseComponents,
    pub effective_start_date : String,          // indicates the time this course was released
    pub effective_end_date : Option<String>,        // indicates the time the course may end (could be null)
    pub course_group_id : i32,
    pub course_number : i32,
    pub department : Vec<String>,
    pub subject_code : String,
    pub credits : i32,          // (i.e. 1,3,4)
}
/**
 * Header data that has been removed:
 * "Accept-Encoding"
 * "Priority"
 * "Pragma"
 * "Sec-Fetch-Dest"
 * "Sec-Fetch-Mode"
 * "Sec-Fetch-Site"
 * 
 * Header data that has been added:
 * REFERER (most likely what caused the issue with the data retrieval originally)
 */

 // something important to note
 // headers aren't really needed, they will work just fine even with an empty header
 // what's important is the payload and query params that are being passed in
 // the get_headers() function is more so for organization purpose, doesn't have any impact on the actual API logic
pub fn get_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, HeaderValue::from_static("*/*"));
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert("User-Agent", HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/132.0.0.0 Safari/537.36"));
    headers.insert("Priority", HeaderValue::from_static("u=1, i"));
    headers.insert("Pragma", HeaderValue::from_static("no-cache"));
    headers.insert(ORIGIN, HeaderValue::from_static("https://ccny-undergraduate.catalog.cuny.edu"));
    headers.insert(REFERER, HeaderValue::from_static("https://ccny-undergraduate.catalog.cuny.edu/"));      // was originally missing, caused error with data retrieval
    headers.insert("sec-ch-ua", HeaderValue::from_static("\"Not A(Brand\";v=\"8\", \"Chromium\";v=\"132\", \"Google Chrome\";v=\"132\""));
    headers.insert("sec-ch-ua-mobile", HeaderValue::from_static("?0"));
    headers.insert("sec-ch-ua-platform", HeaderValue::from_static("\"macOS\""));
    headers
}

// Function to fetch courses by department
// NOTE : requires prior knowledge of department_code from API payload
// department name is passed in and the corresponding department code is retrieved
// this function will be called upon within the higher abstracted function
pub async fn fetch_courses_by_department_helper(department_code: &str) -> Result<serde_json::Value> {

    // BASE API URL where various query parameters needs to be passed
    let base_url = "https://app.coursedog.com/api/v1/cm/cty01/courses/search/%24filters";
    
    // Query parameters is the same as website, regardless of the department
    let query_params = [
        ("catalogId", "tyrc1I8cy2QhVy5W5L2I"),
        ("skip", "0"),
        ("limit", "0"),     // set to 0 to ensure all courses are retrieved for a particular department.
        ("orderBy", "catalogDisplayName,transcriptDescription,longName,name"),
        ("formatDependents", "false"),
        ("effectiveDatesRange", "2024-08-28,2024-08-28"),
        ("columns", "displayName,department,name,courseNumber,subjectCode,code,courseGroupId,credits.creditHours,longName,career,components,customFields.catalogRequirementDesignation,customFields.catalogAttributes")
    ];

   // serde_json::json is where json! is coming from
    let payload = json!({
        "condition": "AND",
        "filters": [
            {
                "condition": "and",
                "filters": [
                    {
                        "id": "status-course",
                        "name": "status",
                        "inputType": "select",
                        "group": "course",
                        "type": "is",
                        "value": "Active"
                    },
                    {
                        "id": "catalogPrint-course",
                        "name": "catalogPrint",
                        "inputType": "boolean",
                        "group": "course",
                        "type": "is",
                        "value": true
                    },
                    {
                        "id": "career-course",
                        "name": "career",
                        "inputType": "careerSelect",
                        "group": "course",
                        "type": "is",
                        "value": "Undergraduate"
                    },
                    {
                        "id": "attributes-course",
                        "name": "attributes",
                        "inputType": "attributeSelect",
                        "group": "course",
                        "type": "doesNotContain",
                        "value": ["EXPR - EXPR (Experimental)"]
                    }
                ]
            },
            {
                "id": "departments-course",
                "name": "departments",
                "inputType": "select",
                "group": "course",
                "type": "contains",
                "value": [department_code]      // this department code is the only thing that
            }
        ]
    });

    // Create client and send request
    let client = reqwest::Client::new();
    let response = client
        .post(reqwest::Url::parse_with_params(base_url, &query_params)?)
        .headers(get_headers())
        .json(&payload)
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow::anyhow!("Failed to fetch courses: {}", response.status()));
    }

    let response_data = response.json().await?;
    Ok(response_data)
}

// Helper function to save response to file
pub fn save_to_file(data: &serde_json::Value, filename: &str) -> Result<PathBuf> {
    let file = fs::File::create(filename)?;

    // to_writer_pretty() : serialize the given data strucuture as a pretty-printed JSON into the I/O stream.
    serde_json::to_writer_pretty(file, data)?;
    Ok(PathBuf::from(filename))
}

// department_name : this is the user input
// TODO : define a struct to handle the response type
// should store Result<SomeStruct> later
pub async fn fetch_courses_by_department(department_name : &str) -> Result<serde_json::Value, anyhow::Error>{
    let department_mapping = get_department_mappings();
    let key_error_handler = String::from("None");

    // pass in the input validation function to convert the department_name to lowercase
    // reduces any kind of case sensetivity error that may arise
    let department_id = department_mapping.get(&input_validation(department_name)).unwrap_or(key_error_handler.borrow());

    if department_id == "None" {

        // specify an error message stating the department doesn't exist
        eprintln!("A department by this name doesn't exist, please refer to the list of departments.");

        // early abruption indicating end of program
        return Err(anyhow::Error::msg("Program Failed"));
    }

    // otherwise, if department name is valid
    // note that it's an array of data
    let course_info = fetch_courses_by_department_helper(department_id.borrow()).await?["data"].clone();


    // iterator logic (nested loop to bypass the indexing)
    for course in course_info.as_array().iter() {
        for course_info in course.iter() {
            println!("{:?}", course_info["name"]);
        }
    }
    // let course_component_instance = CourseComponents {
    //     course_type : 
    // }

    // let course_info = CourseInfo {
    //     unique_id : course_info["_id"],
    //     course_name : course_info["name"],
    //     career : course_info["career"],
    //     course_code : course_info["code"],
    //     course_components : course_component_instance,
    //     effective_end_date : Some(course_info["effectiveEndDate"]),
    //     effective_start_date : course_info["effectiveStartDate"],
    //     course_group_id : course_info["courseGroupId"],     // NOTE : needed to retrieve further information about a particular course
    //     course_number : course_info["courseNumber"],
    //     department : course_info["departments"][0],     // can cause errors
    //     subject_code : course_info["subjectCode"],
    //     credits : course_info["credits"]["creditHours"]["max"]
    // }

    Ok(course_info)
}

// TODO : define the logic for the function below
// should store Result<Vec<SomeStruct>> after
pub async fn fetch_all_courses() -> Result<()> {

    // retrieve list of departments
    let department_list = get_department_list();
    for department in department_list.iter() {
        // println!("current department : {department:?}");
        let course_data = fetch_courses_by_department(department).await?;
        // println!("{course_data:#?}");
        // save the data to the file
        let mut curr_dept = String::from(department);
        curr_dept.push_str(" data.json");       // append borrowed string
        save_to_file(&course_data, &curr_dept);
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let department_mappings = get_department_mappings();
    // print_hashmap_keys(department_mappings);        // experimental function (not sure of use case)

    // returns a list cotnianing the list of departments
    println!("{:#?}", get_department_list());
    // let course_fetch_result = fetch_courses_by_department("physics").await?;
    // fetch_all_courses().await?;
    let courses_data = fetch_courses_by_department("Biology").await?;
    
    // println!("{course_fetch_result:?}");
    Ok(())


}


// TODO : define a function that will take a user input and format it
// static function, the department name and their corresponding ID 
// has been identified manually
// the keys are very case sensetive
pub fn get_department_mappings() -> HashMap<String, String> {
    
    let test_map = HashMap::from([
        ("administration".to_owned(), "ADMIN-CTY".to_owned()),
        ("anthropology".to_owned(), "ANTH-CTY".to_owned()),
        ("architecture".to_owned(), "ARCH-CTY".to_owned()),
        ("art".to_owned(), "ART-CTY".to_owned()),
        ("biology".to_owned(), "BIO-CTY".to_owned()),
        ("biomedical education".to_owned(), "MED-CTY".to_owned()),
        ("biomedical engineering".to_owned(), "BME-CTY".to_owned()),
        ("cuny honors college".to_owned(), "HONOR-CTY".to_owned()),
        ("center for worker education".to_owned(), "CWE-CTY".to_owned()),
        ("chemical engineering".to_owned(), "CHE-CTY".to_owned()),
        ("chemistry".to_owned(), "CHEM-CTY".to_owned()),
        ("civil engineering".to_owned(), "CE-CTY".to_owned()),
        ("classical and modern languages and literatures".to_owned(), "LANG-CTY".to_owned()),
        ("computer science".to_owned(), "CSC-CTY".to_owned()),
        ("division of science".to_owned(), "DIVSCI-CTY".to_owned()),
        ("earth and atmospheric science".to_owned(), "EAS-CTY".to_owned()),
        ("economics and business".to_owned(), "ECO-CTY".to_owned()),
        ("education".to_owned(), "EDUC-CTY".to_owned()),
        ("electrical engineering".to_owned(), "EE-CTY".to_owned()),
        ("engineering".to_owned(), "ENGR-CTY".to_owned()),
        ("english".to_owned(), "ENGL-CTY".to_owned()),
        ("history".to_owned(), "HIST-CTY".to_owned()),
        ("humanities and the arts".to_owned(), "HART-CTY".to_owned()),
        ("interdisciplinary arts and sciences".to_owned(), "IAS-CTY".to_owned()),
        ("latin american & latino studies".to_owned(), "LALS-CTY".to_owned()),
        ("learning leadership and culture".to_owned(), "LRLDC-CTY".to_owned()),
        ("mathematics".to_owned(), "MATH-CTY".to_owned()),
        ("mechanical engineering".to_owned(), "ME-CTY".to_owned()),
        ("media and communication arts".to_owned(), "MCA-CTY".to_owned()),
        ("music".to_owned(), "MUS-CTY".to_owned()),
        ("philosophy".to_owned(), "PHIL-CTY".to_owned()),
        ("physics".to_owned(), "PHY-CTY".to_owned()),
        ("political science".to_owned(), "PSC-CTY".to_owned()),
        ("psychology".to_owned(), "PSY-CTY".to_owned()),
        ("school of biomedical education".to_owned(), "BMED-CTY".to_owned()),
        ("science".to_owned(), "SCI-CTY".to_owned()),
        ("social science".to_owned(), "SSCI-CTY".to_owned()),
        ("sociology".to_owned(), "SOC-CTY".to_owned()),
        ("teaching and learning".to_owned(), "TCHLR-CTY".to_owned()),
        ("grove school of engineering".to_owned(), "GROVE-CTY".to_owned()),
        ("theatre and speech".to_owned(), "THSP-CTY".to_owned())
        ]);
    test_map
}


// define the function for input validation
// this will take the user input and return the lower_case version of the user input
pub fn input_validation(user_input : &str) -> String {
    user_input.to_owned().to_lowercase()
}

// function used to retrieve list of departments
pub fn print_hashmap_keys(hashmap_input : HashMap<String, String>) {
    for (key,value) in hashmap_input.into_iter() {
        println!("current department : {key:?}");
    }
}

// more appropriate function to isolate list of departments
pub fn get_department_list() -> Vec<String> {
    let department_mapping = get_department_mappings();
    let mut department_list = Vec::new();
    for (key,value) in department_mapping.into_iter() {
        department_list.push(String::from(key));
    }

    department_list
}