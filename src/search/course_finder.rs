use anyhow::Result;
use crate::{
    models::CourseInfo,
    api::fetch_courses_by_department,
    storage::CourseStorage,
};

use reqwest;
use serde_json::json;
use crate::models::CourseInfo;

/// Finds a course by name and department
/// 
/// # Arguments
/// * `course_name` - Name or partial name of the course
/// * `department_name` - Name of the department
/// 
/// # Examples
/// ```rust
/// let course = find_course_by_name_and_dept("machine learning", "computer science").await?;
/// ```
pub async fn find_course_by_name_and_dept(
    course_name : &str,     // (i.e. introduction to art, world humanities, web design)
    department_name : &str
) -> Result<CourseInfo> {
    // Get list of department courses
    let courses = fetch_courses_by_department(department_name).await?;

    // extract course name for matching
    //
    // there exists various methods to modify or consume iterators
    //
    // these methods are called iterator adaptors and are chained together to create expressive, functional code.

    // some common iteartor adaptors include:
    // .map() : Transforms each element in the iteartor.
    // .filter() : filters elements based on a predicate function.
    // .fold() : aggregates elements into a single value.
    //
    // the .map() method transforms each element in the iterator by applying a closure to it.
    // the closure takes an element as input and returns a new value, which is then yieled by the iteartor.
    //
    // refer below for some examples that has been commented out
    let course_names : Vec<String> = courses.iter().map(|c| c.course_name.clone()).collect();

    // create matcher
    let matcher = StringMatcher::new(course_names);

    // Find the closest match
    // find_closest is a object instantiated based method from StringMatcher
    //
    // Option::ok_or_else() and Option::unwrap_or_else() are both functions that accepts closure
    // and the function will be executed only "if needed"
    let matched_name = matcher.find_closest(course_name).ok_or_else(|| anyhow::anyhow!("No matching course found"));

    // iter() : creates a reference iterator
    // This means collection is not moved and can be continued after the iteration
    // the iter() method is used when we want to iterate over elements without taking ownership
    // 
    // into_iter() : consumes the collection, calling into_iter() on a collection transforms it into an iterator that yields items by value.
    // this method consumes the collection, meaning you can't use it afterward because it's ownership is transferred to the iterator instead.
    // the "workaround" is to use a .clone() method
    //
    // find() : the find() method is used to search for the first occurence of a specific value wtihin an iterable (like a string or vector) and return the index of that value if found
    // Find and return the course
    courses.into_iter().find(|c| c.course_name == matched_name).ok_or_else(|| anyhow::anyhow!("Course not found"))
}



/// Retrieves detailed course information by course ID
// pub async fn get_course_details(course_id : &str) -> Result<CourseInfo> {
//     let client
// }




// example code to understand how iter(), filter() and fold() method works within an array
// code below is from rust playground
//
// fn collect_and_iter() {

//     // we don't have to use iter() method here
//     let range_vector : Vec<usize> = (1..=10).collect();
    
//     // we can now use the iter() and corresponding consumption method 
//     let square_vectors : Vec<usize> = range_vector.iter().map(|elem| elem * elem).collect();
//     println!("{square_vectors:#?}");
    
//     // we can apply additional modifications
//     // takes in a predicate
//     let filtered_square_vector : Vec<usize> = square_vectors.into_iter()
//                                                 .filter(|square_elem| square_elem > &30)
//                                                     .collect();
                                                    
//     println!("{filtered_square_vector:#?}");    
    
//     // we can consume a cloned version of range_vector instead
//     for mut data in range_vector.clone().into_iter() {
//         data = data / 2;
//         println!("current data : {data:?}");
//     }
    
//     println!("{range_vector:#?}");
// }

// fn main() {
//     collect_and_iter();
// }