# エンティティ定義

## 備品 (PublicItem)

### 説明

管理対象となる備品(≠私物)の情報を保存。

### 属性

| 名前                  | データ型 | 制約                                | 説明                                                                |
| --------------------- | -------- | ----------------------------------- | ------------------------------------------------------------------- |
| `public_item_id`      | UUID     | PRIMARY KEY                         | 備品のユニークID                                                    |
| `name`                | TEXT     | NOT NULL                            | 備品名(製品名と別なのは、PC愛称などを入れる想定)                    |
| `product_id`          | UUID     | FOREIGN KEY, NOT NULL               | 製品のユニークID(Productへの外部キー)                               |
| `cost`                | INT      | `cost`>=0                           | 備品の購入コスト                                                    |
| `purchase_date`       | DATE     | NOT NULL                            | 導入日                                                              |
| `expiration_date`     | DATE     | `expiration_date` > `purchase_date` | 耐用期限                                                            |
| `is_remaining`        | BOOLEAN  | DEFAULT TRUE, NOT NULL              | 現存しているか(廃棄済みや失効済みならFALSE)                         |
| `purchase_request_id` | TEXT     | FOREIGN KEY                         | 追加元の購入申請（PurchaseRequestへの外部キー、寄付等の場合はNULL） |
| `remarks`             | TEXT     |                                     | 備考欄                                                              |

## 私物(PrivateItem)

### 説明

スリッパやPCのアカウントなど、部員が個人的に管理している製品の情報を保存。

### 属性

| 名前                 | データ型 | 制約                   | 説明                                        |
| -------------------- | -------- | ---------------------- | ------------------------------------------- |
| `private_item_id`    | UUID     | PRIMARY KEY            | 私物のユニークID                            |
| `name`               | TEXT     | NOT NULL               | 製品名                                      |
| `owner_id`           | UUID     | FOREIGN KEY            | 所有者（Userへの外部キー）                  |
| `post_grad_treat_id` | TEXT     | FOREIGN KEY            | 卒業後の処理（PostGradTreatへの外部キー）   |
| `model_number`       | TEXT     |                        | 型番                                        |
| `is_remaining`       | BOOLEAN  | DEFAULT TRUE, NOT NULL | 現存しているか(廃棄済みや失効済みならFALSE) |
| `remarks`            | TEXT     |                        | 備考欄                                      |

## 卒業後の私物の処理(PostGradTreat)

### 説明

私物が所有者の卒業後にどうなるかを管理。`未定`となっている場合は卒業前にSlackでメンションする等の対応が必要。

| 名前         | データ型 | 制約        | 説明                         |
| ------------ | -------- | ----------- | ---------------------------- |
| `treat_id`   | UUID     | PRIMARY KEY | 処理のユニークID             |
| `treat_name` | TEXT     | NOT NULL    | 処理名（未定や回収、寄付等） |

## 製品 (Product)

### 説明

製品に関する一般的な情報。備品と違い、管理対象でない (申請中の) 製品も扱えるよう属性を抽出した。

### 属性

| 名前           | データ型 | 制約        | 説明             |
| -------------- | -------- | ----------- | ---------------- |
| `product_id`   | UUID     | PRIMARY KEY | 製品のユニークID |
| `name`         | TEXT     | NOT NULL    | 製品名           |
| `model_number` | TEXT     |             | 型番             |
| `product_url`  | TEXT     |             | 商品のURL        |
| `remarks`      | TEXT     |             | 備考欄           |

## 分類 (Category)

### 説明

製品を分類するカテゴリ情報を保存。

### 属性

| 名前          | データ型 | 制約             | 説明                 |
| ------------- | -------- | ---------------- | -------------------- |
| `category_id` | TEXT     | PRIMARY KEY      | カテゴリのユニークID |
| `name`        | TEXT     | UNIQUE, NOT NULL | カテゴリ名           |
| `remarks`     | TEXT     |                  | 備考欄               |

