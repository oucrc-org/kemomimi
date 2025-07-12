import Link from "next/link";
import Image from "next/image";

export default function Home() {
  return (
    <div className="min-h-screen bg-gray-100">
      <div className="container mx-auto p-4">
        {/* ヘッダー */}
        <div className="bg-gray-800 text-white text-center text-4xl tracking-wider rounded p-2 mb-6">
          <div className="flex items-center justify-center gap-4">
            <Image
              src="/logo.png"
              alt="KEMOMIMI Logo"
              width={400}
              height={80}
              className="h-16 w-auto"
              priority
            />
          </div>
        </div>

        {/* ナビゲーション */}
        <div className="bg-white rounded shadow-sm border p-4 mb-6">
          <nav className="flex gap-6 justify-center">
            <Link 
              href="/public-items" 
              className="text-gray-700 hover:text-blue-600 font-medium px-4 py-2 rounded hover:bg-gray-50 transition-colors"
            >
              備品一覧
            </Link>
            <Link 
              href="/categories" 
              className="text-gray-700 hover:text-blue-600 font-medium px-4 py-2 rounded hover:bg-gray-50 transition-colors"
            >
              カテゴリ管理
            </Link>
          </nav>
        </div>

        {/* メインコンテンツ */}
        <div className="grid md:grid-cols-2 gap-6 mb-6">
          {/* 備品管理セクション */}
          <div className="bg-white rounded shadow-sm border p-6">
            <h2 className="text-xl font-bold text-gray-800 mb-4">備品管理</h2>
            <p className="text-gray-600 mb-4 text-sm">
              組織の備品を効率的に管理するためのシステムです。
            </p>
            <div className="space-y-3">
              <Link 
                href="/public-items"
                className="block w-full bg-blue-600 hover:bg-blue-700 text-white py-2 px-4 rounded text-center transition-colors"
              >
                備品一覧を見る
              </Link>
              <Link 
                href="/categories"
                className="block w-full bg-gray-600 hover:bg-gray-700 text-white py-2 px-4 rounded text-center transition-colors"
              >
                カテゴリ管理
              </Link>
            </div>
          </div>

          {/* システム情報 */}
          <div className="bg-white rounded shadow-sm border p-6">
            <h2 className="text-xl font-bold text-gray-800 mb-4">システム情報</h2>
            <div className="space-y-3 text-sm">
              <div className="flex justify-between py-2 border-b border-gray-200">
                <span className="text-gray-600">バージョン</span>
                <span className="font-medium">v1.0.0</span>
              </div>
              <div className="flex justify-between py-2 border-b border-gray-200">
                <span className="text-gray-600">最終更新</span>
                <span className="font-medium">2025/07/12</span>
              </div>
              <div className="flex justify-between py-2">
                <span className="text-gray-600">ステータス</span>
                <span className="font-medium text-green-600">稼働中</span>
              </div>
            </div>
          </div>
        </div>

        {/* 機能一覧 */}
        <div className="bg-white rounded shadow-sm border p-6 mb-6">
          <h2 className="text-xl font-bold text-gray-800 mb-4">主な機能</h2>
          <div className="grid md:grid-cols-3 gap-4">
            <div className="p-4 border border-gray-200 rounded">
              <h3 className="font-semibold text-gray-800 mb-2">備品登録・管理</h3>
              <p className="text-gray-600 text-sm">備品情報の登録、編集、削除が可能です。</p>
            </div>
            <div className="p-4 border border-gray-200 rounded">
              <h3 className="font-semibold text-gray-800 mb-2">検索・フィルタ</h3>
              <p className="text-gray-600 text-sm">カテゴリやキーワードで備品を検索できます。</p>
            </div>
            <div className="p-4 border border-gray-200 rounded">
              <h3 className="font-semibold text-gray-800 mb-2">履歴管理</h3>
              <p className="text-gray-600 text-sm">操作履歴を記録・確認できます。</p>
            </div>
          </div>
        </div>

        {/* クイックアクセス */}
        <div className="bg-white rounded shadow-sm border p-6">
          <h2 className="text-xl font-bold text-gray-800 mb-4">クイックアクセス</h2>
          <div className="grid grid-cols-2 md:grid-cols-4 gap-3">
            <Link 
              href="/public-items"
              className="border border-gray-300 hover:border-blue-500 p-3 rounded text-center transition-colors"
            >
              <div className="text-2xl mb-1">📋</div>
              <div className="text-sm font-medium">全備品</div>
            </Link>
            <Link 
              href="/categories"
              className="border border-gray-300 hover:border-blue-500 p-3 rounded text-center transition-colors"
            >
              <div className="text-2xl mb-1">📁</div>
              <div className="text-sm font-medium">カテゴリ</div>
            </Link>
            <div className="border border-gray-200 p-3 rounded text-center text-gray-400 cursor-not-allowed">
              <div className="text-2xl mb-1">➕</div>
              <div className="text-sm font-medium">新規登録</div>
              <div className="text-xs">準備中</div>
            </div>
            <div className="border border-gray-200 p-3 rounded text-center text-gray-400 cursor-not-allowed">
              <div className="text-2xl mb-1">📈</div>
              <div className="text-sm font-medium">統計</div>
              <div className="text-xs">準備中</div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
