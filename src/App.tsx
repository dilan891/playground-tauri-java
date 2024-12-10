import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import jsPDF from 'jspdf';
//import excel from 'excel4node';
import "./App.css";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  const testApi = async () => {
    fetch("http://localhost:8180/api/test", {
      method: "GET",

    }
    ).then((response) => response.text())
    .then((data) => {
      console.log(data);
      alert(data);
    })
    .catch((error) => {
      console.error('Error:', error);
      alert(error);
    });

  }

  const createExamplePdf = () => {
    const doc = new jsPDF();
    doc.text("Hello world!", 10, 10);
    doc.save("a4.pdf");
  }

  const createExcel = () => {
    //const wb = new excel.Workbook();
    //const ws = wb.addWorksheet('Sheet 1');
    //ws.cell(1, 1).string('Hello');
    //wb.write('Excel.xlsx');
  }

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    setGreetMsg(await invoke("greet", { name }));
  }

  return (
    <main className="container">
      <h1>Welcome to Tauri + React</h1>

      <div className="row">
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://reactjs.org" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <p>Click on the Tauri, Vite, and React logos to learn more.</p>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button type="submit">Greet</button>
      </form>
      <p>{greetMsg}</p>
      <button onClick={testApi}>LLama a java</button>
      <button onClick={createExamplePdf}>crear pdf</button>
      <button >crear excel</button>
    </main>
  );
}

export default App;
