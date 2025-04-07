import { NextResponse } from 'next/server';

export async function GET(request: Request) {
  const url = new URL(request.url);
  const sort = url.searchParams.get('sort');
  const filter = url.searchParams.get('filter');
  const search = url.searchParams.get('search');

  let publicItems = [
    {
      public_item_id: "1",
      name: "KEMOMIMI",
      cost: 3000000,
      category: {
        category_id: "1",
        name: "KEMOMIMI",
        remarks: "KAWAII"
      },
      approval_date: "2025-01-01",
      expiration_date: "2026-01-01",
      is_remaining: false,
      main_user: {
        user_id: "string",
        handle_name: "KEMO",
        screen_name: "string",
        slack_id: "string",
        is_admin: true,
        is_member: true,
        graduation_date: "2019-08-24",
        remarks: "string"
      },
      remarks: "なまもの",
    },

    {
      public_item_id: "2",
      name: "PC",
      cost: 334,
      category: {
        category_id: "1",
        name: "KEMOMIMI",
        remarks: "KAWAII"
      },
      approval_date: "2025-01-01",
      expiration_date: "2026-01-01",
      is_remaining: true,
      main_user: {
        user_id: "string",
        handle_name: "SEIGETSU",
        screen_name: "string",
        slack_id: "string",
        is_admin: true,
        is_member: true,
        graduation_date: "2019-08-24",
        remarks: "string"
      },
      remarks: "",
    },

    {
      public_item_id: "3",
      name: "KEMOMIMI",
      cost: 3000000,
      category: {
        category_id: "1",
        name: "KEMOMIMI",
        remarks: "KAWAII"
      },
      approval_date: "2020-01-01",
      expiration_date: "2021-01-01",
      is_remaining: true,
      main_user: {
        user_id: "string",
        handle_name: "KEMO",
        screen_name: "string",
        slack_id: "string",
        is_admin: true,
        is_member: true,
        graduation_date: "2019-08-24",
        remarks: "string"
      },
      remarks: "なまもの",
    },
  ];

  // フィルタリング
  if (filter) {
    publicItems = publicItems.filter(item => {
      if (filter === 'is_remaining') {
        return item.is_remaining;
      }
      if (filter === 'category_id') {
        return item.category.category_id === url.searchParams.get('category_id');
      }
      if (filter === 'user_id') {
        return item.main_user.user_id === url.searchParams.get('user_id');
      }
      return true;
    });
  }

  // 検索
  if (search) {
    publicItems = publicItems.filter(item => item.name.includes(search));
  }

  // ソート
  if (sort) {
    publicItems.sort((a, b) => {
      if (sort === 'public_item_id') {
        return a.public_item_id.localeCompare(b.public_item_id);
      }
      if (sort === 'cost') {
        return a.cost - b.cost;
      }
      if (sort === 'approval_date') {
        return new Date(a.approval_date).getTime() - new Date(b.approval_date).getTime();
      }
      if (sort === 'expiration_date') {
        return new Date(a.expiration_date).getTime() - new Date(b.expiration_date).getTime();
      }
      return 0;
    });
  }

  return NextResponse.json(publicItems);
}
