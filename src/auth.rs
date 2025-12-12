use yup_oauth2::{
    AccessToken, Error,
    InstalledFlowAuthenticator, InstalledFlowReturnMethod,
    read_application_secret
}; // OAuth 2.0 for authentication on Google Classroom API
use dirs; // access to the filesystem in order to find the secret client
use std::path::PathBuf; // for filesystem access

// constants for scopes
// the scope define de permission level for the auth
// we define the scope as readonly because we only need to get data, not make modifications
pub const COURSES_READONLY: &str = "https://www.googleapis.com/auth/classroom.courses.readonly";
pub const SUBMISSIONS_READONLY: &str = "https://www.googleapis.com/auth/classroom.student-submissions.students.readonly";
pub const ROSTERS_READONLY: &str = "https://www.googleapis.com/auth/classroom.rosters.readonly";

// gets the secrent client filepath to read it from the authenticator
fn get_secret_path() -> Option<PathBuf> {
    if let Some(mut config_dir) = dirs::config_dir() { // access to .config dir
        config_dir.push("classroom-rust"); // classroom-rust folder
        config_dir.push("cs_classroom-rust.json"); // client secret name
        Some(config_dir)
    } else {
        None
    }
}

async fn authenticate(scopes: &[&str]) -> Result<AccessToken, Error> {
    // Read application secret from the file
    let secret_file_path = get_secret_path().unwrap();
    let client_secret = read_application_secret(secret_file_path).await.unwrap();
    // Create an authenticator that uses an InstalledFlow to authenticate using the browser.
    // It gets a temporary token and stores it in a cache file and replaces it when expired.
    let auth = InstalledFlowAuthenticator::builder(client_secret, InstalledFlowReturnMethod::HTTPRedirect)
        .persist_tokens_to_disk("tokencache.json")
        .build()
        .await
        .unwrap();

    // We authenticate the user. This call will:
    //   a. Check if valid tokens exist in 'tokens.json'.
    //   b. If not, open a browser for the user to grant consent.
    //   c. Capture the authorization code.
    //   d. Exchange the code for access and refresh tokens.
    //   e. Store the new tokens in 'tokens.json'.
    auth.token(scopes).await
}

// authenticate in the API and get the token
pub async fn create_token() -> AccessToken {
    authenticate(&[
        COURSES_READONLY,
        SUBMISSIONS_READONLY,
        ROSTERS_READONLY,
    ]).await.unwrap()
}