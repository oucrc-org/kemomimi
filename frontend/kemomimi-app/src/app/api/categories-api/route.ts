import { NextResponse } from 'next/server';

export async function GET(request: Request) {
  // モックデータ - カテゴリ一覧
  const categories = [
    {
      category_id: "1",
      name: "KEMOMIMI",
      remarks: "KAWAII"
    },
    {
      category_id: "2", 
      name: "コンピュータ",
      remarks: "PC関連機器"
    },
    {
      category_id: "3",
      name: "家電",
      remarks: "家電製品"
    },
    {
      category_id: "4",
      name: "文房具",
      remarks: "事務用品"
    },
    {
      category_id: "5",
      name: "その他",
      remarks: "その他の備品"
    }
  ];

  return NextResponse.json(categories);
}

export async function POST(request: Request) {
  try {
    const body = await request.json();
    
    // 新しいカテゴリIDを生成（実際のAPIでは自動生成される）
    const newCategoryId = String(Date.now());
    
    const newCategory = {
      category_id: newCategoryId,
      name: body.name,
      remarks: body.remarks || ""
    };

    console.log('新しいカテゴリが作成されました:', newCategory);

    return NextResponse.json(newCategory, { status: 201 });
  } catch (error) {
    console.error('カテゴリ作成エラー:', error);
    return NextResponse.json(
      { error: 'カテゴリの作成に失敗しました' },
      { status: 500 }
    );
  }
}
