// 製品列のコンポーネント
import React from 'react';
import { PublicItem } from '../../../utils/api';

interface ItemRowProps {
  item: PublicItem;
}

const ItemRow: React.FC<ItemRowProps> = ({ item }) => {
  const now = new Date();
  const expirationDate = new Date(item.expiration_date);

  let rowClass = "hover:bg-gray-100";
  if (expirationDate < now) {
    rowClass = "bg-red-400 hover:bg-red-500";
  }
  else if (! item.is_remaining) {
    rowClass = "bg-gray-400 hover:bg-gray-500";
  }
  return (
    <tr className={rowClass}>
      <td className="px-4 py-3 text-center">{item.name}</td>
      <td className="px-4 py-3 text-center">{item.category.name}</td>
      <td className="px-4 py-3 text-center">￥{item.cost.toLocaleString()}</td>
      <td className="px-4 py-3 text-center">{item.approval_date}</td>
      <td className="px-4 py-3 text-center">{item.expiration_date}</td>
      <td className="px-4 py-3 text-center">{item.main_user?.handle_name || 'N/A'}</td>
      <td className="px-4 py-3 text-center">{item.remarks}</td>
    </tr>
  );
}

export default ItemRow;
