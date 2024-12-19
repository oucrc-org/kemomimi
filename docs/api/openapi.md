<!-- Generator: Widdershins v4.0.1 -->

<h1 id="kemomimi-api">KEMOMIMI API v0.2</h1>

> Scroll down for code samples, example requests and responses. Select a language for code samples from the tabs above or the mobile navigation menu.

備品管理システムのREST API仕様書です。

Base URLs:

* <a href="http://localhost:3000">http://localhost:3000</a>

<h1 id="kemomimi-api-publicitems">PublicItems</h1>

備品

## get__public-items

`GET /public-items`

*備品一覧取得*

<h3 id="get__public-items-parameters">Parameters</h3>

|Name|In|Type|Required|Description|
|---|---|---|---|---|
|sort|query|string|false|ソート対象のフィールド名|
|filter|query|string|false|フィルター対象のフィールド名|
|search|query|string|false|検索対象のフィールド名|

#### Enumerated Values

|Parameter|Value|
|---|---|
|sort|public_item_id|
|sort|cost|
|sort|approval_date|
|sort|expiration_date|
|filter|is_remaining|
|filter|category_id|
|filter|user_id|
|search|public_item_name|

> Example responses

> 200 Response

```json
[
  {
    "public_item_id": "string",
    "name": "string",
    "category": {
      "category_id": "string",
      "name": "string",
      "remarks": "string"
    },
    "cost": 0,
    "approval_date": "2019-08-24",
    "expiration_date": "2019-08-24",
    "is_remaining": true,
    "main_user": {
      "user_id": "string",
      "handle_name": "string",
      "screen_name": "string",
      "slack_id": "string",
      "is_admin": true,
      "is_member": true,
      "graduation_date": "2019-08-24",
      "remarks": "string"
    },
    "remarks": "string"
  }
]
```

<h3 id="get__public-items-responses">Responses</h3>

|Status|Meaning|Description|Schema|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|備品一覧|Inline|

<h3 id="get__public-items-responseschema">Response Schema</h3>

Status Code **200**

