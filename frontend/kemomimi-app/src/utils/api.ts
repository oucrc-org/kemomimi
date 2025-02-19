import type { PublicItem, Product } from './types';

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

export const fetchPublicItems = async (params?: FetchPublicItemsParams): Promise<PublicItem[]> => {
  const query = new URLSearchParams(params as any).toString();
  const response = await fetch(`/api/public-items-api?${query}`);
  if (!response.ok) throw new Error('Failed to fetch public items');
  return response.json();
};

export const fetchProducts = async (params?: FetchProductsParams): Promise<Product[]> => {
  const query = new URLSearchParams(params as any).toString();
  const response = await fetch(`/api/products-api?${query}`);
  if (!response.ok) throw new Error('Failed to fetch products');
  return response.json();
};
