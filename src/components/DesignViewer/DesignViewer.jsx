// Main component for visualizing the design
import React, { useEffect, useRef } from 'react';
import cytoscape from 'cytoscape';
import fcose from 'cytoscape-fcose';
import { flattenDesign, createNestedDesign } from '../../utils/designTransformers';
import { baseNodeStyle, parentNodeStyle, edgeStyle, getLayoutConfig } from '../../utils/cytoscapeConfig';
import './DesignViewer.css';

// Register the fcose layout
cytoscape.use(fcose);

function DesignViewer({ design, viewMode }) {
  const cyRef = useRef(null);
  const containerRef = useRef(null);

  // Initialize or update the graph when design or view mode changes
  useEffect(() => {
    if (!design || !containerRef.current) return;

    console.log(`Initializing Cytoscape with ${viewMode} view mode`);
    
    // Choose the appropriate elements based on view mode
    const elements = viewMode === 'tree' 
      ? flattenDesign(design) 
      : createNestedDesign(design);
    
    // Destroy old instance if it exists
    if (cyRef.current) {
      cyRef.current.destroy();
    }

    try {
      // Create new Cytoscape instance
      cyRef.current = cytoscape({
        container: containerRef.current,
        elements,
        style: [
          {
            selector: "node",
            style: baseNodeStyle
          },
          {
            selector: "node:parent",
            style: parentNodeStyle
          },
          {
            selector: "edge",
            style: edgeStyle
          },
        ],
        layout: getLayoutConfig(viewMode),
      });

      // Add interaction handlers
      cyRef.current.on('tap', 'node', function(evt) {
        const node = evt.target;
        // Don't zoom on parent nodes in nested view
        if (viewMode === 'nested' && node.isParent()) return;
        
        cyRef.current.fit(node, 50);
      });

      // Add a fit-all zoom control
      cyRef.current.on('tap', function(evt) {
        if (evt.target === cyRef.current) {
          cyRef.current.fit();
        }
      });
    } catch (error) {
      console.error("Error initializing Cytoscape:", error);
    }

    // Cleanup
    return () => {
      if (cyRef.current) {
        cyRef.current.destroy();
      }
    };
  }, [design, viewMode]);

  // Show mode description if design exists
  const hasDesign = design && design.top_instances && design.top_instances.length > 0;

  return (
    <>
      {hasDesign && (
        <div className="mode-description">
          <p>
            <strong>Current: {viewMode === 'tree' ? 'Tree View' : 'Nested View'}</strong> - 
            {viewMode === 'tree' 
              ? ' Shows module instantiations as a hierarchy tree' 
              : ' Shows modules contained inside their parent modules'}
          </p>
          {viewMode === 'nested' && (
            <p className="note">
              Note: IO connections between modules require backend parser enhancements
            </p>
          )}
        </div>
      )}
      <div ref={containerRef} className="graph-container" />
    </>
  );
}

export default DesignViewer; 