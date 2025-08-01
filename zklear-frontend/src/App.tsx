import React from 'react';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import './App.css';
import Dashboard from './components/Dashboard';
import ApiSpec from './components/ApiSpec';
import Navigation from './components/Navigation';

function App() {
  return (
    <Router>
      <div className="App">
        <Navigation />
        <div className="content">
          <Routes>
            <Route path="/" element={<Dashboard />} />
            <Route path="/api-spec" element={<ApiSpec />} />
          </Routes>
        </div>
      </div>
    </Router>
  );
}

export default App;
