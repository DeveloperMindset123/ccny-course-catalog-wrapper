use anyhow::Result;
use reqwest;
use reqwest::header::{HeaderMap, HeaderValue};
use async_compression::futures::bufread::GzipDecoder;
use futures::{
    io::{self, BufReader, ErrorKind},
    prelude::*,
};
use std::{fs, io::Write};
use filepath::FilePath;
use std::path::PathBuf;
// TODO : define a function that will determine the department and retrieve the courses from the particular department.

#[tokio::main]
pub async fn main() -> Result<()> {
    let base_url = "https://ccny-undergraduate.catalog.cuny.edu";

    // pass query params within an array
    // elements being tuples
    // query params should be passed in as strings
    let query_params = [
        ("catalogId", "tyrc1I8cy2QhVy5W5L2I"),
        ("skip", "0"),
        ("limit", "10"),
        ("orderBy", "catalogDisplayName,transcriptDescription,longName,name"),
        ("formatDependents", "false"),
        ("effectiveDatesRange", "2024-08-28,2024-08-28"),
        ("columns", "displayName,department,name,courseNumber,subjectCode,code,courseGroupId,credits.creditHours,longName,career,components,customFields.catalogRequirementDesignation,customFields.catalogAttributes")
    ];

    // create a new client
    let client = reqwest::Client::new();

    // use the json! macro froms serde_json to create the json body
    let payload = serde_json::json!({
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
          "value": [
            "EXPR - EXPR (Experimental)"
          ]
        }
      ]
    },
    {
      "id": "departments-course",
      "name": "departments",
      "inputType": "select",
      "group": "course",
      "type": "contains",
      "value": [
        "DIVSCI-CTY"
      ]
    }
  ]
});
    // println!("{payload:#?}");        // tested : worked
    let get_header_data = get_headers();    
    // println!("{get_header_data:#?}");        // tested : worked

    let response = client.post("
https://app.coursedog.com/api/v1/cm/cty01/courses/search/%24filters").form(&query_params).json(&payload).send().await?;
    let mut response_data : serde_json::Value = response.json().await?;
    println!("{:#?}", response_data);

    // if !response.status().is_success() {
    //     return Err(anyhow::anyhow!("Error with making post request"));
    // }
    // let reader = response.bytes_stream().map_err(|e| io::Error::new(ErrorKind::Other, e)).into_async_read();

    // let mut gz_decoder = GzipDecoder::new(BufReader::new(reader));
    // gz_decoder.multiple_members(true);

    // let decoder = BufReader::new(gz_decoder);
    // let mut lines_stream = decoder.lines().map(|l| l.unwrap());

    // let mut line = lines_stream.next().await.unwrap_or(String::from(""));
    // while line.len() > 0 {
    //     println!("{:?}", line);
    //     line = lines_stream.next().await.unwrap_or(String::from(""));
    // }
    // let mut response_data : serde_json::Value = response.json().await?;
    // let mut res : String = serde_json::to_string(&response_data)?;
    // let some_string_data = serde_json::from_str(&res)?;
    // println!("{:#?}", response.text().await?);
    // println!("{response_data:#?}");
    Ok(())
}

// define a helper that will retrieve the headers
pub fn get_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("User-Agent", HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/132.0.0.0 Safari/537.36"));
    headers.insert("Accept", HeaderValue::from_static("*/*"));
    headers.insert("Accept-Encoding", HeaderValue::from_static("gzip, deflate, br, zstd"));
    headers.insert("Content-Type", HeaderValue::from_static("*/*"));
    headers.insert("Priority", HeaderValue::from_static("u=1, i"));
    headers.insert("Pragma", HeaderValue::from_static("no-cache"));
    // note how string formatting works
    headers.insert("Sec-Ch-Ua", HeaderValue::from_static("\"Not A(Brand\";v=\"8\", \"Chromium\";v=\"132\", \"Google Chrome\";v=\"132\""));
    headers.insert("Content-Length", HeaderValue::from_static("100"));
    headers.insert("Sec-Ch-Ua-Mobile", HeaderValue::from_static("?0"));
    headers.insert("Sec-Ch-Ua-Platform", HeaderValue::from_static("\"macOS\""));
    headers.insert("Sec-Fetch-Dest", HeaderValue::from_static("empty"));
    headers.insert("Sec-Fetch-Mode", HeaderValue::from_static("cors"));
    headers.insert("Sec-Fetch-Site", HeaderValue::from_static("cross-site"));
    headers.insert("Origin", HeaderValue::from_static("https://ccny-undergraduate.catalog.cuny.edu"));
    
    // return the header map
    headers
}

