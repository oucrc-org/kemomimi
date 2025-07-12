"use client";

import React, { useState, useEffect } from 'react';
import { fetchPublicItems, fetchCategories } from '../../utils/api';
import type { PublicItem, Product, Category } from '../../utils/types';
import ItemTable from './components/ItemList';
import SearchBar from './components/SearchBar';
import AddButton from './components/AddButton';
import ItemForm from './components/Form';
import EditItemModal from './components/Edit';

const PublicItemsPage: React.FC = () => {
  const [items, setItems] = useState<PublicItem[]>([]);
  const [categories, setCategories] = useState<Category[]>([]);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);
  // モーダルの状態管理
  const [isModalOpen, setIsModalOpen] = useState<boolean>(false);
  // 編集モーダルの状態管理
  const [isEditModalOpen, setIsEditModalOpen] = useState<boolean>(false);
  const [editingItem, setEditingItem] = useState<PublicItem | null>(null);

  const loadItems = async (searchTerm?: string) => {
    try {
      setLoading(true);
      const data = await fetchPublicItems(searchTerm ? { search: searchTerm } : undefined);
      setItems(data);
    } catch (err) {
      setError('Failed to fetch items');
    } finally {
      setLoading(false);
    }
  };

  const loadCategories = async () => {
    try {
      const data = await fetchCategories();
      setCategories(data);
    } catch (err) {
      console.error('Failed to fetch categories');
    }
  };

  useEffect(() => {
    loadItems();
    loadCategories();
  }, []);

  const handleSearch = async (searchTerm: string) => {
    loadItems(searchTerm);
  };

  const handleOpenModal = () => {
    setIsModalOpen(true);
  };
  
  const handleCloseModal = () => {
    setIsModalOpen(false);
  };
  
  const handleAddSuccess = () => {
    setIsModalOpen(false);
    loadItems();
  };

  const handleEdit = (item: PublicItem) => {
    setEditingItem(item);
    setIsEditModalOpen(true);
  };

  const handleCloseEditModal = () => {
    setIsEditModalOpen(false);
    setEditingItem(null);
  };

  const handleSaveEdit = async (updatedItem: PublicItem) => {
    try {
      // TODO: APIを呼び出して更新処理を実装
      // await updatePublicItem(updatedItem.public_item_id, updatedItem);
      
      // 一時的にローカル状態を更新
      setItems(items.map(item => 
        item.public_item_id === updatedItem.public_item_id ? updatedItem : item
      ));
      
      setIsEditModalOpen(false);
      setEditingItem(null);
    } catch (err) {
      console.error('Failed to update item:', err);
    }
  };

  const handleDelete = async (itemId: string) => {
    try {
      // TODO: APIを呼び出して削除処理を実装
      // await deletePublicItem(itemId);
      
      // 一時的にローカル状態を更新
      setItems(items.filter(item => item.public_item_id !== itemId));
    } catch (err) {
      console.error('Failed to delete item:', err);
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
        <AddButton onOpenModal={handleOpenModal} />
      </div>
      <div className="my-2"></div> 
      {loading && <p>Loading...</p>}
      {error && <p className="text-red-500">{error}</p>}
      <ItemTable items={items} onEdit={handleEdit} onDelete={handleDelete} />

      {/* モーダルコンポーネント */}
      <ItemForm 
        isOpen={isModalOpen}
        onClose={handleCloseModal}
        onSuccess={handleAddSuccess}
      />

      {/* 編集モーダル */}
      <EditItemModal
        item={editingItem}
        categories={categories}
        isOpen={isEditModalOpen}
        onClose={handleCloseEditModal}
        onSave={handleSaveEdit}
      />
    </div>
  );
};

export default PublicItemsPage;
