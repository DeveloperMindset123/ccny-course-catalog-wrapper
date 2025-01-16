use anyhow::Result;
use reqwest;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, CONTENT_TYPE, ORIGIN, REFERER};
use serde_json::json;
use std::{fs, io::Write};
use std::path::PathBuf;
use std::collections::HashMap;
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
    
    println!("{courses:#?}");
    // Save to file
    save_to_file(&courses, "csc_courses.json")?;
    let test_hashmap_example = get_department_mappings();
    println!("{:#?}", test_hashmap_example);
    println!("Courses fetched and saved successfully!");
    Ok(())
}


// TODO : define a function that will take a user input and format it
pub fn get_department_mappings() -> HashMap<String, String> {
    let test_map = HashMap::from([("Mercury".to_owned(), "0.4".to_owned())]);
    test_map
}


