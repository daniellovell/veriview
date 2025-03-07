// Transform design data for different visualization modes

// Create a tree view with edges between parent and child nodes
export function flattenDesign(design) {
  if (!design || !design.top_instances) return [];

  const elements = [];

  function traverse(instance, parentId = null) {
    // Create a unique ID for the instance
    const id = `${instance.instance_name}_${instance.module_type}`;
    elements.push({
      data: {
        id,
        label: `${instance.instance_name} (${instance.module_type})`,
        moduleType: instance.module_type,
        instanceName: instance.instance_name,
      },
    });

    if (parentId) {
      elements.push({
        data: { source: parentId, target: id, label: 'instantiates' },
      });
    }

    if (instance.children && instance.children.length > 0) {
      instance.children.forEach((child) => traverse(child, id));
    }
  }

  design.top_instances.forEach((top) => traverse(top));
  return elements;
}

// Create nested compound nodes for containment view
export function createNestedDesign(design) {
  if (!design || !design.top_instances) return [];

  const elements = [];

  function traverse(instance, parentId = null) {
    const id = `${instance.instance_name}_${instance.module_type}`;

    elements.push({
      data: {
        id,
        label: `${instance.instance_name} (${instance.module_type})`,
        moduleType: instance.module_type,
        instanceName: instance.instance_name,
        parent: parentId,
      },
    });

    if (instance.children && instance.children.length > 0) {
      instance.children.forEach((child) => traverse(child, id));
    }
  }

  design.top_instances.forEach((top) => traverse(top));
  return elements;
}
