import React from 'react';
import { NavLink } from 'react-router-dom';

const Navigation: React.FC = () => {
  return (
    <nav className="nav">
      <div className="nav-container">
        <NavLink to="/" className="nav-brand">
          <span style={{
            background: 'linear-gradient(135deg, #2d3748 0%, #4a5568 100%)',
            WebkitBackgroundClip: 'text',
            WebkitTextFillColor: 'transparent',
            backgroundClip: 'text',
            fontWeight: '700',
            fontSize: '1.5rem'
          }}>
            Zklear
          </span>
        </NavLink>
        <div className="nav-links">
          <NavLink 
            to="/" 
            className={({ isActive }) => `nav-link ${isActive ? 'active' : ''}`}
          >
            Dashboard
          </NavLink>
          <NavLink 
            to="/api-spec" 
            className={({ isActive }) => `nav-link ${isActive ? 'active' : ''}`}
          >
            API Spec
          </NavLink>
        </div>
      </div>
    </nav>
  );
};

export default Navigation; 