pub async fn create_file(fileName : &str) -> (fs::File, PathBuf) {
  let mut file = fs::File::create(fileName).unwrap();
  let filePath = file.path().unwrap();    // Ok("/path/to/file") -> "/path/to/file"
  (file, filePath)
}

pub async fn save_data_to_file(mut file : fs::File, data : &str) {
  file.write_all(data.as_bytes()).expect("failed to write json data to file")
}
// {
//     "condition": "AND",
//     "filters": [
//         {
//             "condition": "and",
//             "filters": [
//                 {
//                     "id": "status-course",
//                     "name": "status",
//                     "inputType": "select",
//                     "group": "course",
//                     "type": "is",
//                     "value": "Active"
//                 },
//                 {
//                     "id": "catalogPrint-course",
//                     "name": "catalogPrint",
//                     "inputType": "boolean",
//                     "group": "course",
//                     "type": "is",
//                     "value": true
//                 },
//                 {
//                     "id": "career-course",
//                     "name": "career",
//                     "inputType": "careerSelect",
//                     "group": "course",
//                     "type": "is",
//                     "value": "Undergraduate"
//                 },
//                 {
//                     "id": "attributes-course",
//                     "name": "attributes",
//                     "inputType": "attributeSelect",
//                     "group": "course",
//                     "type": "doesNotContain",
//                     "value": [
//                         "EXPR - EXPR (Experimental)"
//                     ]
//                 }
//             ]
//         },
//         {
//             "id": "departments-course",
//             "name": "departments",
//             "inputType": "select",
//             "group": "course",
//             "type": "contains",
//             "value": [
//                 "BME-CTY"       // this is what is being inputted
//             ]
//         }
//     ]
// }

// {
//   "condition": "AND",
//   "filters": [
//     {
//       "condition": "and",
//       "filters": [
//         {
//           "id": "status-course",
//           "name": "status",
//           "inputType": "select",
//           "group": "course",
//           "type": "is",
//           "value": "Active"
//         },
//         {
//           "id": "catalogPrint-course",
//           "name": "catalogPrint",
//           "inputType": "boolean",
//           "group": "course",
//           "type": "is",
//           "value": true
//         },
//         {
//           "id": "career-course",
//           "name": "career",
//           "inputType": "careerSelect",
//           "group": "course",
//           "type": "is",
//           "value": "Undergraduate"
//         },
//         {
//           "id": "attributes-course",
//           "name": "attributes",
//           "inputType": "attributeSelect",
//           "group": "course",
//           "type": "doesNotContain",
//           "value": [
//             "EXPR - EXPR (Experimental)"
//           ]
//         }
//       ]
//     },
//     {
//       "id": "departments-course",
//       "name": "departments",
//       "inputType": "select",
//       "group": "course",
//       "type": "contains",
//       "value": [
//         "CWE-CTY"
//       ]
//     }
//   ]
// }

// {
//   "condition": "AND",
//   "filters": [
//     {
//       "condition": "and",
//       "filters": [
//         {
//           "id": "status-course",
//           "name": "status",
//           "inputType": "select",
//           "group": "course",
//           "type": "is",
//           "value": "Active"
//         },
//         {
//           "id": "catalogPrint-course",
//           "name": "catalogPrint",
//           "inputType": "boolean",
//           "group": "course",
//           "type": "is",
//           "value": true
//         },
//         {
//           "id": "career-course",
//           "name": "career",
//           "inputType": "careerSelect",
//           "group": "course",
//           "type": "is",
//           "value": "Undergraduate"
//         },
//         {
//           "id": "attributes-course",
//           "name": "attributes",
//           "inputType": "attributeSelect",
//           "group": "course",
//           "type": "doesNotContain",
//           "value": [
//             "EXPR - EXPR (Experimental)"
//           ]
//         }
//       ]
//     },
//     {
//       "id": "departments-course",
//       "name": "departments",
//       "inputType": "select",
//       "group": "course",
//       "type": "contains",
//       "value": [
//         "CHE-CTY"       // this value also differs
//       ]
//     }
//   ]
// }
// this string remains practically unchanged
// "https://app.coursedog.com/api/v1/cm/cty01/courses/search/%24filters?catalogId=tyrc1I8cy2QhVy5W5L2I&skip=0&limit=20&orderBy=catalogDisplayName%2CtranscriptDescription%2ClongName%2Cname&formatDependents=false&effectiveDatesRange=2024-08-28%2C2024-08-28&columns=displayName%2Cdepartment%2Cname%2CcourseNumber%2CsubjectCode%2Ccode%2CcourseGroupId%2Ccredits.creditHours%2ClongName%2Ccareer%2Ccomponents%2CcustomFields.catalogRequirementDesignation%2CcustomFields.catalogAttributes"

// test the following functio



