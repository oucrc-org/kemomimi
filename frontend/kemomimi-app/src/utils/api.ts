import type { PublicItem, Product, PublicItemEntry, ProductEntry } from './types';

interface FetchPublicItemsParams {
  sort?: 'public_item_id' | 'cost' | 'approval_date';
  filter?: 'is_remaining' | 'category_id';
  search?: string;
}

interface FetchProductsParams {
  sort?: 'product_id' | 'name';
  filter?: 'category_id';
  search?: string;
}

// 本番サーバーとモックの切り替え
const API_BASE_URL = process.env.NEXT_PUBLIC_API_URL || '/api';
const USE_MOCK_API = process.env.NEXT_PUBLIC_MOCK_API === 'true';

// 備品一覧取得
export const fetchPublicItems = async (params?: FetchPublicItemsParams): Promise<PublicItem[]> => {
  const query = new URLSearchParams(params as any).toString();
  const endpoint = USE_MOCK_API 
    ? `${API_BASE_URL}/public-items-api?${query}`
    : `${API_BASE_URL}/public-items?${query}`;
    
  const response = await fetch(endpoint);
  if (!response.ok) throw new Error('Failed to fetch public items');
  return response.json();
};

// 製品一覧取得
export const fetchProducts = async (params?: FetchProductsParams): Promise<Product[]> => {
  const query = new URLSearchParams(params as any).toString();
  const endpoint = USE_MOCK_API 
    ? `${API_BASE_URL}/products-api?${query}`
    : `${API_BASE_URL}/products?${query}`;
    
  const response = await fetch(endpoint);
  if (!response.ok) throw new Error('Failed to fetch products');
  return response.json();
};

// カテゴリ取得
export const fetchCategories = async (): Promise<any[]> => {
  const endpoint = USE_MOCK_API 
    ? `${API_BASE_URL}/categories-api`
    : `${API_BASE_URL}/categories`;
    
  const response = await fetch(endpoint);
  if (!response.ok) throw new Error('Failed to fetch categories');
  return response.json();
};

// ユーザー取得
export const fetchUsers = async (): Promise<any[]> => {
  const endpoint = USE_MOCK_API 
    ? `${API_BASE_URL}/users-api`
    : `${API_BASE_URL}/users`;
    
  const response = await fetch(endpoint);
  if (!response.ok) throw new Error('Failed to fetch users');
  return response.json();
};

// 備品登録
export const addPublicItem = async (data: PublicItemEntry): Promise<PublicItem> => {
  const endpoint = USE_MOCK_API 
    ? `${API_BASE_URL}/public-items-api`
    : `${API_BASE_URL}/public-items`;
    
  const response = await fetch(endpoint, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(data),
  });
  
  if (!response.ok) {
    const errorText = await response.text();
    throw new Error(`Failed to create public item: ${response.status} ${errorText}`);
  }
  
  return response.json();
};

// 製品登録
export const createProduct = async (data: ProductEntry): Promise<Product> => {
  const endpoint = USE_MOCK_API 
    ? `${API_BASE_URL}/products-api`
    : `${API_BASE_URL}/products`;
    
  const response = await fetch(endpoint, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(data),
  });
  
  if (!response.ok) {
    const errorText = await response.text();
    throw new Error(`Failed to create product: ${response.status} ${errorText}`);
  }
  
  return response.json();
};

