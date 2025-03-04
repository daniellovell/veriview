// Header component with app title and controls
import React from 'react';
import './Header.css';

function Header({ viewMode, toggleViewMode, handleFileOpen, hasDesign }) {
  return (
    <div className="top-bar">
      <h3 className="app-title">Verilog Design Viewer</h3>
      <div className="controls">
        {hasDesign && (
          <button 
            className="view-toggle-button" 
            onClick={toggleViewMode}
          >
            {viewMode === 'tree' ? 'Switch to Nested View' : 'Switch to Tree View'}
          </button>
        )}
        <button className="action-button" onClick={handleFileOpen}>
          Open Verilog Files
        </button>
      </div>
    </div>
  );
}

export default Header; 