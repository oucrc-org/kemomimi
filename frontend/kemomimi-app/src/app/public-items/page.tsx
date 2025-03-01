"use client";

import React, { useState, useEffect } from 'react';
import { fetchPublicItems } from '../../utils/api';
import type { PublicItem, Product } from '../../utils/types';
import ItemTable from './components/ItemList';
import SearchBar from './components/SearchBar';
import AddButton from './components/AddButton';

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
      <div className="bg-gray-800 text-white text-center text-4xl tracking-wider rounded p-6 mb-4">
      KEMOMIMI SYSTEM
      </div>
      <h1 className="text-2xl font-bold mb-4">Public Items</h1>
      <div className="flex justify-between items-center mb-1">
        <SearchBar onSearch={handleSearch} />
        <AddButton />
      </div>
      <div className="my-2"></div> 
      {loading && <p>Loading...</p>}
      {error && <p className="text-red-500">{error}</p>}
      <ItemTable items={items} />
    </div>
  );
};

export default PublicItemsPage;
