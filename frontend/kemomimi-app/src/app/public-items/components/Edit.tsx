import React, { useState, useEffect } from 'react';
import { PublicItem, Category } from '../../../utils/types';

// Material-UI components
import IconButton from '@mui/material/IconButton';
import CloseIcon from '@mui/icons-material/Close';

interface EditItemModalProps {
  item: PublicItem | null;
  categories: Category[];
  isOpen: boolean;
  onClose: () => void;
  onSave: (updatedItem: PublicItem) => void;
}

const EditItemModal: React.FC<EditItemModalProps> = ({
  item,
  isOpen,
  onClose,
  onSave
}) => {
  const [formData, setFormData] = useState<Partial<PublicItem>>({});

  useEffect(() => {
    if (item) {
      setFormData({
        name: item.name,
        cost: item.cost,
        approval_date: item.approval_date,
        expiration_date: item.expiration_date,
        remarks: item.remarks || '',
      });
    }
  }, [item]);

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (item) {
      const updatedItem: PublicItem = {
        ...item,
        ...formData,
      };
      onSave(updatedItem);
    }
  };

  if (!isOpen || !item) return null;

  return (
    <div
      className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50"
      aria-labelledby="modal-title"
      role="dialog"
      aria-modal="true"
      onClick={onClose}
    >
      <div
        className="bg-white rounded-lg shadow-lg w-full max-w-md p-6 relative max-h-[80vh] overflow-y-auto"
        onClick={(e) => e.stopPropagation()}
      >
        <div className="flex justify-between items-center mb-4">
          <h2 id="modal-title" className="text-2xl font-semibold">
            備品を編集
          </h2>
        </div>

        <form onSubmit={handleSubmit} className="space-y-4" noValidate>
          <div className="mb-4">
            <label htmlFor="name" className="block text-sm font-medium text-gray-700">
              備品名
            </label>
            <input
              id="name"
              type="text"
              value={formData.name || ''}
              onChange={(e) => setFormData({ ...formData, name: e.target.value })}
              className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
              required
            />
          </div>

          <div className="mb-4">
            <label htmlFor="cost" className="block text-sm font-medium text-gray-700">
              価格
            </label>
            <input
              id="cost"
              type="number"
              value={formData.cost || ''}
              onChange={(e) => setFormData({ ...formData, cost: e.target.value ? Number(e.target.value) : undefined })}
              className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
            />
          </div>

          <div className="mb-4">
            <label htmlFor="approval_date" className="block text-sm font-medium text-gray-700">
              承認日
            </label>
            <input
              id="approval_date"
              type="date"
              value={formData.approval_date || ''}
              onChange={(e) => setFormData({ ...formData, approval_date: e.target.value })}
              className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
            />
          </div>

          <div className="mb-4">
            <label htmlFor="expiration_date" className="block text-sm font-medium text-gray-700">
              有効期限
            </label>
            <input
              id="expiration_date"
              type="date"
              value={formData.expiration_date || ''}
              onChange={(e) => setFormData({ ...formData, expiration_date: e.target.value })}
              className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
            />
          </div>

          <div className="mb-4">
            <label htmlFor="remarks" className="block text-sm font-medium text-gray-700">
              備考
            </label>
            <textarea
              id="remarks"
              value={formData.remarks || ''}
              onChange={(e) => setFormData({ ...formData, remarks: e.target.value })}
              rows={3}
              className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
            />
          </div>

          <div className="flex justify-end">
            <div className="flex gap-3">
              <button
                type="submit"
                className="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500"
              >
                保存
              </button>
              <button
                type="button"
                onClick={onClose}
                className="px-4 py-2 text-gray-700 border border-gray-300 rounded-md hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-gray-500"
              >
                キャンセル
              </button>
            </div>
          </div>
        </form>

        {/* 右上の閉じるボタン */}
        <IconButton
          onClick={onClose}
          aria-label="Close"
          size="small"
          sx={{
            position: 'absolute',
            top: 12,
            right: 12,
            zIndex: 10
          }}
        >
          <CloseIcon />
        </IconButton>
      </div>
    </div>
  );
};

export default EditItemModal;
