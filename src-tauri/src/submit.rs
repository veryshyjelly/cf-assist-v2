use std::sync::Mutex;

use serde::{Deserialize, Serialize};
use tauri_plugin_http::reqwest;

use crate::{utils::ResultTrait, AppState};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Solution {
    empty: bool,
    problem_name: String,
    url: String,
    source_code: String,
    file_name: String,
    language_id: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmptySolution {
    pub empty: bool,
}

#[tauri::command]
pub async fn submit_solution(app_state: tauri::State<'_, Mutex<AppState>>) -> Result<(), String> {
    let state = app_state.lock().unwrap().clone();
    let source_code = state
        .config
        .get_final_code(&state.problem, &state.directory)?;
    let client = reqwest::Client::builder().build().map_to_string()?;

    let problem_name = state
        .problem
        .url
        .split('/')
        .rev()
        .take(2)
        .collect::<Vec<&str>>()
        .into_iter()
        .rev()
        .collect::<Vec<&str>>()
        .join("");

    let solution = Solution {
        empty: false,
        language_id: state.get_language().map_to_string()?.cf_id,
        problem_name,
        source_code,
        file_name: state.get_language().map_to_string()?.source_file,
        url: state.problem.url,
    };

    let post_request = client
        .post("http://localhost:27121/submit")
        .json(&solution)
        .build()
        .map_to_string()?;

    client.execute(post_request).await.map_to_string()?;

    Ok(())
}
