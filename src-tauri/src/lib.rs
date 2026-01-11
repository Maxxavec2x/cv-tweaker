use reqwest::Error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use tauri::Manager;
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

fn load_projects<P: AsRef<Path>>(path: P) -> Result<Vec<Project>, serde_json::Error> {
    let file = File::open(path).expect("expected an accessible file path for projects");
    let reader = BufReader::new(file);
    let projects: Vec<Project> = serde_json::from_reader(reader)?;

    Ok(projects)
}

#[tauri::command]
fn handle_job_desc(job_desc: &str) -> String {
    job_desc.to_string()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let path = app
                .path()
                .resolve("projects.json", tauri::path::BaseDirectory::Resource)?;

            let mut project_list =
                load_projects(path).expect("expected a serializable json file for projects");

            println!("PROJECT LIST BEFORE EMBEDDING : {:?}", project_list);
            for project in project_list.iter_mut() {
                project
                    .get_embedding()
                    .expect("Could not get embedding for project");
            }

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![handle_job_desc])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