|Name|Type|Required|Restrictions|Description|
|---|---|---|---|---|
|*anonymous*|[[PublicItem](#schemapublicitem)]|false|none|none|
|» public_item_id|string|true|none|備品のユニークID|
|» name|string|true|none|備品名|
|» category|[Category](#schemacategory)|false|none|none|
|»» category_id|string|true|none|カテゴリのユニークID|
|»» name|string|true|none|カテゴリ名|
|»» remarks|string|false|none|備考欄|
|» cost|integer|false|none|備品の購入コスト|
|» approval_date|string(date)|false|none|承認日|
|» expiration_date|string(date)|false|none|耐用期限|
|» is_remaining|boolean|true|none|現存しているか|
|» main_user|[User](#schemauser)|false|none|none|
|»» user_id|string|true|none|ユーザーのユニークID|
|»» handle_name|string|true|none|ユーザーのハンドルネーム|
|»» screen_name|string|true|none|ユーザーのスクリーンネーム|
|»» slack_id|string|false|none|ユーザーのSlack ID|
|»» is_admin|boolean|false|none|管理者フラグ|
|»» is_member|boolean|false|none|在籍状況|
|»» graduation_date|string(date)|false|none|卒業日|
|»» remarks|string|false|none|備考欄|
|» remarks|string|false|none|備考欄|

<aside class="success">
This operation does not require authentication
</aside>

## post__public-items

`POST /public-items`

*備品新規登録*

> Body parameter

```json
{
  "name": "string",
  "cost": 0,
  "purchase_date": "2019-08-24",
  "expiration_date": "2019-08-24",
  "is_remaining": true,
  "purchase_request_id": "string",
  "remarks": "string"
}
```

<h3 id="post__public-items-parameters">Parameters</h3>

|Name|In|Type|Required|Description|
|---|---|---|---|---|
|body|body|[PublicItemEntry](#schemapublicitementry)|true|none|

> Example responses

> 201 Response

```json
{
  "public_item_id": "string",
  "name": "string",
  "product": {
    "product_id": "string",
    "name": "string",
    "model_number": "string",
    "product_url": "string",
    "categiries": [
      {
        "category_id": "string",
        "name": "string",
        "remarks": "string"
      }
    ],
    "main_users": [
      {
        "user_id": "string",
        "handle_name": "string",
        "screen_name": "string",
        "slack_id": "string",
        "is_admin": true,
        "is_member": true,
        "graduation_date": "2019-08-24",
        "remarks": "string"
      }
    ],
    "remarks": "string"
  },
  "cost": 0,
  "purchase_date": "2019-08-24",
  "expiration_date": "2019-08-24",
  "is_remaining": true,
  "purchase_request_id": "string",
  "remarks": "string"
}
```

<h3 id="post__public-items-responses">Responses</h3>

|Status|Meaning|Description|Schema|
|---|---|---|---|
|201|[Created](https://tools.ietf.org/html/rfc7231#section-6.3.2)|備品登録成功|[PublicItemDetails](#schemapublicitemdetails)|
|400|[Bad Request](https://tools.ietf.org/html/rfc7231#section-6.5.1)|必須項目の入力漏れなど|None|

<aside class="success">
This operation does not require authentication
</aside>

## get__public-items_{public-item-id}

`GET /public-items/{public-item-id}`

*備品情報取得*

<h3 id="get__public-items_{public-item-id}-parameters">Parameters</h3>

|Name|In|Type|Required|Description|
|---|---|---|---|---|
|public-item-id|path|string|true|備品のID|

> Example responses

> 200 Response

```json
{
  "public_item_id": "string",
  "name": "string",
  "product": {
    "product_id": "string",
    "name": "string",
    "model_number": "string",
    "product_url": "string",
    "categiries": [
      {
        "category_id": "string",
        "name": "string",
        "remarks": "string"
      }
    ],
    "main_users": [
      {
        "user_id": "string",
        "handle_name": "string",
        "screen_name": "string",
        "slack_id": "string",
        "is_admin": true,
        "is_member": true,
        "graduation_date": "2019-08-24",
        "remarks": "string"
      }
    ],
    "remarks": "string"
  },
  "cost": 0,
  "purchase_date": "2019-08-24",
  "expiration_date": "2019-08-24",
  "is_remaining": true,
  "purchase_request_id": "string",
  "remarks": "string"
}
```

<h3 id="get__public-items_{public-item-id}-responses">Responses</h3>

|Status|Meaning|Description|Schema|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|備品情報|[PublicItemDetails](#schemapublicitemdetails)|
|404|[Not Found](https://tools.ietf.org/html/rfc7231#section-6.5.4)|備品が見つかりません|None|

<aside class="success">
This operation does not require authentication
</aside>

## put__public-items_{public-item-id}

`PUT /public-items/{public-item-id}`

*備品情報更新*

> Body parameter

```json
{
  "public_item_id": "string",
  "name": "string",
  "product": {
    "product_id": "string",
    "name": "string",
    "model_number": "string",
    "product_url": "string",
    "categiries": [
      {
        "category_id": "string",
        "name": "string",
        "remarks": "string"
      }
    ],
    "main_users": [
      {
        "user_id": "string",
        "handle_name": "string",
        "screen_name": "string",
        "slack_id": "string",
        "is_admin": true,
        "is_member": true,
        "graduation_date": "2019-08-24",
        "remarks": "string"
      }
    ],
    "remarks": "string"
  },
  "cost": 0,
  "purchase_date": "2019-08-24",
  "expiration_date": "2019-08-24",
  "is_remaining": true,
  "purchase_request_id": "string",
  "remarks": "string"
}
```

<h3 id="put__public-items_{public-item-id}-parameters">Parameters</h3>

|Name|In|Type|Required|Description|
|---|---|---|---|---|
|public-item-id|path|string|true|備品のID|
|body|body|[PublicItemDetails](#schemapublicitemdetails)|true|none|

> Example responses

> 200 Response

```json
{
  "public_item_id": "string",
  "name": "string",
  "product": {
    "product_id": "string",
    "name": "string",
    "model_number": "string",
    "product_url": "string",
    "categiries": [
      {
        "category_id": "string",
        "name": "string",
        "remarks": "string"
      }
    ],
    "main_users": [
      {
        "user_id": "string",
        "handle_name": "string",
        "screen_name": "string",
        "slack_id": "string",
        "is_admin": true,
        "is_member": true,
        "graduation_date": "2019-08-24",
        "remarks": "string"
      }
    ],
    "remarks": "string"
  },
  "cost": 0,
  "purchase_date": "2019-08-24",
  "expiration_date": "2019-08-24",
  "is_remaining": true,
  "purchase_request_id": "string",
  "remarks": "string"
}
```

<h3 id="put__public-items_{public-item-id}-responses">Responses</h3>

|Status|Meaning|Description|Schema|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|備品情報更新成功|[PublicItemDetails](#schemapublicitemdetails)|
|400|[Bad Request](https://tools.ietf.org/html/rfc7231#section-6.5.1)|必須項目の入力漏れなど|None|
|404|[Not Found](https://tools.ietf.org/html/rfc7231#section-6.5.4)|備品が見つかりません|None|

<aside class="success">
This operation does not require authentication
</aside>

## delete__public-items_{public-item-id}

`DELETE /public-items/{public-item-id}`

*備品削除*

<h3 id="delete__public-items_{public-item-id}-parameters">Parameters</h3>

|Name|In|Type|Required|Description|
|---|---|---|---|---|
|public-item-id|path|string|true|備品のID|

<h3 id="delete__public-items_{public-item-id}-responses">Responses</h3>

|Status|Meaning|Description|Schema|
|---|---|---|---|
|204|[No Content](https://tools.ietf.org/html/rfc7231#section-6.3.5)|備品削除成功|None|
|404|[Not Found](https://tools.ietf.org/html/rfc7231#section-6.5.4)|備品が見つかりません|None|

<aside class="success">
This operation does not require authentication
</aside>

<h1 id="kemomimi-api-privateitems">PrivateItems</h1>

私物

## get__private-items

`GET /private-items`

*私物一覧取得*

> Example responses

> 200 Response

```json
[
  {
    "private_item_id": "string",
    "name": "string",
    "owner_id": "string",
    "post_grad_treat_id": "string",
    "model_number": "string",
    "is_remaining": true,
    "remarks": "string"
  }
]
```

<h3 id="get__private-items-responses">Responses</h3>

|Status|Meaning|Description|Schema|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|私物一覧|Inline|

<h3 id="get__private-items-responseschema">Response Schema</h3>

Status Code **200**

|Name|Type|Required|Restrictions|Description|
|---|---|---|---|---|
|*anonymous*|[[PrivateItem](#schemaprivateitem)]|false|none|none|
|» private_item_id|string|true|none|私物のユニークID|
|» name|string|false|none|製品名|
|» owner_id|string|false|none|所有者ID|
|» post_grad_treat_id|string|false|none|卒業後の処理ID|
|» model_number|string|false|none|型番|
|» is_remaining|boolean|false|none|現存しているか|
|» remarks|string|false|none|備考欄|

<aside class="success">
This operation does not require authentication
</aside>

## post__private-items

`POST /private-items`

*私物新規登録*

> Body parameter

```json
{
  "private_item_id": "string",
  "name": "string",
  "owner_id": "string",
  "post_grad_treat_id": "string",
  "model_number": "string",
  "is_remaining": true,
  "remarks": "string"
}
```

<h3 id="post__private-items-parameters">Parameters</h3>

|Name|In|Type|Required|Description|
|---|---|---|---|---|
|body|body|[PrivateItem](#schemaprivateitem)|true|none|

> Example responses

> 201 Response

```json
{
  "private_item_id": "string",
  "name": "string",
  "owner_id": "string",
  "post_grad_treat_id": "string",
  "model_number": "string",
  "is_remaining": true,
  "remarks": "string"
}
```

<h3 id="post__private-items-responses">Responses</h3>

|Status|Meaning|Description|Schema|
|---|---|---|---|
|201|[Created](https://tools.ietf.org/html/rfc7231#section-6.3.2)|私物登録成功|[PrivateItem](#schemaprivateitem)|
|400|[Bad Request](https://tools.ietf.org/html/rfc7231#section-6.5.1)|必須項目の入力漏れなど|None|

<aside class="success">
This operation does not require authentication
</aside>

## get__private-items_{private-item-id}

`GET /private-items/{private-item-id}`

*私物情報取得*

<h3 id="get__private-items_{private-item-id}-parameters">Parameters</h3>

|Name|In|Type|Required|Description|
|---|---|---|---|---|
|private-item-id|path|string|true|私物のID|

> Example responses

> 200 Response

```json
{
  "private_item_id": "string",
  "name": "string",
  "owner_id": "string",
  "post_grad_treat_id": "string",
  "model_number": "string",
  "is_remaining": true,
  "remarks": "string"
}
```

<h3 id="get__private-items_{private-item-id}-responses">Responses</h3>

|Status|Meaning|Description|Schema|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|私物情報|[PrivateItem](#schemaprivateitem)|
|404|[Not Found](https://tools.ietf.org/html/rfc7231#section-6.5.4)|私物が見つかりません|None|

<aside class="success">
This operation does not require authentication
</aside>

## put__private-items_{private-item-id}

`PUT /private-items/{private-item-id}`

*私物情報更新*

> Body parameter

```json
{
  "private_item_id": "string",
  "name": "string",
  "owner_id": "string",
  "post_grad_treat_id": "string",
  "model_number": "string",
  "is_remaining": true,
  "remarks": "string"
}
```

<h3 id="put__private-items_{private-item-id}-parameters">Parameters</h3>

|Name|In|Type|Required|Description|
|---|---|---|---|---|
|private-item-id|path|string|true|私物のID|
|body|body|[PrivateItem](#schemaprivateitem)|true|none|

> Example responses

> 200 Response

```json
{
  "private_item_id": "string",
  "name": "string",
  "owner_id": "string",
  "post_grad_treat_id": "string",
  "model_number": "string",
  "is_remaining": true,
  "remarks": "string"
}
```

<h3 id="put__private-items_{private-item-id}-responses">Responses</h3>

|Status|Meaning|Description|Schema|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|私物情報更新成功|[PrivateItem](#schemaprivateitem)|
|400|[Bad Request](https://tools.ietf.org/html/rfc7231#section-6.5.1)|必須項目の入力漏れなど|None|
|404|[Not Found](https://tools.ietf.org/html/rfc7231#section-6.5.4)|私物が見つかりません|None|

<aside class="success">
This operation does not require authentication
</aside>

## delete__private-items_{private-item-id}

`DELETE /private-items/{private-item-id}`

*私物削除*

<h3 id="delete__private-items_{private-item-id}-parameters">Parameters</h3>

|Name|In|Type|Required|Description|
|---|---|---|---|---|
|private-item-id|path|string|true|私物のID|

<h3 id="delete__private-items_{private-item-id}-responses">Responses</h3>

|Status|Meaning|Description|Schema|
|---|---|---|---|
|204|[No Content](https://tools.ietf.org/html/rfc7231#section-6.3.5)|私物削除成功|None|
|404|[Not Found](https://tools.ietf.org/html/rfc7231#section-6.5.4)|私物が見つかりません|None|

<aside class="success">
This operation does not require authentication
</aside>

<h1 id="kemomimi-api-categories">Categories</h1>

分類

## get__categories

`GET /categories`

*カテゴリ一覧取得*

> Example responses

> 200 Response

```json
[
  {
    "category_id": "string",
    "name": "string",
    "remarks": "string"
  }
]
```

<h3 id="get__categories-responses">Responses</h3>

|Status|Meaning|Description|Schema|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|カテゴリ一覧|Inline|

<h3 id="get__categories-responseschema">Response Schema</h3>

Status Code **200**

|Name|Type|Required|Restrictions|Description|
|---|---|---|---|---|
|*anonymous*|[[Category](#schemacategory)]|false|none|none|
|» category_id|string|true|none|カテゴリのユニークID|
|» name|string|true|none|カテゴリ名|
|» remarks|string|false|none|備考欄|

<aside class="success">
This operation does not require authentication
</aside>

## post__categories

`POST /categories`

*カテゴリ新規登録*

> Body parameter

```json
{
  "category_id": "string",
  "name": "string",
  "remarks": "string"
}
```

<h3 id="post__categories-parameters">Parameters</h3>

|Name|In|Type|Required|Description|
|---|---|---|---|---|
|body|body|[Category](#schemacategory)|true|none|

> Example responses

> 201 Response

```json
{
  "category_id": "string",
  "name": "string",
  "remarks": "string"
}
```

<h3 id="post__categories-responses">Responses</h3>

|Status|Meaning|Description|Schema|
|---|---|---|---|
|201|[Created](https://tools.ietf.org/html/rfc7231#section-6.3.2)|カテゴリ登録成功|[Category](#schemacategory)|
|400|[Bad Request](https://tools.ietf.org/html/rfc7231#section-6.5.1)|必須項目の入力漏れなど|None|

<aside class="success">
This operation does not require authentication
</aside>

## get__categories_{category-id}

`GET /categories/{category-id}`

*カテゴリ情報取得*

<h3 id="get__categories_{category-id}-parameters">Parameters</h3>

|Name|In|Type|Required|Description|
|---|---|---|---|---|
|category-id|path|string|true|カテゴリのID|

> Example responses

> 200 Response

```json
{
  "category_id": "string",
  "name": "string",
  "remarks": "string"
}
```

<h3 id="get__categories_{category-id}-responses">Responses</h3>

|Status|Meaning|Description|Schema|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|カテゴリ情報|[Category](#schemacategory)|
|404|[Not Found](https://tools.ietf.org/html/rfc7231#section-6.5.4)|カテゴリが見つかりません|None|

<aside class="success">
This operation does not require authentication
</aside>

## put__categories_{category-id}

`PUT /categories/{category-id}`

*カテゴリ情報更新*

> Body parameter

```json
{
  "category_id": "string",
  "name": "string",
  "remarks": "string"
}
```

<h3 id="put__categories_{category-id}-parameters">Parameters</h3>

|Name|In|Type|Required|Description|
|---|---|---|---|---|
|category-id|path|string|true|カテゴリのID|
|body|body|[Category](#schemacategory)|true|none|

> Example responses

> 200 Response

```json
{
  "category_id": "string",
  "name": "string",
  "remarks": "string"
}
```

<h3 id="put__categories_{category-id}-responses">Responses</h3>

|Status|Meaning|Description|Schema|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|カテゴリ情報更新成功|[Category](#schemacategory)|
|400|[Bad Request](https://tools.ietf.org/html/rfc7231#section-6.5.1)|必須項目の入力漏れなど|None|
|404|[Not Found](https://tools.ietf.org/html/rfc7231#section-6.5.4)|カテゴリが見つかりません|None|

<aside class="success">
This operation does not require authentication
</aside>

## delete__categories_{category-id}

`DELETE /categories/{category-id}`

*カテゴリ削除*

<h3 id="delete__categories_{category-id}-parameters">Parameters</h3>

|Name|In|Type|Required|Description|
|---|---|---|---|---|
|category-id|path|string|true|カテゴリのID|

<h3 id="delete__categories_{category-id}-responses">Responses</h3>

|Status|Meaning|Description|Schema|
|---|---|---|---|
|204|[No Content](https://tools.ietf.org/html/rfc7231#section-6.3.5)|カテゴリ削除成功|None|
|404|[Not Found](https://tools.ietf.org/html/rfc7231#section-6.5.4)|カテゴリが見つかりません|None|

<aside class="success">
This operation does not require authentication
</aside>

<h1 id="kemomimi-api-users">Users</h1>

部員

## get__users

`GET /users`

*部員一覧取得*

> Example responses

> 200 Response

```json
[
  {
    "user_id": "string",
    "handle_name": "string",
    "screen_name": "string",
    "slack_id": "string",
    "is_admin": true,
    "is_member": true,
    "graduation_date": "2019-08-24",
    "remarks": "string"
  }
]
```

<h3 id="get__users-responses">Responses</h3>

|Status|Meaning|Description|Schema|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|部員一覧|Inline|

<h3 id="get__users-responseschema">Response Schema</h3>

Status Code **200**

|Name|Type|Required|Restrictions|Description|
|---|---|---|---|---|
|*anonymous*|[[User](#schemauser)]|false|none|none|
|» user_id|string|true|none|ユーザーのユニークID|
|» handle_name|string|true|none|ユーザーのハンドルネーム|
|» screen_name|string|true|none|ユーザーのスクリーンネーム|
|» slack_id|string|false|none|ユーザーのSlack ID|
|» is_admin|boolean|false|none|管理者フラグ|
|» is_member|boolean|false|none|在籍状況|
|» graduation_date|string(date)|false|none|卒業日|
|» remarks|string|false|none|備考欄|

<aside class="success">
This operation does not require authentication
</aside>

## post__users

`POST /users`

*部員新規登録*

> Body parameter

```json
{
  "user_id": "string",
  "handle_name": "string",
  "screen_name": "string",
  "slack_id": "string",
  "is_admin": true,
  "is_member": true,
  "graduation_date": "2019-08-24",
  "remarks": "string"
}
```

<h3 id="post__users-parameters">Parameters</h3>

|Name|In|Type|Required|Description|
|---|---|---|---|---|
|body|body|[User](#schemauser)|true|none|

> Example responses

> 201 Response

```json
{
  "user_id": "string",
  "handle_name": "string",
  "screen_name": "string",
  "slack_id": "string",
  "is_admin": true,
  "is_member": true,
  "graduation_date": "2019-08-24",
  "remarks": "string"
}
```

<h3 id="post__users-responses">Responses</h3>

|Status|Meaning|Description|Schema|
|---|---|---|---|
|201|[Created](https://tools.ietf.org/html/rfc7231#section-6.3.2)|部員登録成功|[User](#schemauser)|
|400|[Bad Request](https://tools.ietf.org/html/rfc7231#section-6.5.1)|必須項目の入力漏れなど|None|

<aside class="success">
This operation does not require authentication
</aside>

## get__users-{user_id}

`GET /users-{user_id}`

*部員情報取得*

<h3 id="get__users-{user_id}-parameters">Parameters</h3>

|Name|In|Type|Required|Description|
|---|---|---|---|---|
|user_id|path|string|true|部員のID|

> Example responses

> 200 Response

```json
{
  "user_id": "string",
  "handle_name": "string",
  "screen_name": "string",
  "slack_id": "string",
  "is_admin": true,
  "is_member": true,
  "graduation_date": "2019-08-24",
  "remarks": "string"
}
```

<h3 id="get__users-{user_id}-responses">Responses</h3>

|Status|Meaning|Description|Schema|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|部員情報|[User](#schemauser)|
|404|[Not Found](https://tools.ietf.org/html/rfc7231#section-6.5.4)|部員が見つかりません|None|

<aside class="success">
This operation does not require authentication
</aside>

## put__users-{user_id}

`PUT /users-{user_id}`

*部員情報更新*

> Body parameter

```json
{
  "user_id": "string",
  "handle_name": "string",
  "screen_name": "string",
  "slack_id": "string",
  "is_admin": true,
  "is_member": true,
  "graduation_date": "2019-08-24",
  "remarks": "string"
}
```

<h3 id="put__users-{user_id}-parameters">Parameters</h3>

|Name|In|Type|Required|Description|
|---|---|---|---|---|
|user_id|path|string|true|部員のID|
|body|body|[User](#schemauser)|true|none|

> Example responses

> 200 Response

```json
{
  "user_id": "string",
  "handle_name": "string",
  "screen_name": "string",
  "slack_id": "string",
  "is_admin": true,
  "is_member": true,
  "graduation_date": "2019-08-24",
  "remarks": "string"
}
```

<h3 id="put__users-{user_id}-responses">Responses</h3>

|Status|Meaning|Description|Schema|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|部員情報更新成功|[User](#schemauser)|
|400|[Bad Request](https://tools.ietf.org/html/rfc7231#section-6.5.1)|必須項目の入力漏れなど|None|
|404|[Not Found](https://tools.ietf.org/html/rfc7231#section-6.5.4)|部員が見つかりません|None|

<aside class="success">
This operation does not require authentication
</aside>

## delete__users-{user_id}

`DELETE /users-{user_id}`

*部員削除*

<h3 id="delete__users-{user_id}-parameters">Parameters</h3>

|Name|In|Type|Required|Description|
|---|---|---|---|---|
|user_id|path|string|true|部員のID|

<h3 id="delete__users-{user_id}-responses">Responses</h3>

|Status|Meaning|Description|Schema|
|---|---|---|---|
|204|[No Content](https://tools.ietf.org/html/rfc7231#section-6.3.5)|部員削除成功|None|
|404|[Not Found](https://tools.ietf.org/html/rfc7231#section-6.5.4)|部員が見つかりません|None|

<aside class="success">
This operation does not require authentication
</aside>

<h1 id="kemomimi-api-products">products</h1>

## get__products

`GET /products`

*製品一覧取得*

> Example responses

> 200 Response

```json
[
  {
    "product_id": "string",
    "name": "string",
    "model_number": "string",
    "product_url": "string",
    "categiries": [
      {
        "category_id": "string",
        "name": "string",
        "remarks": "string"
      }
    ],
    "main_users": [
      {
        "user_id": "string",
        "handle_name": "string",
        "screen_name": "string",
        "slack_id": "string",
        "is_admin": true,
        "is_member": true,
        "graduation_date": "2019-08-24",
        "remarks": "string"
      }
    ],
    "remarks": "string"
  }
]
```

<h3 id="get__products-responses">Responses</h3>

|Status|Meaning|Description|Schema|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|製品一覧|Inline|

<h3 id="get__products-responseschema">Response Schema</h3>

Status Code **200**

|Name|Type|Required|Restrictions|Description|
|---|---|---|---|---|
|*anonymous*|[[Product](#schemaproduct)]|false|none|none|
|» product_id|string|true|none|製品のユニークID|
|» name|string|true|none|製品名|
|» model_number|string|false|none|型番|
|» product_url|string|false|none|商品のURL|
|» categiries|[[Category](#schemacategory)]|false|none|none|
|»» category_id|string|true|none|カテゴリのユニークID|
|»» name|string|true|none|カテゴリ名|
|»» remarks|string|false|none|備考欄|
|» main_users|[[User](#schemauser)]|false|none|none|
|»» user_id|string|true|none|ユーザーのユニークID|
|»» handle_name|string|true|none|ユーザーのハンドルネーム|
|»» screen_name|string|true|none|ユーザーのスクリーンネーム|
|»» slack_id|string|false|none|ユーザーのSlack ID|
|»» is_admin|boolean|false|none|管理者フラグ|
|»» is_member|boolean|false|none|在籍状況|
|»» graduation_date|string(date)|false|none|卒業日|
|»» remarks|string|false|none|備考欄|
|» remarks|string|false|none|備考欄|

<aside class="success">
This operation does not require authentication
</aside>

## post__products

`POST /products`

*製品新規登録*

> Body parameter

```json
{
  "product_id": "string",
  "name": "string",
  "model_number": "string",
  "product_url": "string",
  "categiries": [
    {
      "category_id": "string",
      "name": "string",
      "remarks": "string"
    }
  ],
  "main_users": [
    {
      "user_id": "string",
      "handle_name": "string",
      "screen_name": "string",
      "slack_id": "string",
      "is_admin": true,
      "is_member": true,
      "graduation_date": "2019-08-24",
      "remarks": "string"
    }
  ],
  "remarks": "string"
}
```

<h3 id="post__products-parameters">Parameters</h3>

|Name|In|Type|Required|Description|
|---|---|---|---|---|
|body|body|[Product](#schemaproduct)|true|none|

> Example responses

> 201 Response

```json
{
  "product_id": "string",
  "name": "string",
  "model_number": "string",
  "product_url": "string",
  "categiries": [
    {
      "category_id": "string",
      "name": "string",
      "remarks": "string"
    }
  ],
  "main_users": [
    {
      "user_id": "string",
      "handle_name": "string",
      "screen_name": "string",
      "slack_id": "string",
      "is_admin": true,
      "is_member": true,
      "graduation_date": "2019-08-24",
      "remarks": "string"
    }
  ],
  "remarks": "string"
}
```

<h3 id="post__products-responses">Responses</h3>

|Status|Meaning|Description|Schema|
|---|---|---|---|
|201|[Created](https://tools.ietf.org/html/rfc7231#section-6.3.2)|製品登録成功|[Product](#schemaproduct)|
|400|[Bad Request](https://tools.ietf.org/html/rfc7231#section-6.5.1)|必須項目の入力漏れなど|None|

<aside class="success">
This operation does not require authentication
</aside>

## get__products_{product-id}

`GET /products/{product-id}`

*製品情報取得*

<h3 id="get__products_{product-id}-parameters">Parameters</h3>

|Name|In|Type|Required|Description|
|---|---|---|---|---|
|product-id|path|string|true|製品のID|

> Example responses

> 200 Response

```json
{
  "product_id": "string",
  "name": "string",
  "model_number": "string",
  "product_url": "string",
  "categiries": [
    {
      "category_id": "string",
      "name": "string",
      "remarks": "string"
    }
  ],
  "main_users": [
    {
      "user_id": "string",
      "handle_name": "string",
      "screen_name": "string",
      "slack_id": "string",
      "is_admin": true,
      "is_member": true,
      "graduation_date": "2019-08-24",
      "remarks": "string"
    }
  ],
  "remarks": "string"
}
```

<h3 id="get__products_{product-id}-responses">Responses</h3>

|Status|Meaning|Description|Schema|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|製品情報|[Product](#schemaproduct)|
|404|[Not Found](https://tools.ietf.org/html/rfc7231#section-6.5.4)|製品が見つかりません|None|

<aside class="success">
This operation does not require authentication
</aside>

## put__products_{product-id}

`PUT /products/{product-id}`

*製品情報更新*

> Body parameter

```json
{
  "product_id": "string",
  "name": "string",
  "model_number": "string",
  "product_url": "string",
  "categiries": [
    {
      "category_id": "string",
      "name": "string",
      "remarks": "string"
    }
  ],
  "main_users": [
    {
      "user_id": "string",
      "handle_name": "string",
      "screen_name": "string",
      "slack_id": "string",
      "is_admin": true,
      "is_member": true,
      "graduation_date": "2019-08-24",
      "remarks": "string"
    }
  ],
  "remarks": "string"
}
```

<h3 id="put__products_{product-id}-parameters">Parameters</h3>

|Name|In|Type|Required|Description|
|---|---|---|---|---|
|product-id|path|string|true|製品のID|
|body|body|[Product](#schemaproduct)|true|none|

> Example responses

> 200 Response

```json
{
  "product_id": "string",
  "name": "string",
  "model_number": "string",
  "product_url": "string",
  "categiries": [
    {
      "category_id": "string",
      "name": "string",
      "remarks": "string"
    }
  ],
  "main_users": [
    {
      "user_id": "string",
      "handle_name": "string",
      "screen_name": "string",
      "slack_id": "string",
      "is_admin": true,
      "is_member": true,
      "graduation_date": "2019-08-24",
      "remarks": "string"
    }
  ],
  "remarks": "string"
}
```

<h3 id="put__products_{product-id}-responses">Responses</h3>

|Status|Meaning|Description|Schema|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|製品情報更新成功|[Product](#schemaproduct)|
|400|[Bad Request](https://tools.ietf.org/html/rfc7231#section-6.5.1)|必須項目の入力漏れなど|None|
|404|[Not Found](https://tools.ietf.org/html/rfc7231#section-6.5.4)|製品が見つかりません|None|

<aside class="success">
This operation does not require authentication
</aside>

## delete__products_{product-id}

`DELETE /products/{product-id}`

*製品削除*

<h3 id="delete__products_{product-id}-parameters">Parameters</h3>

|Name|In|Type|Required|Description|
|---|---|---|---|---|
|product-id|path|string|true|製品のID|

<h3 id="delete__products_{product-id}-responses">Responses</h3>

|Status|Meaning|Description|Schema|
|---|---|---|---|
|204|[No Content](https://tools.ietf.org/html/rfc7231#section-6.3.5)|製品削除成功|None|
|404|[Not Found](https://tools.ietf.org/html/rfc7231#section-6.5.4)|製品が見つかりません|None|

<aside class="success">
This operation does not require authentication
</aside>

# Schemas

<h2 id="tocS_PublicItemDetails">PublicItemDetails</h2>
<!-- backwards compatibility -->
<a id="schemapublicitemdetails"></a>
<a id="schema_PublicItemDetails"></a>
<a id="tocSpublicitemdetails"></a>
<a id="tocspublicitemdetails"></a>

```json
{
  "public_item_id": "string",
  "name": "string",
  "product": {
    "product_id": "string",
    "name": "string",
    "model_number": "string",
    "product_url": "string",
    "categiries": [
      {
        "category_id": "string",
        "name": "string",
        "remarks": "string"
      }
    ],
    "main_users": [
      {
        "user_id": "string",
        "handle_name": "string",
        "screen_name": "string",
        "slack_id": "string",
        "is_admin": true,
        "is_member": true,
        "graduation_date": "2019-08-24",
        "remarks": "string"
      }
    ],
    "remarks": "string"
  },
  "cost": 0,
  "purchase_date": "2019-08-24",
  "expiration_date": "2019-08-24",
  "is_remaining": true,
  "purchase_request_id": "string",
  "remarks": "string"
}

```

### Properties

|Name|Type|Required|Restrictions|Description|
|---|---|---|---|---|
|public_item_id|string|true|none|備品のユニークID|
|name|string|false|none|備品名|
|product|[Product](#schemaproduct)|false|none|製品|
|cost|integer|false|none|備品の購入コスト|
|purchase_date|string(date)|false|none|導入日|
|expiration_date|string(date)|false|none|耐用期限|
|is_remaining|boolean|false|none|現存しているか|
|purchase_request_id|string|false|none|追加元の購入申請ID|
|remarks|string|false|none|備考欄|

<h2 id="tocS_PublicItemEntry">PublicItemEntry</h2>
<!-- backwards compatibility -->
<a id="schemapublicitementry"></a>
<a id="schema_PublicItemEntry"></a>
<a id="tocSpublicitementry"></a>
<a id="tocspublicitementry"></a>

```json
{
  "name": "string",
  "cost": 0,
  "purchase_date": "2019-08-24",
  "expiration_date": "2019-08-24",
  "is_remaining": true,
  "purchase_request_id": "string",
  "remarks": "string"
}

```

### Properties

|Name|Type|Required|Restrictions|Description|
|---|---|---|---|---|
|name|string|true|none|備品名|
|cost|integer|false|none|備品の購入コスト(NULLなら申請から)|
|purchase_date|string(date)|false|none|導入日(NULLなら現時刻)|
|expiration_date|string(date)|false|none|耐用期限|
|is_remaining|boolean|false|none|現存しているか(NULLなら現存)|
|purchase_request_id|string|true|none|追加元の購入申請ID|
|remarks|string|false|none|備考欄|

<h2 id="tocS_PublicItem">PublicItem</h2>
<!-- backwards compatibility -->
<a id="schemapublicitem"></a>
<a id="schema_PublicItem"></a>
<a id="tocSpublicitem"></a>
<a id="tocspublicitem"></a>

```json
{
  "public_item_id": "string",
  "name": "string",
  "category": {
    "category_id": "string",
    "name": "string",
    "remarks": "string"
  },
  "cost": 0,
  "approval_date": "2019-08-24",
  "expiration_date": "2019-08-24",
  "is_remaining": true,
  "main_user": {
    "user_id": "string",
    "handle_name": "string",
    "screen_name": "string",
    "slack_id": "string",
    "is_admin": true,
    "is_member": true,
    "graduation_date": "2019-08-24",
    "remarks": "string"
  },
  "remarks": "string"
}

```

### Properties

|Name|Type|Required|Restrictions|Description|
|---|---|---|---|---|
|public_item_id|string|true|none|備品のユニークID|
|name|string|true|none|備品名|
|category|[Category](#schemacategory)|false|none|カテゴリ(1つ)|
|cost|integer|false|none|備品の購入コスト|
|approval_date|string(date)|false|none|承認日|
|expiration_date|string(date)|false|none|耐用期限|
|is_remaining|boolean|true|none|現存しているか|
|main_user|[User](#schemauser)|false|none|主要な利用者(1人)|
|remarks|string|false|none|備考欄|

<h2 id="tocS_PrivateItem">PrivateItem</h2>
<!-- backwards compatibility -->
<a id="schemaprivateitem"></a>
<a id="schema_PrivateItem"></a>
<a id="tocSprivateitem"></a>
<a id="tocsprivateitem"></a>

```json
{
  "private_item_id": "string",
  "name": "string",
  "owner_id": "string",
  "post_grad_treat_id": "string",
  "model_number": "string",
  "is_remaining": true,
  "remarks": "string"
}

```

### Properties

|Name|Type|Required|Restrictions|Description|
|---|---|---|---|---|
|private_item_id|string|true|none|私物のユニークID|
|name|string|false|none|製品名|
|owner_id|string|false|none|所有者ID|
|post_grad_treat_id|string|false|none|卒業後の処理ID|
|model_number|string|false|none|型番|
|is_remaining|boolean|false|none|現存しているか|
|remarks|string|false|none|備考欄|

<h2 id="tocS_Product">Product</h2>
<!-- backwards compatibility -->
<a id="schemaproduct"></a>
<a id="schema_Product"></a>
<a id="tocSproduct"></a>
<a id="tocsproduct"></a>

```json
{
  "product_id": "string",
  "name": "string",
  "model_number": "string",
  "product_url": "string",
  "categiries": [
    {
      "category_id": "string",
      "name": "string",
      "remarks": "string"
    }
  ],
  "main_users": [
    {
      "user_id": "string",
      "handle_name": "string",
      "screen_name": "string",
      "slack_id": "string",
      "is_admin": true,
      "is_member": true,
      "graduation_date": "2019-08-24",
      "remarks": "string"
    }
  ],
  "remarks": "string"
}

```

### Properties

|Name|Type|Required|Restrictions|Description|
|---|---|---|---|---|
|product_id|string|true|none|製品のユニークID|
|name|string|true|none|製品名|
|model_number|string|false|none|型番|
|product_url|string|false|none|商品のURL|
|categiries|[[Category](#schemacategory)]|false|none|none|
|main_users|[[User](#schemauser)]|false|none|none|
|remarks|string|false|none|備考欄|

<h2 id="tocS_Category">Category</h2>
<!-- backwards compatibility -->
<a id="schemacategory"></a>
<a id="schema_Category"></a>
<a id="tocScategory"></a>
<a id="tocscategory"></a>

```json
{
  "category_id": "string",
  "name": "string",
  "remarks": "string"
}

```

### Properties

|Name|Type|Required|Restrictions|Description|
|---|---|---|---|---|
|category_id|string|true|none|カテゴリのユニークID|
|name|string|true|none|カテゴリ名|
|remarks|string|false|none|備考欄|

<h2 id="tocS_User">User</h2>
<!-- backwards compatibility -->
<a id="schemauser"></a>
<a id="schema_User"></a>
<a id="tocSuser"></a>
<a id="tocsuser"></a>

```json
{
  "user_id": "string",
  "handle_name": "string",
  "screen_name": "string",
  "slack_id": "string",
  "is_admin": true,
  "is_member": true,
  "graduation_date": "2019-08-24",
  "remarks": "string"
}

```

### Properties

|Name|Type|Required|Restrictions|Description|
|---|---|---|---|---|
|user_id|string|true|none|ユーザーのユニークID|
|handle_name|string|true|none|ユーザーのハンドルネーム|
|screen_name|string|true|none|ユーザーのスクリーンネーム|
|slack_id|string|false|none|ユーザーのSlack ID|
|is_admin|boolean|false|none|管理者フラグ|
|is_member|boolean|false|none|在籍状況|
|graduation_date|string(date)|false|none|卒業日|
|remarks|string|false|none|備考欄|

