import { useState, useEffect } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import LoginForm from "./components/LoginForm";
function App() {
  const [loggedIn, setLoggedIn] = useState(false);


  // check if theres a user.json file in src-tauri if false then change button text in loginform
  const checkIfLoggedIn = async () => {
    setLoggedIn(await invoke("check_if_logged_in_before"));
  }

  useEffect(() => {
    checkIfLoggedIn();
  }, [])

  return (
    <div className="container">
      <LoginForm loggedIn={loggedIn}/>
    </div>
  );
}

export default App;
