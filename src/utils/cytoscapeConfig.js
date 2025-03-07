// Cytoscape configuration and styles

// Base node style
export const baseNodeStyle = {
  label: 'data(label)',
  'text-valign': 'center',
  'text-halign': 'center',
  'font-size': '14px',
  'font-weight': 'normal',
  'font-family': 'system-ui, sans-serif',
  'background-color': '#4a7ba6',
  color: '#ffffff',
  'text-outline-width': 0,
  'text-max-width': '180px',
  'text-wrap': 'wrap',
  width: 180,
  height: 80,
  shape: 'rectangle',
  'border-width': 2,
  'border-color': '#2b3a4a',
  padding: '15px',
};

// Parent node style for nested view
export const parentNodeStyle = {
  'text-valign': 'top',
  'text-halign': 'center',
  'background-opacity': 0.8,
  'background-color': '#2b3a4a',
  'border-width': 3,
  'border-color': '#1a2430',
  padding: 60,
  shape: 'rectangle',
};

// Edge style
export const edgeStyle = {
  label: 'data(label)',
  'curve-style': 'bezier',
  width: 2,
  'line-color': '#6c757d',
  'target-arrow-shape': 'triangle',
  'target-arrow-color': '#6c757d',
  'font-size': '12px',
  color: '#333333',
  'text-background-color': '#ffffff',
  'text-background-opacity': 0.7,
  'text-background-padding': '3px',
  'edge-text-rotation': 'autorotate',
};

// Layout configurations
export const getLayoutConfig = (viewMode) => {
  return viewMode === 'tree'
    ? {
        name: 'breadthfirst',
        directed: true,
        padding: 30,
        spacingFactor: 1.5,
        rankDir: 'TB',
      }
    : {
        name: 'fcose',
        quality: 'default',
        randomize: true,
        animate: true,
        animationDuration: 200,
        fit: true,
        padding: 40,
        nodeDimensionsIncludeLabels: true,
        uniformNodeDimensions: false,
        packComponents: true,
        samplingType: true,
        sampleSize: 25,
        nodeSeparation: 150,
        nodeRepulsion: 6000,
        idealEdgeLength: 150,
        edgeElasticity: 0.45,
        nestingFactor: 0.1,
        numIter: 2500,
        tile: true,
        gravity: 0.25,
        gravityRangeCompound: 3.0,
        gravityCompound: 2.0,
        gravityRange: 4.0,
        initialEnergyOnIncremental: 0.5,
      };
};
