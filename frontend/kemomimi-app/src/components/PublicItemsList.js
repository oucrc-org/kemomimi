import React, { useState, useEffect } from 'react';
import axios from 'axios';
import './PublicItemsList.css';

const PublicItemsList = () => {
  const [publicItems, setPublicItems] = useState([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);
  const [searchTerm, setSearchTerm] = useState('');
  const [sortField, setSortField] = useState('');

  const BASE_URL = 'http://localhost:3001';

  const fetchPublicItems = async () => {
    setLoading(true);
    setError(null);
    try {
      const params = {};
      if (searchTerm) {
        params.search = 'public_item_name';
        params.searchValue = searchTerm;
      }
      if (sortField) {
        params.sort = sortField;
      }
      const response = await axios.get(`${BASE_URL}/public-items`, { params });
      setPublicItems(response.data);
      setLoading(false);
    } catch (err) {
      setError(err.message);
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchPublicItems();
  }, []);

  const handleSearch = () => {
    fetchPublicItems();
  };

  const handleSortChange = (e) => {
    setSortField(e.target.value);
  };

  const handleKeyDown = (e) => {
    if (e.key === 'Enter') {
      handleSearch();
    }
  };

  if (loading) {
    return <p>読み込み中...</p>;
  }

  if (error) {
    return <p>エラーが発生しました: {error}</p>;
  }

  return (
    <div className="public-items-list">
      <h2>Public Item List</h2>
      <div className="search-container">
        {/* 検索フォーム */}
        <input
          type="text"
          placeholder="Search"
          value={searchTerm}
          onChange={(e) => setSearchTerm(e.target.value)}
          onKeyDown={handleKeyDown}
          className="search-input"
        />
      </div>
      <table className="public-items-table">
        <thead>
          <tr>
            <th>Name</th>
            <th>Category</th>
            <th>Cost</th>
            <th>Approval Date</th>
            <th>Expiration Date</th>
            <th>Main User</th>
            <th>Remarks</th>
          </tr>
        </thead>
        <tbody>
          {publicItems.map((item) => (
            <tr key={item.public_item_id}>
              <td>{item.name}</td>
              <td>{item.category ? item.category.name : 'N/A'}</td>
              <td>{item.cost !== null ? `¥${item.cost}` : 'N/A'}</td>
              <td>{formatDate(item.approval_date)}</td>
              <td>{formatDate(item.expiration_date)}</td>
              <td>{item.main_user ? item.main_user.handle_name : 'N/A'}</td>
              <td>{item.remarks || 'N/A'}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
};

const formatDate = (dateString) => {
  if (!dateString) return 'N/A';
  const options = { year: 'numeric', month: '2-digit', day: '2-digit' };
  return new Date(dateString).toLocaleDateString('ja-JP', options);
};

export default PublicItemsList;