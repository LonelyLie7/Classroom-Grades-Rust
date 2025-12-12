mod auth;
mod fetcher;
mod csv_writer;
mod course_admin;

use std::sync::Arc;

use course_admin::CourseAdmin;
use tokio::task::JoinHandle;

// main async thread
#[tokio::main]
async fn main() {
    // get active courses
    let active_courses = fetcher::active_courses().await.unwrap();
    // create an instance of CourseAdmin for every course
    let mut course_admins: Vec<CourseAdmin> = Vec::new(); 
    for course in active_courses {
        course_admins.push(CourseAdmin::new(course));
    }

    // STARTS CONCURRENT PROCESSING
    // create a semaphore in order to limit concurrence on 8
    let semaphore = Arc::new(tokio::sync::Semaphore::new(8));
    let mut handles: Vec<JoinHandle<()>> = Vec::new();

    for mut admin in course_admins {
        let semaphore = Arc::clone(&semaphore);
        
        let handle = tokio::spawn(async move {
            // acquire semaphore permit (max 8 concurrent courses)
            let _permit = semaphore.acquire().await.expect("Semáforo cerrado");
            
            let course_name = admin.course.name.clone();
            println!("\n[CURSO INICIADO]: {}\n", course_name);
            
            match admin.persist_scores().await {
                Ok(_) => {
                    println!("\n[CURSO FINALIZADO]: {}\n", course_name);
                }
                Err(e) => {
                    eprintln!("[ERROR CURSO {}]: {}", course_name, e);
                }
            }
        });
        
        handles.push(handle);
    }

    // wait for all the task to finish
    for handle in handles {
        // we use `let _ =` to ignore join errors (they may ocur if task panic)
        let _ = handle.await;
    }

/*
    print!("Calificaciones:\nNOMBRE COMPLETO");
    for assignment in &current_admin.assignments {
        print!(",{}", &assignment.title.to_uppercase());
    }
    println!();
    let submisions = &current_admin.submissions;
    for student in &current_admin.students {
        let name = &student.profile.name;
        print!("{} {}", name.family_name, name.given_name);
        for assignment in &current_admin.assignments {
            let score = submisions.get(&(student.user_id.clone(), assignment.id.clone()));
            print!(",{}", score.unwrap_or(&0));
        }
        println!();
    } */
    
    //println!("{:?}", course_admins[0]);

    /*
    println!("{}: {}", course.id, course.name);
    let course_work = fetcher::course_work(&course.id)
        .await.unwrap();
    // create a reference vector with the references names of the assignments
    /*let assignment_names = &course_work.iter()
        .map(|assignment| assignment.title.as_str())
        .collect::<Vec<&str>>();
    // create the CSV file for the course
    let result = csv_writer::create_file_with_header(&course.name, &assignment_names).await;
    if result.is_ok() {
        println!("Archivo: /scores/{}.csv creado exitosamente.", &course.name);
    } else {
        println!("No fue posible crear archivo: /scores/{}.csv\nError: {}", &course.name, result.err().unwrap());
    }*/

    // get the student submissions for every assignment and store them in a Vec<Vec<Assignment>>
    let mut all_the_submissions: Vec<fetcher::StudentSubmission> = Vec::new();
    for assignment in course_work {
        let current_submissions = fetcher::student_submissions(&course.id, &assignment.id).await.unwrap();
        all_the_submissions.extend(current_submissions);
    }
    
    for submission in all_the_submissions {
        println!("{:?}", submission);
    }
    
    let students = fetcher::students_list(&course.id).await.unwrap();
    println!("{}", students.len());
    for student in students {
        let name = student.profile.name;
        let line = format!("{},{},{}", student.user_id, name.family_name, name.given_name);
        println!("{}", line);
        /*if csv_writer::write_line("prueba.csv", &line).await.is_ok(){
            println!("Linea escrita con éxito");
        } else {
            println!("Error al escribir la línea");
        }*/

    }
     */
}