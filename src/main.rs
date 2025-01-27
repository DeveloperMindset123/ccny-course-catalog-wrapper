// NOTE : upon retrieving the list of courses, additional information about the course can be retrieved using the courseGroupId

// GET Method --> https://app.coursedog.com/api/v1/cty01/general/terms : general api to retrieve information regarding all the terms
// TODO : look into the Levenshtein method for string matching as alternative for search lookup and suggesting corresponding value
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
use closestmatch::ClosestMatch;     // library to determine the closest matching string

// This struct is inherited within CourseInfo struct
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CourseComponents {
    pub course_type : String,           // (i.e. LEC)
    pub attendance_type : String,       // (i.e. "Class Meeting")
    pub weekly_hours : i32,         // (i.e. 3 hour per week)
    pub class_size : i32,
    pub final_exam : String,
    pub exam_seat_spacing : i32,
    pub instruction_mode : String   
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CourseInfo {
    pub unique_id : String,
    pub course_name : String,       // course LongName
    pub career : String,
    pub course_code : String,
    pub course_components : CourseComponents,
    pub effective_start_date : String,          // indicates the time this course was released

    // #[serde(deserialize_with="deserialize_null_default")]
    pub effective_end_date : String,        // indicates the time the course may end (could be null)
    pub course_group_id : i32,
    pub course_number : i64,
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

// search for a particular course
// pub struct SearchCourse {
//     pub course_name : String,
//     pub course_id : String
// }

// retrieve course ID based on prior knowledge of course code
pub async fn retrieve_course_id_by_course_code(course_code : &str, department_name : &str) -> String {
    "true".to_string()      // not implemented
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
    // println!("current closest department is : {closest_department:?}");
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
    let closest_matching_course : String = course_search_engine.get_closest(course_name_input.to_string()).unwrap();
    // NOTE : skipping the error handling in the event that course doesn't exist
    // since that's not important right now and can be implemented within the backend itself
    // the value passed into unwrap_or is known as "deref coercion"
    let course_id : String = course_name_and_id_map.get(&closest_matching_course).unwrap_or(&"Course Group ID Does not exist for this course".into()).to_string();
    course_id.to_owned()
}

// course_name : name of the course (i.e. CSC 103, CSC 104)
// need to determine the appropriate course ID that matches the particular course name
// can search through the list of courses available and retrieve the course code corresponding to them based on the previous function that has been defined
// construct a hashmap based on the list of courses, check if the course name matches any 
// we have to set the course_code as the key and course_group_id as the value corresponding to the key
// header related information for this particular API call should remain more or less the same
pub async fn retrieve_specific_course_info(course_name : &str, department_name : &str) -> Result<()>{

    // TODO : implement the logic for this later
    // should be storing a tuple of values
    let course_name_and_code_mapping : BTreeMap<String, (i32, String)> = BTreeMap::new();     // this is where the mapping logic will be stored
    let base_url = "https://app.coursedog.com/api/v1/cm/cty01/courses/search/$filters";

    // TODO : implment logic for retrieving course id based on course_name and department_name
    // 
    // to reduce the code complexity, implement this as a helper function
    // first find the department_name that is the closest matching (make a call to retrieve list of courses within the specific department)
    // use a struct to store the course name and course ID
    // search the closest matching course and return the course ID in 
    // define the query params that needs to be passed in as part of the POST request
    let query_params = [
        ("courseGroupIds", "0571501"),          // NOTE : this group ID should be changing dynamically
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

    println!("{json_response:#?}");

    Ok(())



}

pub async fn retrieve_historical_terms() -> Result<()> {

    // basic GET request to retrieve all the historical term related information
    let get_request_url = "https://app.coursedog.com/api/v1/cty01/general/terms";
    let body = reqwest::get(get_request_url).await?.text().await?;
    let json_string : serde_json::Value = serde_json::from_str(&body)?;
    println!("{json_string:#?}");       // tested : worked

    Ok(())
}

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
// should store Result<Vec<SomeStruct>, anyhow::Error> later
// old return statement : Result<serde_json::Value, anyhow::Error>
pub async fn fetch_courses_by_department(department_name : &str) -> Result<Vec<CourseInfo>, anyhow::Error> {
    let mut CourseInfoVector : Vec<CourseInfo> = Vec::new();        // store results here
    let department_mapping = get_department_mappings();
    let key_error_handler = String::from("None"); 

    // pass in the input validation function to convert the department_name to lowercase
    // reduces any kind of case sensetivity error that may arise
    // input_validation accepts a &str as a parameter
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
    for courses in course_info.as_array().iter() {
        for course_data in courses.iter() {
            // TODO : create the course instance here
            // NOTE : one potential workaround, placing the data into a vector instead
            // let mut course_component_instance;
            // println!("{course_component_instance:#?}")
            // println!("{:#?}", course_data["components"][0]);
            let course_group_id : String = serde_json::from_value(course_data["courseGroupId"].clone()).unwrap();

            let course_integer : i32 = course_group_id.parse().expect("Failed to parse");
            // println!("{course_integer:?}");
            // let course_group_id_num : i32 = serde_json::from_str(&course_group_id).unwrap();

            // println!("current department ID : {course_group_id_num:?}");
            

            let mut course_component_data : Vec<CourseComponents> = Vec::new();
            for data in course_data["components"].as_array().iter() {
                for inner_data in data.iter() {
                    // TODO : remove the to_string values
                    // println!("{:#?}", inner_data["attendanceGenerate"]);
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

            // TODO : process the rest of the data
            // println!("{course_data:#?}");

            // ternary operator
            let effective_end_date_placeholder = serde_json::from_value(course_data["effectiveEndDate"].clone()).unwrap_or("null".to_owned());
            // println!("{effective_end_date_placeholder:?}");

            // let course_number_string : String = serde_json::from_value(course_data["courseNumber"].clone()).unwrap();

            // steps take for string filtering logic:
            //
            // 1. convert to string data
            // 2. remove any non-numerical values
            // 3. parse the string and convert it into an integer of type i64
            let course_number_string : String = serde_json::from_value(course_data["courseNumber"].clone()).unwrap();

            // remove any unneccessary values
            let course_number_string_filtered : String = course_number_string.chars().filter(|c| c.is_digit(10)).collect();
            let course_number_integer : i64 = course_number_string_filtered.parse().expect("failed to parse integer");
            
            let mut course_info_instance = CourseInfo {
                unique_id : serde_json::from_value(course_data["_id"].clone()).unwrap(),

                course_name : serde_json::from_value(course_data["name"].clone()).unwrap(),

                career : serde_json::from_value(course_data["career"].clone()).unwrap(),

                course_code : serde_json::from_value(course_data["code"].clone()).unwrap(),

                course_components : serde_json::from_value(serde_json::to_value(course_component_data[0].clone()).unwrap().clone()).unwrap(),       // only the first instance is relevant

                effective_start_date : serde_json::from_value(course_data["effectiveStartDate"].clone()).unwrap(),

                // TODO : wrap this around a conditional statement
                // effective_end_date : effective_end_date_instance,
                effective_end_date : "unknown".to_owned(),

                course_group_id : course_integer,

                course_number : course_number_integer,

                department : serde_json::from_value(course_data["departments"].clone()).unwrap(),

                subject_code : serde_json::from_value(course_data["subjectCode"].clone()).unwrap(),

                credits : serde_json::from_value(course_data["credits"]["creditHours"]["max"].clone()).unwrap()
                
            };
            CourseInfoVector.push(course_info_instance);
            // println!("{course_info_instance:#?}");
        }

    }


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
    // println!("The course info vector is : {CourseInfoVector:?}");
    println!("{:#?}", CourseInfoVector[0]);
    Ok(CourseInfoVector)
}

// TODO : define the logic for the function below
// should store Result<Vec<SomeStruct>> after
pub async fn fetch_all_courses() -> Result<()> {

    // retrieve list of departments
    // let department_list = get_department_list();
    // for department in department_list.iter() {
    //     // println!("current department : {department:?}");
    //     let course_data = fetch_courses_by_department(department).await?;
    //     // println!("{course_data:#?}");
    //     // save the data to the file
    //     let mut curr_dept = String::from(department);
    //     curr_dept.push_str(" data.json");       // append borrowed string
    //     // save_to_file(&course_data, &curr_dept);
    // }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let department_mappings = get_department_mappings();
    // print_hashmap_keys(department_mappings);        // experimental function (not sure of use case)

    // returns a list cotnianing the list of departments
    // println!("{:#?}", get_department_list());
    // let course_fetch_result = fetch_courses_by_department("physics").await?;
    // fetch_all_courses().await?;
    // let courses_data = fetch_courses_by_department("Biology").await?;

    // department_name should be passed in as a parameter to find the closest matching department
    // let department_name_filtered : String = closest_matching_department("civil");

    // TODO : retrieve_course_id_by_course_name : if the user already knows the course name by heart
    // TODO : retrieve_course_id_by_course_code : 
    // 1st param : name of course (in NLP format)
    // 2nd param : name of department 

    retrieve_course_id_by_course_name("hydraulic", "civil").await;


    // println!("filtered department name is : {department_name_filtered:?}");
    
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
// this is for users to get a general idea of list of departments that are availale 
// returns all the relevant list of departments within CCNY alone
pub fn get_department_list() -> Vec<String> {
    let department_mapping = get_department_mappings();
    let mut department_list = Vec::new();
    for (key,value) in department_mapping.into_iter() {
        department_list.push(String::from(key));
    }

    department_list
}

// function that isolates and returns list of courses relevant to a particular department
// TODO : implement the logic for determining the closest matching department
// should determine the closest matching department by searcing through the department list
pub async fn get_course_list_by_department(department_name : &str) -> Result<Vec<String>, anyhow::Error> {
    let course_list : Vec<String> = Vec::new();
    // call on the function to retrieve the course
    let department_courses : Vec<CourseInfo> = fetch_courses_by_department(department_name).await?;
    println!("{department_courses:#?}");

    Ok(course_list)
}

// This is a helper function, user should not be seeing this function
// I have an array of strings
// I take the smallest length of the department 
// then I create a bag of words out of it to split the list of departments
// find the closest matching department and return it
pub fn find_closest_matching_course(course_name : &str) -> String {
    let mut resulting_string : String = String::new();      // create an empty new string

    // call on the function to retrieve the list of 
    "unimplemented".to_string()     // placeholder
}

// helper function to match and filter based on the closest matching string
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