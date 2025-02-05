use ccny_course_catalog_wrapper::CCNYCourseCatalog;
use tokio;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let mut course_instance = CCNYCourseCatalog::new(String::from("computer science"), Some(String::from("computer sec")));

    println!("{:#?}", course_instance.clone());
    // tested : worked
    // println!("{:#?}",course_instance.get_courses_list().await);

    // tested : worked
    // println!("{:#?}", course_instance.get_course_info().await);

    // test setter methods
    course_instance.set_department_name("electrical engineering");
    course_instance.set_course_name("statistics");
    println!("{:#?}", course_instance.clone());

    Ok(())

}