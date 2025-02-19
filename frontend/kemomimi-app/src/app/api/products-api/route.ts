import { NextResponse } from 'next/server';
import type { Product } from '../../../utils/types';

export async function GET(request: Request) {
  const url = new URL(request.url);
  const sort = url.searchParams.get('sort');
  const filter = url.searchParams.get('filter'); 
  const search = url.searchParams.get('search');

  let products = [
    {
      product_id: "1",
      name: "けもみみ",
      model_number: "KM-001",
      product_url: "https://ahokusa.com/kmm-001",
      categiries: [
        {
          category_id: "1",
          name: "けもみみ",
          remarks: "かわいい"
        }
      ],
      main_users: [
        {
          user_id: "1",
          handle_name: "KEMO",
          screen_name: "ahokusa",
          slack_id: "U123456",
          is_admin: false,
          is_member: true,
          graduation_date: null,
          remarks: "けもみみ"
        }
      ],
      remarks: "けもみみ"
    },
  ];

  // フィルタリング
  if (filter) {
    products = products.filter(product => {
      if (filter === 'category_id') {
        return product.categiries.some(
          cat => cat.category_id === url.searchParams.get('category_id')
        );
      }
      return true;
    });
  }

  // 検索
  if (search) {
    const searchLower = search.toLowerCase();
    products = products.filter(product => 
      product.name.toLowerCase().includes(searchLower) || 
      product.model_number?.toLowerCase().includes(searchLower)
    );
  }

  // ソート
  if (sort) {
    products.sort((a, b) => {
      if (sort === 'product_id') {
        return a.product_id.localeCompare(b.product_id);
      }
      if (sort === 'name') {
        return a.name.localeCompare(b.name);
      }
      return 0;
    });
  }

  return NextResponse.json(products);
}

export async function POST(request: Request) {
  try {
    const body = await request.json();

    // バリデーション
    if (!body.name) {
      return NextResponse.json(
        { error: '製品名は必須です' },
        { status: 400 }
      );
    }

    const newProduct: Product = {
      product_id: "temp-id",
      name: body.name,
      model_number: body.model_number,
      product_url: body.product_url,
      categiries: body.categiries || [],
      main_users: body.main_users || [],
      remarks: body.remarks
    };

    return NextResponse.json(newProduct, { status: 201 });

  } catch (error) {
    console.error('Error:', error);
    return NextResponse.json(
      { error: 'Internal Server Error' },
      { status: 500 }
    );
  }
}
