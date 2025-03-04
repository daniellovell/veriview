// Component for empty states and loading
import React from 'react';
import './EmptyState.css';

function EmptyState({ loading }) {
  return (
    <div className="empty-state">
      {loading ? (
        <p className="loading-text">Loading design...</p>
      ) : (
        <>
          <p className="empty-state-title">No Verilog modules found</p>
          <p className="empty-state-subtitle">Open Verilog files to visualize the design hierarchy</p>
        </>
      )}
    </div>
  );
}

export default EmptyState; 