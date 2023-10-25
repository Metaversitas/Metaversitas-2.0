import React, { useState } from 'react';
// import logo from './logo.svg';
import './App.css';
import { Login } from "./components/Login";
import { Register } from "./components/Register";


function App() {
  const [currentForm, setCurrentForm] = useState('login');

  const toogleForm = (formName) => {
  setCurrentForm(formName);
}

  return (
    <div className="App">
    {
      currentForm === "login" ? <Login onFormSwitch={toogleForm} /> : <Register onFormSwitch={toogleForm}  />
    }
    </div>
  );
}

export default App;
