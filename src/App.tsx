import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [cv, setCV] = useState("");
  const [jobDesc, setJobDesc] = useState("");

  async function get_cv() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    setCV(await invoke("handle_job_desc", { jobDesc }));
  }

  return (
    <main className="container">
      <h1>Welcome to CV tweaker</h1>

      <p>Insert an job or internship description, and get a better CV !</p>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          get_cv()
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setJobDesc(e.currentTarget.value)}
          placeholder="Enter the job description"
        />
        <button type="submit">Get a tweaked CV</button>
      </form>
      <p>{cv}</p>
    </main>
  );
}

export default App;
