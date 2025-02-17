// relevant imports
use reqwest;
use anyhow::Result;
use crate::models::{CourseInfo, CourseComponents};
use serde_json::json;
use closestmatch::ClosestMatch;
use std::borrow::Borrow;
use std::collections::HashMap;
use reqwest::header::{HeaderMap, ACCEPT, HeaderValue, CONTENT_TYPE, ORIGIN, REFERER};


// this will deal with any internal functions related to departments

/// Fetches courses for a specific department
/// 
/// # Arguments
/// * `department_name` - Name of the department
/// 
/// # Examples
/// ```rust
/// let courses = fetch_courses_by_department("computer science").await?;
/// ```
// department_name : this is the user input
pub async fn fetch_courses_by_department(department_name : &str) -> Result<Vec<CourseInfo>, anyhow::Error> {
    let mut CourseInfoVector : Vec<CourseInfo> = Vec::new();        // store results here
    let department_mapping = get_department_mappings();
    let key_error_handler = String::from("None"); 
    let department_id = department_mapping.get(&closest_matching_department(&input_validation(department_name))).unwrap_or(key_error_handler.borrow());
    // let mut course_integer = 0;
    if department_id == "None" {
        eprintln!("A department by this name doesn't exist, please refer to the list of departments.");
        return Err(anyhow::Error::msg("Program Failed"));
    }
    let course_info = fetch_courses_by_department_helper(department_id.borrow()).await?["data"].clone();

    // iterator logic (nested loop to bypass the indexing)
    for courses in course_info.as_array().iter() {
        for course_data in courses.iter() {
            let course_group_id : String = serde_json::from_value(course_data["courseGroupId"].clone()).unwrap();

            // println!("current course group id {course_group_id:?}");
            // if course_group_id.parse::<i32>().is_ok() {
            //     course_integer = course_group_id.parse().expect("Failed to parse");
            // } else {
            //     continue;
            // }
            // // let course_integer : i32 = course_group_id.parse().expect("Failed to parse");

            // println!("current course integer : {course_integer:?}");
            let mut course_component_data : Vec<CourseComponents> = Vec::new();
            for data in course_data["components"].as_array().iter() {
                for inner_data in data.iter() {
                    let mut course_component_instance = CourseComponents {
                        course_type : serde_json::from_value(inner_data["code"].clone()).unwrap(),

                        weekly_hours : serde_json::from_value(inner_data["contactHours"].clone()).unwrap_or(-1),

                        class_size : serde_json::from_value(inner_data["defaultSectionSize"].clone()).unwrap_or(-1),

                        final_exam : serde_json::from_value(inner_data["finalExamType"].clone()).unwrap(),
                        attendance_type : serde_json::from_value(inner_data["attendanceType"].clone()).unwrap(),

                        exam_seat_spacing : serde_json::from_value(inner_data["examSeatSpacing"].clone()).unwrap_or(-1),

                        instruction_mode : serde_json::from_value(inner_data["instructionMode"].clone()).unwrap()
                    };

                    course_component_data.push(course_component_instance);
                }
            }

            let effective_end_date_placeholder = serde_json::from_value(course_data["effectiveEndDate"].clone()).unwrap_or("null".to_owned());
            let course_number_string : String = serde_json::from_value(course_data["courseNumber"].clone()).unwrap();

            let course_number_string_filtered : String = course_number_string.chars().filter(|c| c.is_digit(10)).collect();  // removes any unneccessary values
            let course_number_integer : i64 = course_number_string_filtered.parse().unwrap_or(64);
            if course_number_integer == 64 {
                println!("{:?}",course_number_string_filtered);
            }
            // let credits = course_data["credits"]["creditHours"]["max"].clone().to_string();
            // println!("{credits:?}");

            // let credits_numerical : i32 = 
            // println!("{:?}",serde_json::from_value(course_data["credits"]["creditHours"]["max"].clone()).unwrap());

            // println!("credits converted val : {credits_converted:?}");
            let mut course_info_instance = CourseInfo {
                unique_id : serde_json::from_value(course_data["_id"].clone()).unwrap(),

                course_name : serde_json::from_value(course_data["name"].clone()).unwrap(),

                career : serde_json::from_value(course_data["career"].clone()).unwrap(),

                course_code : serde_json::from_value(course_data["code"].clone()).unwrap(),

                course_components : serde_json::from_value(serde_json::to_value(course_component_data[0].clone()).unwrap().clone()).unwrap(),       // only the first instance is relevant

                effective_start_date : serde_json::from_value(course_data["effectiveStartDate"].clone()).unwrap(),

                effective_end_date : "unknown".to_owned(),      // remains the same throughout

                course_group_id : course_group_id,

                course_number : course_number_integer,

                department : serde_json::from_value(course_data["departments"].clone()).unwrap(),

                subject_code : serde_json::from_value(course_data["subjectCode"].clone()).unwrap(),

                credits : course_data["credits"]["creditHours"]["max"].clone().to_string()
                
            };
            CourseInfoVector.push(course_info_instance);
        }

    }
    Ok(CourseInfoVector)
}

// helper function to match and filter based on the closest matching string
pub fn closest_matching_department(user_input_department_name : &str) -> String {
    let mut result_string : String = String::new();
    let department_list : Vec<String> = get_department_list();        
    let mut min_length = usize::MAX;        // initilize the largest val

    // create the bagged length of vector
    // nested loop statement is needed, since we first need the current string
    // first loop iterates over every individual element within the vector
    // to determine the smallest length of strings
    // bag of words : [1..=min_length]
    for department_name in department_list.iter() {
        min_length = std::cmp::min(min_length, department_name.len());
    }

    let mut bag_length_vector : Vec<usize> = Vec::new();        // create an empty vector
    
    for i in 1..=min_length {
        bag_length_vector.push(i);      // push the range of numerical values
    }

    // create the closestmatching instance string
    let closest_matching_checker = ClosestMatch::new(department_list, bag_length_vector);

    // search for the string
    // the get_closest method takes in an owned string
    // since it's wrapped around
    result_string = closest_matching_checker.get_closest(user_input_department_name.to_string()).unwrap();
    result_string
}


// Helper function to fetch courses by department
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

// converts string based input to lowercase
pub fn input_validation(user_input : &str) -> String {
    user_input.to_owned().to_lowercase()
}

// Helper function that maps name of departments to their corresponding ID
pub fn get_department_mappings() -> HashMap<String, String> {
    
    let department_map = HashMap::from([
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
    department_map
}

// Helper function to retrieve information about headers
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

// calls upon get_department_mappings and isolates the keys into a seperate array that is returned
pub fn get_department_list() -> Vec<String> {
    let department_mapping = get_department_mappings();
    let mut department_list = Vec::new();
    for (key,value) in department_mapping.into_iter() {
        department_list.push(String::from(key));
    }

    department_list
}