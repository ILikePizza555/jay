// Not-code imports
import logo from "./logo.svg";
import "./App.css";
import "bootstrap/dist/css/bootstrap.min.css";

// Code imports
import React from "react";
import Button from "react-bootstrap/Button";

function App() {
    return (
        <div className="App">
            <header className="App-header">
                <img src={logo} className="App-logo" alt="logo" />
                <p>
          Edit <code>src/App.tsx</code> and save to reload.
                </p>
                <a
                    className="App-link"
                    href="https://reactjs.org"
                    target="_blank"
                    rel="noopener noreferrer"
                >
          Learn React
                </a>
                <Button>This is a sematic ui button.</Button>
            </header>
        </div>
    );
}

export default App;
