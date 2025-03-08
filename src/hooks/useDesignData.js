// Custom hook to manage design data
import { useState, useEffect } from 'react';
import { fetchDesignFromCurrentDir, openAndParseVerilogFiles } from '../services/verilogService';

export function useDesignData() {
  const [design, setDesign] = useState(null);
  const [loading, setLoading] = useState(true);

  // Load design from current directory on mount
  useEffect(() => {
    async function loadInitialDesign() {
      try {
        const data = await fetchDesignFromCurrentDir();
        setDesign(data);
      } catch (err) {
        console.error('Failed to load design:', err);
      } finally {
        setLoading(false);
      }
    }

    loadInitialDesign();
  }, []);

  // Function to handle file opening
  const handleFileOpen = async () => {
    try {
      setLoading(true);
      const data = await openAndParseVerilogFiles();
      if (data) {
        setDesign(data);
      }
    } catch (err) {
      console.error('Error in handleFileOpen:', err);
    } finally {
      setLoading(false);
    }
  };

  return { design, loading, handleFileOpen };
}
