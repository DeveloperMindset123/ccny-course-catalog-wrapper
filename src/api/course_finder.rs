// import functions relevant to courses here
// this module may need to interact with department.rs
use super::department::*;
use anyhow::Result;
use reqwest;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, CONTENT_TYPE, ORIGIN, REFERER};
use serde_json::json;
use std::{fs, io::Write};
use std::path::PathBuf;
use std::collections::{HashMap, BTreeMap};
use std::borrow::Borrow;
use serde::{Deserialize, Serialize};
use std::ptr::null;
use closestmatch::ClosestMatch;

pub async fn retrieve_historical_terms() -> Result<()> {

    // basic GET request to retrieve all the historical term related information
    let get_request_url = "https://app.coursedog.com/api/v1/cty01/general/terms";
    let body = reqwest::get(get_request_url).await?.text().await?;
    let json_string : serde_json::Value = serde_json::from_str(&body)?;

    Ok(())
}

// helper function to match and filter based on the closest matching string
// NOTE : this function is used within retrieve_course_id_by_course_name
pub fn closest_matching_department(user_input_department_name : &str) -> String {
    let mut result_string : String = String::new();
    // get_department_list() is a synchronous function
    let department_list : Vec<String> = get_department_list();        // returns a vector of Strings
    let mut min_length = usize::MAX;        // initilize the largest val

    // create the bagged length of vector
    // nested loop statement is needed, since we first need the current string
    // first loop iterates over every individual element within the vector
    // to determine the smallest length of strings
    // bag of words : [1..=min_length]
    for department_name in department_list.iter() {
        min_length = std::cmp::min(min_length, department_name.len());
    }

    // create an empty vector
    let mut bag_length_vector : Vec<usize> = Vec::new();        
    
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

// retrieve the course group ID based on prior knowledge of course_name (not to be mistaken)
// the function should take in the department name as the parameter
// iterate over the returned data and isolate the course name and course code
// we will need 2 things : a hashmap to map the course name to the course group ID
// an array to store the name of the courses that will be used for searching purposes
pub async fn retrieve_course_id_by_course_name(course_name_input : &str, department_name : &str) -> String {
    let mut closest_course_groupID = String::new();        // result will be stored here
    let mut course_name_list : Vec<String> = Vec::new();    // isolates name of courses based on the retrieved data
    let mut course_name_and_id_map : HashMap<String, String> = HashMap::new();         // maps course name to course group ID
    let mut smallest_course_length = usize::MAX;        // stores the length of smallest course
    let mut bag_of_words : Vec<usize> = Vec::new();     // stores the length of possible subarrays

    let closest_department : String = closest_matching_department(department_name);
    let mut courses_by_department = fetch_courses_by_department(&closest_department).await.unwrap();

    // isolate the courses and store them within course_name_list vector
    // form the hashmap as well
    for course_data in courses_by_department.iter() {
        course_name_list.push(course_data.course_name.clone().to_lowercase());
        course_name_and_id_map.insert(course_data.course_name.clone().to_lowercase(), course_data.course_group_id.to_string());
        smallest_course_length = std::cmp::min(smallest_course_length, course_data.course_name.clone().len());
    }
    
    // create the bag of word array and search for the closest matching course
    for val in 0..=smallest_course_length {
        bag_of_words.push(val);
    }

    // search for the closest matching course
    let course_search_engine = ClosestMatch::new(course_name_list.clone(), bag_of_words);

    // let error_msg : &str = "Course Group ID Does not exist for this course";
    // the value passed into unwrap_or is known as "deref coercion"
    let closest_matching_course : String = course_search_engine.get_closest(course_name_input.to_string()).unwrap();
    let course_id : String = course_name_and_id_map.get(&closest_matching_course).unwrap_or(&"Course Group ID Does not exist for this course".into()).to_string();
    course_id.to_owned()
}

// course_name : name of the course (i.e. CSC 103, CSC 104)
// need to determine the appropriate course ID that matches the particular course name
// can search through the list of courses available and retrieve the course code corresponding to them based on the previous function that has been defined
// construct a hashmap based on the list of courses, check if the course name matches any 
// we have to set the course_code as the key and course_group_id as the value corresponding to the key
// header related information for this particular API call should remain more or less the same
pub async fn retrieve_specific_course_info(course_name : &str, department_name : &str) -> Result<serde_json::Value>{
    // NOTE : if the first value of the course_group_id starts with a 1, that means we don't have to prepend a 0 to the existing string
    // otherwise however we do have to prepend a 0 to the string
    // to ensure that all courses can be searched

    // should be storing a tuple of values
    let course_name_and_code_mapping : BTreeMap<String, (i32, String)> = BTreeMap::new();     // this is where the mapping logic will be stored
    let base_url = "https://app.coursedog.com/api/v1/cm/cty01/courses/search/$filters";
    let mut complete_course_group_id : String = String::new();
    let course_group_id : &str = &retrieve_course_id_by_course_name(course_name, department_name).await;

    // check and test the control group ID
    // control flow to determine whether course group id is 6 or 7 characters long
    if course_group_id.len() < 7 {
        complete_course_group_id = "0".to_owned() + course_group_id;   
    } else {
        complete_course_group_id = course_group_id.to_string();
    }


    let course_group_id_ref : &str = &complete_course_group_id;
    // to reduce the code complexity, implement this as a helper function
    // first find the department_name that is the closest matching (make a call to retrieve list of courses within the specific department)
    // use a struct to store the course name and course ID
    // search the closest matching course and return the course ID in 
    // define the query params that needs to be passed in as part of the POST request
    let query_params = [
        ("courseGroupIds", course_group_id_ref),          // NOTE : this group ID should be changing dynamically
        ("effectiveDatesRange", "2024-08-28,2024-08-28"),       // NOTE : this value should also update dynamically, it's on the course list struct

        // below statements can be the same throughout (meaning they are static query params)
        ("formatDependents", "false"),
        ("includeRelatedData", "true"),
        ("includeCrosslisted", "false"),
        ("includeCourseEquivalencies", "true"),
        ("includePending", "false"),
        ("includeMappedDocumentItems", "true"),
        ("returnResultsWithTotalCount", "false"),
        ("doNotDisplayAllMappedRevisionsAsDependencies", "true"),
        ("columns", "departments,courseTypicallyOffered,career,credits,components,topics,catalogAttributes,description,requirementGroup,courseSchedule,customFields.ZK6fC,longName,institution,consent,customFields.cuPathwaysAttribute,subjectCode,courseNumber,customFields.cuLibartsFlag,code,name,college,status,institutionId,rawCourseId,crseOfferNbr,customFields.catalogAttributes,customFields.rawCourseId")
    ];

    // commented out as this was used for debugging purpose only
    // utils::print_type_of(&query_params);

    // NOTE : there's no payload involved for this query parameter
    let client = reqwest::Client::new();
    let response = client.post(reqwest::Url::parse_with_params(base_url, &query_params)?).headers(get_headers()).send().await?;
    if !response.status().is_success() {
        return Err(anyhow::anyhow!("Failed to fetch courses: {}", response.status()));
    }

    // retrieve the raw text data
    let response_data_raw = response.text().await?;

    // convert the text data to Json format
    let json_response : serde_json::Value = serde_json::from_str(&response_data_raw)?;

    // println!("{json_response:#?}");

    Ok(json_response)
}