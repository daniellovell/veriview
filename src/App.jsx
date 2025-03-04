// src/App.jsx
import React, { useEffect, useRef, useState } from "react";
import "./App.css";

// In Tauri v2, `invoke` is from @tauri-apps/api/core
import { invoke } from "@tauri-apps/api/core";
// The dialog plugin has its own package: @tauri-apps/plugin-dialog
import { open as openDialog } from "@tauri-apps/plugin-dialog";

import cytoscape from "cytoscape";

// Helper: recursively flatten the instance tree into Cytoscape elements
function flattenDesign(design) {
  const elements = [];

  function traverse(instance, parentId = null) {
    // Create a unique ID for the instance
    const id = `${instance.instance_name}_${instance.module_type}`;
    elements.push({
      data: { id, label: `${instance.instance_name} (${instance.module_type})` },
    });

    if (parentId) {
      elements.push({
        data: { source: parentId, target: id, label: "instantiates" },
      });
    }

    if (instance.children && instance.children.length > 0) {
      instance.children.forEach((child) => traverse(child, id));
    }
  }

  design.top_instances.forEach((top) => traverse(top));
  return elements;
}

function App() {
  const [design, setDesign] = useState(null);
  const [loading, setLoading] = useState(true);
  const cyRef = useRef(null);
  const containerRef = useRef(null);

  // Opens a file dialog to pick Verilog files
  const handleFileOpen = async () => {
    try {
      // Open the Tauri v2 dialog
      const selected = await openDialog({
        multiple: true,
        filters: [
          { name: "Verilog Files", extensions: ["v", "sv"] },
        ],
      });

      // Check if user canceled or not
      if (selected) {
        setLoading(true);
        // The dialog can return a single path or array of paths
        const filePaths = Array.isArray(selected) ? selected : [selected];
        const data = await invoke("parse_verilog_files", { filePaths });
        setDesign(data);
      }
    } catch (err) {
      console.error("Error opening files:", err);
    } finally {
      setLoading(false);
    }
  };

  // On mount, parse all .v/.sv files in the current dir
  useEffect(() => {
    async function fetchDesign() {
      try {
        console.log("Fetching design from current directory...");
        const data = await invoke("parse_verilog_files", { filePaths: [] });
        console.log("Received design:", data);
        setDesign(data);
      } catch (err) {
        console.error("Error invoking parse_verilog_files:", err);
      } finally {
        setLoading(false);
      }
    }
    fetchDesign();
  }, []);

  // Whenever "design" changes, build the Cytoscape graph
  useEffect(() => {
    if (design && containerRef.current) {
      console.log("Initializing Cytoscape with elements:", flattenDesign(design));
      const elements = flattenDesign(design);

      // Destroy old instance if it exists
      if (cyRef.current) {
        cyRef.current.destroy();
      }

      cyRef.current = cytoscape({
        container: containerRef.current,
        elements,
        style: [
          {
            selector: "node",
            style: {
              label: "data(label)",
              "text-valign": "center",
              "text-halign": "center",
              "font-size": "14px",
              "font-weight": "normal",
              "font-family": "system-ui, sans-serif",
              "background-color": "#4a7ba6",
              color: "#ffffff",
              "text-outline-width": 0,
              "text-max-width": "180px",
              "text-wrap": "wrap",
              width: "label",
              height: "label",
              shape: "rectangle",
              "border-width": 2,
              "border-color": "#2b3a4a",
              padding: "15px",
            },
          },
          {
            selector: "edge",
            style: {
              label: "data(label)",
              "curve-style": "bezier",
              width: 2,
              "line-color": "#6c757d",
              "target-arrow-shape": "triangle",
              "target-arrow-color": "#6c757d",
              "font-size": "12px",
              color: "#333333",
              "text-background-color": "#ffffff",
              "text-background-opacity": 0.7,
              "text-background-padding": "3px",
              "edge-text-rotation": "autorotate",
            },
          },
        ],
        layout: {
          name: "breadthfirst",
          directed: true,
          padding: 30,
          spacingFactor: 1.5,
          rankDir: "TB",
        },
      });

      cyRef.current.on('tap', 'node', function(evt){
        const node = evt.target;
        cyRef.current.fit(node, 50);
      });
    }
  }, [design]);

  return (
    <div className="app-container">
      {/* Top bar with button to open files - always visible */}
      <div className="top-bar">
        <h3 className="app-title">Verilog Design Viewer</h3>
        <button className="action-button" onClick={handleFileOpen}>
          Open Verilog Files
        </button>
      </div>

      {/* Loading indicator or no design message */}
      {(!design || (design && design.top_instances.length === 0)) && (
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
      )}

      {/* Always render the container, but it will be empty if no design */}
      <div ref={containerRef} className="graph-container" />
    </div>
  );
}

export default App;

