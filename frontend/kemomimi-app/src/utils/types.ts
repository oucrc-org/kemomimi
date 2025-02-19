export interface PublicItem {
  public_item_id: string;  // 備品のユニークID
  name: string;           // 備品名
  is_remaining: boolean;  // 現存しているか

  category?: Category;    // カテゴリ(1つ)
  cost?: number;         // 備品の購入コスト
  approval_date?: string; // 承認日(DATE型)
  expiration_date?: string; // 耐用期限(DATE型)
  main_user?: User;      // 主要な利用者(1人)
  remarks?: string;      // 備考欄
}

export interface Product {
  product_id: string;    // 製品のユニークID 
  name: string;         // 製品名

  model_number?: string;  // 型番
  product_url?: string;   // 商品のURL
  categiries?: Category[];  // カテゴリ配列
  main_users?: User[];    // メインユーザー配列
  remarks?: string;      // 備考欄
}

interface Category {
  category_id: string;  // カテゴリのユニークID
  name: string;        // カテゴリ名 
  remarks?: string;    // 備考欄
}

interface User {
  user_id: string;     // ユーザーID
  screen_name: string; // ユーザー名
  handle_name: string; // ハンドルネーム
  slack_id?: string;   // SlackID
  is_admin: boolean;   // 管理者
  is_member: boolean;  // メンバー
  graduation_date: string; // 卒業日(DATE型)
  remarks?: string;    // 備考欄
}