## 製品-カテゴリ関係 (ProductCategory)

### 説明

`Product` と `Category` の多対多の関係を管理。

### 属性

| 名前          | データ型                      | 制約        | 説明                                 |
| ------------- | ----------------------------- | ----------- | ------------------------------------ |
| `product_id`  | UUID                          | FOREIGN KEY | 製品のID（Productへの外部キー）      |
| `category_id` | TEXT                          | FOREIGN KEY | カテゴリのID（Categoryへの外部キー） |
| PRIMARY KEY   | (`product_id`, `category_id`) |             |

## 部員 (member)

### 説明

システムを利用するユーザーの情報を保存。製品によっては部員の私物が管理対象となる(ボードゲーム、漫画等)ので、ユーザーの卒業日なども保存する必要がある。

### 属性

| 名前              | データ型 | 制約                    | 説明                             |
| ----------------- | -------- | ----------------------- | -------------------------------- |
| `user_id`         | UUID     | PRIMARY KEY             | ユーザーのユニークID             |
| `handle_name`     | TEXT     | NOT NULL                | ユーザーのハンドルネーム         |
| `screen_name`     | TEXT     | UNIQUE, NOT NULL        | ユーザーのスクリーンネーム       |
| `slack_id`        | TEXT     | UNIQUE                  | ユーザーのSlack ID(通知等に使用) |
| `is_admin`        | BOOLEAN  | DEFAULT FALSE, NOT NULL | 管理者フラグ                     |
| `is_member`       | BOOLEAN  | DEFAULT TRUE, NOT NULL  | 在籍状況                         |
| `graduation_date` | DATE     |                         | 卒業日                           |
| `remarks`         | TEXT     |                         | 備考欄                           |


## メインユーザー - 備品関係 (MainUserPublicItem)

### 説明

「備品」と「メインユーザー」の多対多の関係を管理する中間テーブル。

### 属性

| 名前             | データ型                      | 制約        | 説明                                     |
| ---------------- | ----------------------------- | ----------- | ---------------------------------------- |
| `public_item_id` | UUID                          | FOREIGN KEY | 備品のID (`PublicItem` への外部キー)     |
| `user_id`        | UUID                          | FOREIGN KEY | メインユーザーのID (`User` への外部キー) |
| PRIMARY KEY      | (`public_item_id`, `user_id`) |             | -                                        |


## 購入申請 (PurchaseRequest)

### 説明

新しい備品の購入申請を管理。

### 属性

| 名前                  | データ型 | 制約                 | 説明                                                                        |
| --------------------- | -------- | -------------------- | --------------------------------------------------------------------------- |
| `purchase_request_id` | UUID     | PRIMARY KEY          | 申請のユニークID                                                            |
| `applicant_id`        | UUID     | FOREIGN KEY          | 申請者のID（Userへの外部キー）                                              |
| `product_id`          | UUID     | FOREIGN KEY          | 申請する製品のID                                                            |
| `cost`                | INT      | COST>=0              | 申請時の想定費用                                                            |
| `status_id`           | TEXT     | FOREIGN KEY          | 購入申請状態ID（Statusへの外部キー、pending(保留)やapproved(承認済み)など） |
| `request_date`        | DATE     | DEFAULT CURRENT_DATE | 申請作成日                                                                  |
| `approval_date`       | DATE     |                      | 承認日                                                                      |
| `remarks`             | TEXT     |                      | 備考欄                                                                      |

## 購入申請の状態 (PurchaseRequestStatus)

### 説明

購入申請の状態一覧を管理。

### 属性

| 名前                         | データ型 | 制約             | 説明                      |
| ---------------------------- | -------- | ---------------- | ------------------------- |
| `purchase_request_status_id` | UUID     | PRIMARY KEY      | 状態のユニークID          |
| `status_name`                | TEXT     | UNIQUE, NOT NULL | 状態名（例: Pendingなど） |
| `remarks`                    | TEXT     |                  | 備考欄                    |
