import React, { useState, useEffect } from 'react';
import { Product } from '../../../utils/api';

interface PublicItemEntry {
  name: string;
  cost?: number;
  purchase_date?: string;
  expiration_date?: string;
  is_remaining: boolean;
  remarks?: string;
}

interface ItemFormProps {
  isOpen: boolean;
  onClose: () => void;
  onSuccess?: () => void;
}

const ItemForm: React.FC<ItemFormProps> = ({ isOpen, onClose, onSuccess }) => {
  const [name, setName] = useState('');
  const [cost, setCost] = useState<number | undefined>(undefined);
  const [purchaseDate, setPurchaseDate] = useState('');
  const [expirationDate, setExpirationDate] = useState('');
  const [isRemaining, setIsRemaining] = useState(true);
  const [remarks, setRemarks] = useState('');

  const [selectedProduct, setSelectedProduct] = useState<Product | null>(null);
  const [products, setProducts] = useState<Product[]>([]);

  const [errors, setErrors] = useState<{ [key in keyof PublicItemEntry]?: string }>({});

  const firstInputRef = React.useRef<HTMLInputElement>(null);

  useEffect(() => {
    if (isOpen && firstInputRef.current) {
      firstInputRef.current.focus();
    }
  }, [isOpen]);

  useEffect(() => {
    if (isOpen) {
      // フォームが開いた際に製品一覧を取得する
      fetchProducts();
    }
  }, [isOpen]);

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === 'Escape' && isOpen) {
        onClose();
      }
    };
    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [isOpen, onClose]);

  const validate = (): { [key in keyof PublicItemEntry]?: string } => {
    const newErrors: { [key in keyof PublicItemEntry]?: string } = {};
    if (!name.trim()) {
      newErrors.name = '備品名は必須です。';
    }
    return newErrors;
  };

  const fetchProducts = async () => {
    try {
      const response = await fetch('/products');
      if (response.ok) {
        const data = await response.json();
        setProducts(data);
      } else {
        console.error('製品一覧の取得に失敗しました。', response.status);
      }
    } catch (error) {
      console.error('製品一覧取得エラー:', error);
    }
  };

  const handleSelect = (e: React.ChangeEvent<HTMLSelectElement>) => {
    const productId = e.target.value;
    const product = products.find(prod => prod.product_id === productId);
    setSelectedProduct(product || null);
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    const validationErrors = validate();
    if (Object.keys(validationErrors).length > 0) {
      setErrors(validationErrors);
      return;
    }

    const postData: PublicItemEntry = {
      name: name.trim(),
      cost,
      purchase_date: purchaseDate || undefined,
      expiration_date: expirationDate || undefined,
      is_remaining: isRemaining,
      remarks: remarks.trim() || undefined,
    };

    try {
      const response = await fetch('/public-items', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(postData),
      });

      if (!response.ok) {
        console.error('登録エラー:', response.status, await response.text());
        alert(`登録に失敗しました (status: ${response.status})`);
        return;
      }

      const data = await response.json();
      console.log('登録成功:', data);

      // 登録後の処理
      setName('');
      setCost(undefined);
      setPurchaseDate('');
      setExpirationDate('');
      setIsRemaining(true);
      setRemarks('');
      setErrors({});

      onClose();
      if (onSuccess) {
        onSuccess();
      }
    } catch (error) {
      console.error('通信エラー:', error);
      alert('通信エラーが発生しました');
    }
  };

  if (!isOpen) return null;

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
        <h2 id="modal-title" className="text-2xl font-semibold mb-4">新規備品登録</h2>
        <form onSubmit={handleSubmit} noValidate>
          {/* Name */}
          <div className="mb-4">
            <label htmlFor="name" className="block text-sm font-medium text-gray-700">
              備品名 <span className="text-red-500">*</span>
            </label>
            <input
              id="name"
              type="text"
              ref={firstInputRef}
              value={name}
              onChange={(e) => setName(e.target.value)}
              className={`mt-1 block w-full px-3 py-2 border ${errors.name ? 'border-red-500' : 'border-gray-300'
                } rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500`}
              aria-invalid={!!errors.name}
              aria-describedby={errors.name ? 'name-error' : undefined}
            />
            {errors.name && (
              <p className="mt-1 text-sm text-red-600" id="name-error">
                {errors.name}
              </p>
            )}
          </div>
          {/* Product */}
          <label htmlFor="product-select">製品を選択：</label>
          <select id="product-select" onChange={handleSelect} defaultValue="">
            <option value="">-- 製品を選択 --</option>
            {products.map((product) => (
              <option key={product.product_id} value={product.product_id}>
                {product.name}
              </option>
            ))}
          </select>
          {/* Product detail */}
          {selectedProduct && (
            <div style={{ marginTop: '1rem' }}>
              <h3>選択された製品の詳細</h3>
              <p><strong>ID:</strong> {selectedProduct.product_id}</p>
              <p><strong>名称:</strong> {selectedProduct.name}</p>
              {selectedProduct.categiries && (
                <>
                  <p><strong>カテゴリ名:</strong> {selectedProduct.categiries[0].name}</p>
                  <p><strong>備考:</strong> {selectedProduct.categiries[0].remarks}</p>
                </>
              )}
            </div>
          )}
          {/* Cost */}
          <div className="mb-4">
            <label htmlFor="cost" className="block text-sm font-medium text-gray-700">
              購入コスト
            </label>
            <input
              id="cost"
              type="number"
              value={cost === undefined ? '' : cost}
              onChange={(e) => setCost(e.target.value ? Number(e.target.value) : undefined)}
              className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
            />
          </div>

          {/* Purchase Date */}
          <div className="mb-4">
            <label htmlFor="purchase_date" className="block text-sm font-medium text-gray-700">
              購入日
            </label>
            <input
              id="purchase_date"
              type="date"
              value={purchaseDate}
              onChange={(e) => setPurchaseDate(e.target.value)}
              className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
            />
          </div>

          {/* Expiration Date */}
          <div className="mb-4">
            <label htmlFor="expiration_date" className="block text-sm font-medium text-gray-700">
              耐用期限
            </label>
            <input
              id="expiration_date"
              type="date"
              value={expirationDate}
              onChange={(e) => setExpirationDate(e.target.value)}
              className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
            />
          </div>

          {/* is_remaining */}
          <div className="mb-4 flex items-center">
            <input
              id="is_remaining"
              type="checkbox"
              checked={isRemaining}
              onChange={(e) => setIsRemaining(e.target.checked)}
              className="h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
            />
            <label htmlFor="is_remaining" className="ml-2 block text-sm text-gray-700">
              現存しているか
            </label>
          </div>

          {/* remarks */}
          <div className="mb-4">
            <label htmlFor="remarks" className="block text-sm font-medium text-gray-700">
              備考
            </label>
            <textarea
              id="remarks"
              value={remarks}
              onChange={(e) => setRemarks(e.target.value)}
              className="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
            />
          </div>

          {/* ボタン */}
          <div className="flex justify-end space-x-2">
            <button
              type="submit"
              className="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500"
            >
              登録
            </button>
            <button
              type="button"
              onClick={onClose}
              className="px-4 py-2 bg-gray-300 text-gray-700 rounded-md hover:bg-gray-400 focus:outline-none focus:ring-2 focus:ring-gray-500"
            >
              キャンセル
            </button>
          </div>
        </form>

        {/* 右上の閉じるボタン */}
        <button
          onClick={onClose}
          className="absolute top-3 right-3 text-gray-500 hover:text-gray-700 focus:outline-none"
          aria-label="Close"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            className="h-6 w-6"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M6 18L18 6M6 6l12 12"
            />
          </svg>
        </button>
      </div>
    </div>
  );
};

export default ItemForm;
