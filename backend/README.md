# 基本的な開発情報

## コマンド系

分割しているopenapiファイルを統合する
`swagger-cli bundle -o merged.yaml -t yaml D:\dev\kemomimi\docs\api\openapi.yaml`

生成されたopenapiファイルを確認する
`openapi-generator-cli validate -i "D:\dev\kemomimi\docs\api\merged.yaml"`

コードを生成する
`openapi-generator-cli generate -i "D:\dev\kemomimi\docs\api\merged.yaml" -g rust-axum -o "D:\dev\kemomimi\backend\openapi_gen"`

mdドキュメントを生成する
`npx widdershins --omitHeader --code true merged.yaml openapi.md"`

## その他

- openapi generatorで生成すると`Cargo.toml`のversionがおかしくなってエラーになるので注意
- `#[async_trait]`をつけるとトレイとのメンバを自動でフィルしてくれなくなる
- 