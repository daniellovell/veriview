// src/App.jsx
import React, { useState } from 'react';
import Header from './components/Header/Header';
import EmptyState from './components/EmptyState/EmptyState';
import DesignViewer from './components/DesignViewer/DesignViewer';
import { useDesignData } from './hooks/useDesignData';
import './App.css';

function App() {
  const [viewMode, setViewMode] = useState('tree');
  const { design, loading, handleFileOpen } = useDesignData();

  // Toggle between view modes
  const toggleViewMode = () => {
    setViewMode((prevMode) => (prevMode === 'tree' ? 'nested' : 'tree'));
  };

  // Check if we have a design with modules
  const hasDesign = design && design.top_instances && design.top_instances.length > 0;

  return (
    <div className="app-container">
      <Header
        viewMode={viewMode}
        toggleViewMode={toggleViewMode}
        handleFileOpen={handleFileOpen}
        hasDesign={hasDesign}
      />

      {!hasDesign && <EmptyState loading={loading} />}

      <DesignViewer design={design} viewMode={viewMode} />
    </div>
  );
}

export default App;
