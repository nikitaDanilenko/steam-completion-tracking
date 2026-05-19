import React, {useEffect} from 'react';

const Login: React.FC = () => {
  const handleSteamLogin = () => {
    // Redirect to the backend endpoint for initiating Steam OpenID login
    window.location.href = 'http://localhost:3000/auth/steam';
  };

  useEffect(() => {
    // Check for Steam login callback parameters in the URL
    const urlParams = new URLSearchParams(window.location.search);
    const steamToken = urlParams.get('steamToken');

    if (steamToken) {
      // Store the Steam token in local storage or a cookie for future API requests
      localStorage.setItem('steamToken', steamToken);
      // Optionally, redirect to another page after successful login
      window.location.href = '/';
    }
  }, []);

  return (
    <div className="login-container">
      <h1>Login</h1>
      <button className="steam-login-button" onClick={handleSteamLogin}>
        Login via Steam
      </button>
    </div>
  );
};

export default Login;
