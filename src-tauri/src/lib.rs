use reqwest::Error;
use serde::{Deserialize, Serialize};
use serde_json::json;

use std::collections::HashMap;
struct Job {
    desc: String,
    keywords: String,
}

/// A project I did
#[derive(Serialize, Deserialize, Debug, Default)]
struct Project {
    id: String,
    title: String,
    description: String,
    #[serde(default)]
    embedding: Vec<f32>,
}

#[derive(Deserialize)]
struct EmbeddingResponse {
    embedding: Vec<f32>,
}

impl Project {
    fn get_embedding(&mut self) -> Result<(), Error> {
        let mut map = HashMap::new();
        map.insert("model", "nomic-embed-text");
        map.insert("prompt", &self.description);

        let client = reqwest::blocking::Client::new();
        let res = client
            .post("http://localhost:11434/api/embeddings")
            .json(&map)
            .send()?;
        let data: EmbeddingResponse = res.json::<EmbeddingResponse>()?;
        self.embedding = data.embedding;
        Ok(())
    }
}

fn load_projects() -> Result<Vec<Project>, serde_json::Error> {
    // For now its hardcoded, later I will load it from a file
    let json = json!([{
        "id": "projet1",
        "title": "CV-tweaker",
        "description": "Development of a CV-tweaking app using Rust (Tauri), a React JS frontend and ollama embedding model nomic-embed-text",
    }]);

    let projects: Vec<Project> = serde_json::from_value(json)?;

    Ok(projects)
}

#[tauri::command]
fn handle_job_desc(job_desc: &str) -> String {
    job_desc.to_string()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut project_list = load_projects().expect("expected a serializable json file for project");
    for project in project_list.iter_mut() {
        if let Err(e) = project.get_embedding() {
            panic!("Could not get embedding for project {:?}: {}", project, e);
        }
    }

    println!("PROJECT LIST AFTER EMBEDDING : {:?}", project_list);

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![handle_job_desc])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
