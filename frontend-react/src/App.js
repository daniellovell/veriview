import React, { useEffect, useState, useRef } from 'react';
import cytoscape from 'cytoscape';

function App() {
  const [netlist, setNetlist] = useState(null);
  const cyRef = useRef(null);
  const containerRef = useRef(null);

  // Fetch the netlist JSON from the backend server.
  useEffect(() => {
    fetch('http://localhost:18080/netlist')
      .then(response => response.json())
      .then(data => setNetlist(data))
      .catch(err => console.error('Error fetching netlist:', err));
  }, []);

  // When netlist is loaded, initialize the Cytoscape graph.
  useEffect(() => {
    if (netlist && containerRef.current) {
      const elements = [];

      // Create a node for each module.
      netlist.modules.forEach((module) => {
        elements.push({
          data: { id: module.name, label: module.name, complexity: module.complexity }
        });

        // For each submodule instantiation, add an instance node and edges.
        if (module.submodules && module.submodules.length > 0) {
          module.submodules.forEach(inst => {
            const instanceId = `${module.name}_${inst.instanceName}`;
            // Create a node for the instance.
            elements.push({
              data: { id: instanceId, label: inst.instanceName }
            });
            // Edge from parent module to the instance.
            elements.push({
              data: { source: module.name, target: instanceId, label: "instantiates" }
            });
            // Edge from the instance node to the module type.
            elements.push({
              data: { source: instanceId, target: inst.moduleName, label: "module" }
            });
          });
        }
      });

      // Initialize Cytoscape.
      cyRef.current = cytoscape({
        container: containerRef.current,
        elements: elements,
        style: [
          {
            selector: 'node',
            style: {
              'label': 'data(label)',
              'background-color': '#666',
              'text-valign': 'center',
              'text-halign': 'center',
              'width': '50',
              'height': '50',
              'font-size': '12px',
              'color': '#fff'
            }
          },
          {
            selector: 'edge',
            style: {
              'label': 'data(label)',
              'curve-style': 'bezier',
              'target-arrow-shape': 'triangle',
              'line-color': '#ccc',
              'target-arrow-color': '#ccc',
              'width': 2,
              'font-size': '10px'
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
