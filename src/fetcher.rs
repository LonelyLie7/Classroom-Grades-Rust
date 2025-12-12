// this file contains the functions that makes requests to the Classroom API

use serde::{Deserialize, de::DeserializeOwned};

use crate::auth;

// ------- URL´S CONSTANTS -------
const COURSES: &str = "https://classroom.googleapis.com/v1/courses";
const COURSE_WORK: &str = "courseWork";
const STUDENT_SUBMISSIONS: &str = "studentSubmissions";
const STUDENTS: &str = "students";

// ------- ENUMS FOR JSON -------
#[derive(Debug, Deserialize, PartialEq)]
enum Status {
    ACTIVE,
    ARCHIVED,
    PUBLISHED,
}

// ------- STRUCTS FOR JSON -------
#[derive(Debug, Deserialize)]
struct CoursesContainer {
    courses: Vec<Course>,
}

#[derive(Debug, Deserialize)]
pub struct Course {
    pub id: String,
    pub name: String,
    #[serde(rename = "courseState")]
    course_state: Status,
}

#[derive(Debug, Deserialize)]
struct CourseWork {
    #[serde(rename = "courseWork")]
    course_work: Vec<Assignment>,
}

// possible upgrade: add topic field in order to filter assignations by topic
#[derive(Debug, Deserialize)]
pub struct Assignment {
    pub id: String,
    pub title: String,
    // #[serde(rename = "state")]
    // assignment_state: Status,
    // #[serde(rename = "topicId")]
    // topic_id: String,
}

#[derive(Debug, Deserialize)]
struct SubmissionsContainer {
    #[serde(rename = "studentSubmissions")]
    submissions: Vec<StudentSubmission>,
}

#[derive(Debug, Deserialize)]
pub struct StudentSubmission {
    //pub id: String,
    #[serde(rename = "courseWorkId")]
    pub assignment_id: String,
    #[serde(rename = "userId")]
    pub student_id: String,
    #[serde(rename = "assignedGrade")]
    pub assigned_grade: Option<i8>,
}

#[derive(Debug, Deserialize)]
pub struct StudentsContainer {
    students: Vec<Student>,
    #[serde(rename = "nextPageToken")]
    next_page: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Student {
    #[serde(rename = "userId")]
    pub user_id: String,
    pub profile: Profile,
}

#[derive(Debug, Deserialize)]
pub struct Profile {
    //pub id: String,
    pub name: Name,
}

#[derive(Debug, Deserialize)]
pub struct Name {
    #[serde(rename = "givenName")]
    pub given_name: String,
    #[serde(rename = "familyName")]
    pub family_name: String, 
}

// ------- FUNCTIONS TO GET DATA FROM THE API -------
// get active courses
pub async fn active_courses() -> Result<Vec<Course>, reqwest::Error> {
    let url = COURSES;
    let courses_container = base_request::<CoursesContainer>(&url).await;

    if courses_container.is_ok() { 
        Ok(courses_container.unwrap()
            .courses.into_iter()
            .filter(|course| course.course_state.eq(&Status::ACTIVE))
            .collect())
    } else {
        Err(courses_container.err().unwrap())
    }
}

// get assignments inside a course by id
pub async fn course_work(course_id: &str) -> Result<Vec<Assignment>, reqwest::Error> {
    let url = format!("{}/{}/{}", 
        COURSES, 
        course_id, 
        COURSE_WORK
    );

    let course_work = base_request::<CourseWork>( &url).await;

    if course_work.is_ok() {
        Ok(course_work.unwrap().course_work)
    } else {
        Err(course_work.err().unwrap())
    }
}

// get student submissions to an assignment by id
pub async fn student_submissions(course_id: &str, assignment_id: &str) -> Result<Vec<StudentSubmission>, reqwest::Error> {
    let url = format!("{}/{}/{}/{}/{}", 
        COURSES, 
        course_id, 
        COURSE_WORK, 
        assignment_id, 
        STUDENT_SUBMISSIONS
    );
    
    let submissions_container = base_request::<SubmissionsContainer>(&url).await;

    if submissions_container.is_ok() {
        Ok(submissions_container.unwrap().submissions)
    } else {
        Err(submissions_container.err().unwrap())
    }
}

// get the students list of a course by id (it is paginated)
pub async fn students_list(course_id: &str) -> Result<Vec<Student>, reqwest::Error> {
    let token = auth::create_token().await;
    let client = reqwest::Client::new();
    // vec for all the objects in the request
    let mut all_students: Vec<Student> = Vec::new();
    // we start without a page token
    let mut page_token: Option<String> = None;
    // url of the paginated endpoint
    let url = format!("{}/{}/{}",
        COURSES,
        course_id,
        STUDENTS
    );

    loop {
        let mut url_params: Vec<(&str, &str)> = Vec::new(); // for page token and other params
        // if we have a next page
        if let Some(token) = &page_token {
            url_params.push(("pageToken", &token)); // Add the page token if exists
        }

        let response = client.get(&url) // endpoint url
        .bearer_auth(&token.token().unwrap()) // auth headers
        .query(&url_params) // limit of objects
        .send()
        .await;
        
        if response.is_ok() {
            let current_body = response
                .unwrap()
                .json()
                .await;

            if current_body.is_ok() {
                let current_list: StudentsContainer = current_body.unwrap(); // get the objects in the current page
                // println!("Fetched {} students on this page.", current_list.students.len());
                all_students.extend(current_list.students); // add them to the list

                page_token = current_list.next_page;

                if page_token.is_none() { // if theres no other page
                    break; // we are done
                }
            } else {
                return Err(current_body.err().unwrap());
            }
        } else {
            return Err(response.err().unwrap());
        }
    }
    // println!("All Students: {}", all_students.len());
    Ok(all_students)
}

// base request for every "GET" operation to the API
async fn base_request<K: DeserializeOwned>(url: &str) -> Result<K, reqwest::Error> {
    let token = auth::create_token().await;
    let client = reqwest::Client::new();

    let response = client.get(url) // endpoint url
        .bearer_auth(&token.token().unwrap()) // auth headers
        .send()
        .await;

    if response.is_ok() {
        let body:Result<K, reqwest::Error> = response
            .unwrap()
            .json()
            .await;

        if body.is_ok() {
            Ok(body.unwrap())
        } else {
            Err(body.err().unwrap())
        }
    } else {
        Err(response.err().unwrap())
    }
}