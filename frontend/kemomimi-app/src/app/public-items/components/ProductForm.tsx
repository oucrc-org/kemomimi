import React from 'react';
import { Product } from '../../../utils/types';

interface ProductDetailsProps {
  isNew: boolean;
  product: Product | null;
}

const ProductDetails: React.FC<ProductDetailsProps> = ({ isNew, product }) => {
  if (isNew) {
    return (
      <div className="mt-6 bg-gray-50 rounded-lg p-4">
        <h3 className="text-lg font-medium text-gray-900 mb-4">新規製品登録</h3>
        <div className="space-y-4">
          <div>
            <label htmlFor="productName" className="block text-sm font-medium text-gray-700">
              製品名 <span className="text-red-500">*</span>
            </label>
            <input
              id="productName"
              type="text"
              className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
            />
          </div>
          <div>
            <label htmlFor="modelNumber" className="block text-sm font-medium text-gray-700">
              型番
            </label>
            <input
              id="modelNumber"
              type="text"
              className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
            />
          </div>
          <div>
            <label htmlFor="category" className="block text-sm font-medium text-gray-700">
              カテゴリ <span className="text-red-500">*</span>
            </label>
            <select
              id="category"
              className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
            >
              <option value="">カテゴリを選択</option>
              {/* カテゴリの選択肢をここに追加 */}
            </select>
          </div>
          <div>
            <label htmlFor="mainUser" className="block text-sm font-medium text-gray-700">
              メインユーザー <span className="text-red-500">*</span>
            </label>
            <select
              id="mainUser"
              className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
            >
              <option value="">メインユーザーを選択</option>
              {/* メインユーザーの選択肢をここに追加 */}
            </select>
          </div>
          <div>
            <label htmlFor="productRemarks" className="block text-sm font-medium text-gray-700">
              備考
            </label>
            <textarea
              id="productRemarks"
              className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
            />
          </div>
        </div>
      </div>
    );
  }

  if (product) {
    return (
      <div className="mt-6 bg-gray-50 rounded-lg p-4">
        <h3 className="text-lg font-medium text-gray-900 mb-4">製品情報</h3>
        <div className="space-y-3">
          <div className="grid grid-cols-3 gap-4">
            <div>
              <p className="text-sm font-medium text-gray-600">ID</p>
              <p className="mt-1 text-sm text-gray-900">{product.product_id}</p>
            </div>
            <div>
              <p className="text-sm font-medium text-gray-600">製品名</p>
              <p className="mt-1 text-sm text-gray-900">{product.name}</p>
            </div>
            <div>
              <p className="text-sm font-medium text-gray-600">型番</p>
              <p className="mt-1 text-sm text-gray-900">{product.model_number || '---'}</p>
            </div>
          </div>
          {product.main_users && (
            <div className="mt-4 border-t pt-4">
              <div>
                <p className="text-sm font-medium text-gray-600">メインユーザー</p>
                <p className="mt-1 text-sm text-gray-900">{product.main_users[0].handle_name}</p>
              </div>
            </div>
          )}
          {product.categories && (
            <div className="mt-4 border-t pt-4">
              <div>
                <p className="text-sm font-medium text-gray-600">カテゴリ名</p>
                <p className="mt-1 text-sm text-gray-900">{product.categories[0].name}</p>
              </div>
            </div>
          )}
          {product.remarks && (
            <div className="mt-4 border-t pt-4">
              <div>
                <p className="text-sm font-medium text-gray-600">備考</p>
                <p className="mt-1 text-sm text-gray-900">{product.remarks || "---"}</p>
              </div>
            </div>
          )}
        </div>
      </div>
    );
  }

  return null;
};

export default ProductDetails;
