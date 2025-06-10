// 製品列のコンポーネント
import React, { useState } from 'react';
import { PublicItem } from '../../../utils/types';

interface ItemRowProps {
  item: PublicItem;
  onEdit?: (item: PublicItem) => void;
  onDelete?: (itemId: string) => void;
}

const ItemRow: React.FC<ItemRowProps> = ({ item, onEdit, onDelete }) => {
  const [showDropdown, setShowDropdown] = useState(false);

  // 背景色の条件分岐
  const getBgColor = () => {
    if (!item.is_remaining) return 'bg-gray-700 hover:bg-gray-600';
    if (item.expiration_date && new Date(item.expiration_date) < new Date()) return 'bg-red-600 hover:bg-red-500';
    return 'bg-white hover:bg-gray-100';
  };

  return (
    <tr className={getBgColor()}>
      <td className="px-4 py-3">{item.name}</td>
      <td className="px-4 py-3 text-center">{item.category?.name || '-'}</td>
      <td className="px-4 py-3 text-center">{item.cost ? `¥${item.cost.toLocaleString()}` : '-'}</td>
      <td className="px-4 py-3 text-center">{item.approval_date || '-'}</td>
      <td className="px-4 py-3 text-center">{item.expiration_date}</td>
      <td className="px-4 py-3 text-center">{item.main_user?.handle_name || 'N/A'}</td>
      <td className="px-4 py-3 text-center">{item.remarks}</td>
      <td className="px-4 py-3 text-center relative">
        <button 
          onClick={() => setShowDropdown(!showDropdown)} 
          className="text-gray-500 hover:text-gray-700 focus:outline-none"
        >
          <span className="text-xl font-bold">&#8942;</span> {/* 3点リーダー */}
        </button>
        
        {showDropdown && (
          <div className="absolute right-0 mt-2 w-48 bg-white rounded-md shadow-lg z-10 border border-gray-200">
            <div className="py-1">
              <button
                onClick={() => {
                  setShowDropdown(false);
                  onEdit && onEdit(item);
                }}
                className="w-full text-left px-4 py-2 text-sm text-blue-600 hover:bg-blue-50 flex items-center gap-2 transition-colors"
              >
                <svg xmlns="http://www.w3.org/2000/svg" className="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z" />
                </svg>
                編集
              </button>
              <button
                onClick={() => {
                  if (confirm('本当にこの備品を削除しますか？')) {
                    onDelete && onDelete(item.public_item_id);
                  }
                  setShowDropdown(false);
                }}
                className="w-full text-left px-4 py-2 text-sm text-red-600 hover:bg-gray-100"
              >
                削除
              </button>
            </div>
          </div>
        )}
      </td>
    </tr>
  );
};

export default ItemRow;
