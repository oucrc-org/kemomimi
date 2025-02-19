// 製品登録ボタンのコンポーネント
import React, { useState } from 'react';
import ItemForm from './Form';

const AddButton: React.FC = () => {
  const [isModalOpen, setIsModalOpen] = useState(false);

  const handleOpenModal = () => {
    setIsModalOpen(true);
  }

  const handleCloseModal = () => {
    setIsModalOpen(false);
  }

  const handleFormSubmit = () => {
    // ここでAPIリクエストを送信する
    console.log('登録成功');
    setIsModalOpen(false);
  }

  return (
    <div className="flex flex-col items-center">
      <button
        onClick={handleOpenModal}
        // GPT部分　やばい可能性あり
        className="border px-6 py-2 rounded-md hover:bg-gray-100 focus:outline-none focus:ring-2 transition duration-200 flex items-center space-x-2"
        aria-haspopup="dialog"
        aria-expanded={isModalOpen}
        aria-controls="item-form"
      >
        {/* アイコン */}
        <svg xmlns="http://www.w3.org/2000/svg" className="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 4v16m8-8H4" />
        </svg>
        <span>新規登録</span>
      </button>

      {/* モーダル */}
      <ItemForm
        isOpen={isModalOpen}
        onClose={handleCloseModal}
        onSuccess={handleFormSubmit}
      />
    </div>
  );
};

export default AddButton;
