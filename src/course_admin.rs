// import for authentication
use std::collections::HashMap;

// import for http request
use crate::{csv_writer, fetcher::{self, Assignment, Course, Student}};

// main course object
#[derive(Debug)]
pub struct CourseAdmin {
    pub course: Course,
    pub students: Vec<Student>,
    pub assignments: Vec<Assignment>,
    pub submissions: HashMap<(String, String), i8>,
}

// this struct is responsible for managing data from the API to its parent thread 
impl CourseAdmin {
    // counstructor
    // recieves an individual course basic info
    pub fn new(course: Course) -> Self {
        Self {
            // basic course info
            course,
            // empty vecs for the lists of objects inside the course
            students: Vec::new(),
            assignments: Vec::new(),
            submissions: HashMap::new(),
        }
    }

    // load students from the API based on the course ID
    async fn load_students(&mut self) -> Result<(), reqwest::Error> {
        // get the students list from the API
        let students_list = fetcher::
            students_list(
                &self.course.id
            ).await;
        
        // set it to the field in the CourseAdmin struct
        if students_list.is_ok() {
            self.students = students_list.unwrap();
            println!("[ÉXITO]: Curso - {}: Alumnos cargados exitosamente", self.course.name);
            Ok(())
        } else {
            println!("[ERROR]: Curso - {}: No fue posible cargar los alumnos", self.course.name);
            Err(students_list.err().unwrap())
        }
    }

    // load assignments from the API based on course ID
    async fn load_assignments(&mut self) -> Result<(), reqwest::Error> {
        // get the assignments list from the API
        let assignments_list = fetcher::
            course_work(&self.course.id).await;
        
        // set it to the field in the CourseAdmin struct
        if assignments_list.is_ok() {
            self.assignments = assignments_list.unwrap();
            println!("[ÉXITO]: Curso - {}: Asignaciones cargadas exitosamente", self.course.name);
            Ok(())
        } else {
            println!("[ERROR]: Curso - {}: No fue posible cargar las asignaciones", self.course.name);
            Err(assignments_list.err().unwrap())
        }
    }

    // load the submissions from the API based on assignments list (which means that is not possible to load assignments if the assignments list is empty)
    async fn load_submissions(&mut self) -> Result<(), reqwest::Error> {
        if !self.assignments.is_empty() {
            // load all the submissions of the course no matter the assignment
            // transform the submissions to <(student_id, assignment_id), assigned_grade> objects and store them in a HashMap where the tuple with the ID's is the key
            // this will allow us to organize them when we're gonna persist them into the CSV file.
            for assignment in &self.assignments {
                let submissions_result = fetcher::student_submissions(
                        &self.course.id,
                        &assignment.id)
                    .await;

                if submissions_result.is_ok() {
                    let current_submissions: HashMap<(String, String), i8> = submissions_result
                        .unwrap()
                        .into_iter()
                        .map(|sbmsn| (
                            (sbmsn.student_id, sbmsn.assignment_id), 
                            sbmsn.assigned_grade.unwrap_or(-1)))
                        .collect();
                    self.submissions.extend(current_submissions);
                println!("[ÉXITO]: Asignación - {}: Entregas cargadas exitosamente", assignment.title);
                } else {
                    println!("[ERROR]: Asignación - {}: No fue posible cargar las entregas", assignment.title);
                    return Err(submissions_result.err().unwrap());
                }
            }
        }
        Ok(())
    }

    // main function that loads data from the API to the struct fileds
    async fn load_data(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // load data to its place in the struct
        let students_load_result = self.load_students().await;
        let assignments_load_result = self.load_assignments().await;
        
        if students_load_result.is_ok() && assignments_load_result.is_ok() {
            // sort students by family_name
            self.students.sort_by(|a, b| {
                a.profile.name.family_name.cmp(&b.profile.name.family_name)
            });
            //sort assingments by title
            self.assignments.sort_by(|a, b| {
                a.title.cmp(&b.title)
            });
            println!("[ÉXITO]: Curso - {}: Datos ordenados exitosamente", self.course.name);
            println!("\t[INFO]: Estudiantes: {}", self.students.len());
            println!("\t[INFO]: Asignaciones: {}", self.assignments.len());

            // load submissions
            self.load_submissions().await?;
            println!("\t[INFO]: Entregas: {}", self.submissions.len());
        }

        Ok(())
    }

    // main function that persists the course scores to the CSV file
    pub async fn persist_scores(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        //load data to the struct
        self.load_data().await?;

        // create the CSV file based on course name and assignment names and propagate errors
        let current_course_name = &self.course.name;
        csv_writer::create_file_with_header(
            &current_course_name,
            &self.assignments.iter()
                .map(|a| 
                    a.title.as_str()
                ).collect::<Vec<&str>>() 
        ).await?;
        
        // reference for practicity and legibility
        let submisions = &self.submissions;

        // build an individual line with the scores for every student
        for student in &self.students {
            let name = &student.profile.name;
            let mut line = format!("{} {}", name.family_name, name.given_name);
            for assignment in &self.assignments {
                let score = submisions.get(&(student.user_id.clone(), assignment.id.clone()));
                line += format!(",{}", score.unwrap_or(&0)).as_str();
            }

            // append the line in the previously created CSV scores file and propagate errors
            csv_writer::append_line(&current_course_name, &line).await?;
        }

        Ok(())
    }
}