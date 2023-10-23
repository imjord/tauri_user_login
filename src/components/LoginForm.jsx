import React, { useState } from 'react';
import './LoginForm.css';
import { invoke } from "@tauri-apps/api/tauri";

const LoginForm = ({loggedIn}) => {
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const [message, setMessage] = useState("");
  



  async function login() {
    setMessage(await invoke("sign_user_in", {username, password}));
  }

  return (
    <div className="login-form">
      {loggedIn ? <h2>Login</h2> : <h2>Sign up</h2>}
      <div className="input-container">
        <label htmlFor="username">Username</label>
        <input
          type="text"
          id="username"
          value={username}
          onChange={(e) => setUsername(e.target.value)}
        />
      </div>
      <div className="input-container">
        <label htmlFor="password">Password</label>
        <input
          type="password"
          id="password"
          value={password}
          onChange={(e) => setPassword(e.target.value)}
        />
      </div>
      {loggedIn ? <button onClick={login}>Login</button> : <button onClick={login}>Create Account</button>}
      <p className='message'>{message}</p>
    </div>
  );
};

export default LoginForm;