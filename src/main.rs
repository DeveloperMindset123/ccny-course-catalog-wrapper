// NOTE : upon retrieving the list of courses, additional infomration about the course can be retrieved using the courseGroupId

// GET Method --> https://app.coursedog.com/api/v1/cty01/general/terms : general api to retrieve information regarding all the terms

use anyhow::Result;
use reqwest;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, CONTENT_TYPE, ORIGIN, REFERER};
use serde_json::json;
use std::{fs, io::Write};
use std::path::PathBuf;
use std::collections::HashMap;
use std::borrow::Borrow;
// define the function to get the header
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
pub async fn fetch_courses_by_department(department_code: &str) -> Result<serde_json::Value> {

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
    serde_json::to_writer_pretty(file, data)?;
    Ok(PathBuf::from(filename))
}

#[tokio::main]
async fn main() -> Result<()> {
    // Example usage: fetch Computer Science courses
    let courses = fetch_courses_by_department("EAS-CTY").await?;
    
    // println!("{courses:#?}");
    // // Save to file
    // save_to_file(&courses, "csc_courses.json")?;
    let department_mappings = get_department_mappings();
    print_hashmap_keys(department_mappings);

    // borrow() : the ownership of a value is transferred temporarily to an entity and then returned to the original owner entity at the end of the program execution.
    // let no_key_handler = String::from("Key doesn't exist, please check the list of departments and try again, please be cautious to avoid spelling errors");
    // let value = department_mappings.get("art").unwrap_or(no_key_handler.borrow());
    // println!("{value:?}");

    // let no_key_handler_cloned = no_key_handler.clone();
    // let non_existent_value = department_mappings.get("biology_department").unwrap_or(no_key_handler.borrow());
    // println!("{non_existent_value:?}");
    // println!("{:#?}", test_hashmap_example);
    // println!("Courses fetched and saved successfully!");

    // tested : worked
    // println!("Testing user input : {:?}", input_validation("Some Random Department"));
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