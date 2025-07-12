//　製品リストのコンポーネント
import React from 'react';
import { PublicItem } from '../../../utils/types';
import ItemRow from './ItemRow';

interface ItemTableProps {
  items: PublicItem[];
  onEdit?: (item: PublicItem) => void;
  onDelete?: (itemId: string) => void;
}

const ItemTable: React.FC<ItemTableProps> = ({ items, onEdit, onDelete }) => (
  <div className="overflow-x-auto">
    <table className="min-w-full bg-white border border-gray-200 rounded-md shadow-md">
      <thead className="bg-gray-100">
        <tr>
          <th className="px-4 py-3 text-center text-gray-700">備品名</th>
          <th className="px-4 py-3 text-center text-gray-700">カテゴリ</th>
          <th className="px-4 py-3 text-center text-gray-700">購入コスト</th>
          <th className="px-4 py-3 text-center text-gray-700">承認日</th>
          <th className="px-4 py-3 text-center text-gray-700">耐用期限</th>
          <th className="px-4 py-3 text-center text-gray-700">メインユーザー</th>
          <th className="px-4 py-3 text-center text-gray-700">備考</th>
          <th className="px-4 py-3 text-center text-gray-700">操作</th>
        </tr>
      </thead>
      <tbody>
        {items.map((item) => (
          <ItemRow 
            key={item.public_item_id} 
            item={item} 
            onEdit={onEdit}
            onDelete={onDelete}
          />
        ))}
      </tbody>
    </table>
  </div>
);

export default ItemTable;
