use reqwest::Error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use tauri::Manager;
use tauri::State;

#[derive(Default)]
struct Job {
    description: String,
    embedding: Vec<f32>,
}

/// A project I did
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
struct Project {
    id: String,
    title: String,
    description: String,
    #[serde(default)]
    embedding: Vec<f32>,

    #[serde(default)]
    score: f32,
}

#[derive(Deserialize)]
struct EmbeddingResponse {
    embedding: Vec<f32>,
}

impl Job {
    /// To get embedding I use ollama with the model nomic-embed-text
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

/// Calculate the angle between the vectors
/// If the angle is small : the meaning is close
/// https://en.wikipedia.org/wiki/Cosine_similarity
fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let dot: f32 = a.iter().zip(b).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    dot / (norm_a * norm_b)
}

fn rank_project(mut projects: Vec<Project>, job: Job) -> Vec<Project> {
    for project in projects.iter_mut() {
        project.score = cosine_similarity(&job.embedding, &project.embedding);
    }

    projects.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    projects
}

/// This method is the main entry point from the ui
#[tauri::command]
fn handle_job_desc(job_desc: &str, projects: State<Vec<Project>>) -> String {
    let mut job = Job {
        description: job_desc.to_string(),
        ..Default::default()
    };
    job.get_embedding()
        .expect("Could not get embedding from job_desc");
    let ranked = rank_project(projects.inner().to_vec(), job);
    format!(
        "Top 3 projects !:\nTop 1: {}, with score: {}
        \nTop 2: {}, with score {}\nTop3: {}, with score: {}",
        ranked[0].id, ranked[0].score, ranked[1].id, ranked[1].score, ranked[2].id, ranked[2].score
    )
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
            app.manage(project_list);

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![handle_job_desc])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
