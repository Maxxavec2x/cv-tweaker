# CV-Tweaker

A CV tweaking app using Tauri 

## How does it work:

- The list of your project is parsed, and we get a vector embedding of them using ollama nomic-embed-text model. (Done 1 time at startup)
- When you paste a job offer in the gui, we create a vector embedding of it and we compare it to the vectors of your projects. Then, we rank them to determine the best N project.
- We then parse your CV and change the boilerplates to the best projects for that specific job offer.

