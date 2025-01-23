"use client";

import React, { useState, useEffect } from 'react';
import { fetchPublicItems, PublicItem } from '../../utils/api';
import ItemTable from './components/ItemList';
import SearchBar from './components/SearchBar';

const PublicItemsPage: React.FC = () => {
  const [items, setItems] = useState<PublicItem[]>([]);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const loadItems = async () => {
      try {
        const data = await fetchPublicItems();
        setItems(data);
      } catch (err) {
        setError('Failed to fetch items');
      } finally {
        setLoading(false);
      }
    };

    loadItems();
  }, []);

  const handleSearch = async (searchTerm: string) => {
    setLoading(true);
    try {
      const data = await fetchPublicItems({ search: searchTerm });
      setItems(data);
    } catch (err) {
      setError('Failed to fetch items');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="container mx-auto p-4">
      <h1 className="text-2xl font-bold mb-4">Public Items</h1>
      <SearchBar onSearch={handleSearch} />
      <div className="my-2"></div> 
      {loading && <p>Loading...</p>}
      {error && <p className="text-red-500">{error}</p>}
      <ItemTable items={items} />
    </div>
  );
};

export default PublicItemsPage;
