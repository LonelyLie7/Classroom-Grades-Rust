use std::path::{Path, PathBuf};

use tokio::{fs::{self, OpenOptions}, io::AsyncWriteExt};

// make the filename path based on course name
fn filename_path(course_name: &str) -> PathBuf {
    Path::new("grades")
        .join(format!("{}.csv",course_name))
}

// create a CSV file based on the course name
pub async fn create_file_with_header(course_name: &str, assignment_names: &[&str]) -> Result<(), std::io::Error> {
    // build header
    let mut header = "ALUMNO".to_string();
    for name in assignment_names {
        header.push_str(&format!(",{}", name));
    }
    header += "\n";
    // create CSV file and parent dirs
    let filename_path = filename_path(course_name);
    if let Some(parent) = filename_path.parent() {
        fs::create_dir_all(parent).await?;
    }
    // write header
    fs::write(filename_path,header).await
}

// append a line in the CSV file
// create the file and write inside if it doesnt exist
pub async fn append_line(course_name: &str, line: &str) -> Result<(), Box<dyn std::error::Error>> {
    let filename_path = filename_path(course_name);
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .write(true)
        .open(filename_path)
        .await?;

    let line_with_newline = format!("{}\n", line);
    file.write_all(line_with_newline.as_bytes()).await?;

    Ok(())
}