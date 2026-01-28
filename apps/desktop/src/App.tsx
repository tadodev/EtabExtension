import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import type { Project } from '@etab-extension/shared/types';
import './App.css';

function App() {
  const [name, setName] = useState('');
  const [greetMsg, setGreetMsg] = useState('');
  const [projects, setProjects] = useState<Project[]>([]);

  async function greet() {
    const message = await invoke<string>('greet', { name });
    setGreetMsg(message);
  }

  async function createProject() {
    const project = await invoke<Project>('create_project', {
      name: 'My Project',
      description: 'Test project'
    });
    setProjects([...projects, project]);
  }

  async function loadProjects() {
    const allProjects = await invoke<Project[]>('get_projects');
    setProjects(allProjects);
  }

  return (
    <div className="container">
      <h1>ETAB Extension</h1>

      <div className="row">
        <input
          placeholder="Enter a name..."
          value={name}
          onChange={(e) => setName(e.target.value)}
        />
        <button onClick={greet}>Greet</button>
      </div>

      {greetMsg && <p>{greetMsg}</p>}

      <div className="row">
        <button onClick={createProject}>Create Project</button>
        <button onClick={loadProjects}>Load Projects</button>
      </div>

      <div>
        <h2>Projects ({projects.length})</h2>
        {projects.map((project) => (
          <div key={project.id} style={{ border: '1px solid #ccc', padding: '10px', margin: '10px 0' }}>
            <h3>{project.name}</h3>
            <p>{project.description}</p>
            <small>Created: {new Date(project.created_at).toLocaleString()}</small>
          </div>
        ))}
      </div>
    </div>
  );
}

export default App;