// src/App.js
import React, { useEffect, useState, useRef } from 'react';
import cytoscape from 'cytoscape';

function App() {
  const [netlist, setNetlist] = useState(null);
  const cyRef = useRef(null);
  const containerRef = useRef(null);

  // Fetch the netlist JSON from the backend server on port 8080
  useEffect(() => {
    fetch('http://localhost:8080/api/netlist')
      .then(response => response.json())
      .then(data => setNetlist(data))
      .catch(err => console.error('Error fetching netlist:', err));
  }, []);

  // When netlist is loaded, initialize the Cytoscape graph
  useEffect(() => {
    if (netlist && containerRef.current) {
      const elements = [];

      // Create a node for each module
      netlist.modules.forEach((module) => {
        // The parent module node
        elements.push({
          data: { id: module.name, label: module.name, complexity: module.complexity }
        });

        // For each submodule instantiation, add an instance node and edges
        if (module.submodules && module.submodules.length > 0) {
          module.submodules.forEach(inst => {
            const instanceId = `${module.name}_${inst.instanceName}`;

            // Node for the instance
            elements.push({
              data: { id: instanceId, label: inst.instanceName }
            });

            // Edge: parent module -> instance node
            elements.push({
              data: { source: module.name, target: instanceId, label: 'instantiates' }
            });

            // Edge: instance node -> actual module name
            elements.push({
              data: { source: instanceId, target: inst.moduleName, label: 'moduleOf' }
            });
          });
        }
      });

      // Initialize Cytoscape
      cyRef.current = cytoscape({
        container: containerRef.current,
        elements,
        style: [
          {
            selector: 'node',
            style: {
              'label': 'data(label)',
              'text-valign': 'center',
              'text-halign': 'center',
              'font-size': '12px',
              'background-color': '#666',
              'color': '#fff',
              'width': 40,
              'height': 40,
            }
          },
          {
            selector: 'edge',
            style: {
              'label': 'data(label)',
              'curve-style': 'bezier',
              'width': 2,
              'line-color': '#ccc',
              'target-arrow-shape': 'triangle',
              'target-arrow-color': '#ccc',
              'font-size': '10px',
            }
          }
        ],
        layout: {
          name: 'breadthfirst',
          directed: true,
          padding: 10
        }
      });
    }
  }, [netlist]);

  return (
    <div style={{ height: '100vh', width: '100%' }} ref={containerRef}>
      {!netlist && <p>Loading netlist...</p>}
    </div>
  );
}

export default App;
