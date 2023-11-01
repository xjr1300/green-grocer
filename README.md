# 八百屋アプリ

## 概要／動機

* いまさらながら、「Clean Architecture 達人に学ぶソフトウェアの構造と設計」を読んだ。
* 八百屋販売アプリで、クリーンアーキテクチャ及びドメイン駆動設計で設計または実装することを試行する。

## 目標

* クリーンアーキテクチャ
  * 依存性の方向を意識して設計
* ドメイン駆動設計
  * 集約単位でデータを永続化
  * 集約または集約の内部は、集約ルートを経由してアクセス
  * 集約単位でリポジトリを設計して、ユースケースごとにメソッドを定義
  * 具象リポジトリのユースケースごとにメソッドでは、必要に応じてトランザクションを開始して、メソッドの終了でコミット／ロールバック

## ユースケース

* お客さん
  * 代金を支払い野菜を購入する。
  * 購入した野菜のうち、いずれかの野菜を返品する
  * 購入した野菜をすべて返品する

## 集約、エンティティ、値オブジェクト

* 販売
  * 販売全体の情報と、個々の野菜を販売した実績を示す販売明細を管理する集約
  * ルートエンティティは`販売`
* 販売
  * `販売`集約の集約ルート（ルートエンティティ）
  * 販売日時、販売明細（複数）、合計販売金額などをフィールドに持つ
* 販売明細
  * 販売した野菜、単価、数量、小計などをフィールドに持つ値オブジェクト

## ドメインルール

* お客さんは、1回の購入（`販売`）で、1つ以上の野菜を1以上の数量で購入できる。
  * `販売`エンティティには、必ず1つ以上の`販売明細`が存在する。
  * `販売明細`の数量は、必ず1以上とする。
  * `販売明細`の小計を合計した結果を、`販売`の`合計販売金額`に記録する。
* お客さんは、代金を支払うとその`販売`が確定して、野菜を追加で購入できない。
  * 既存の`販売`エンティティに`販売明細`を追加できない。
  * この場合、お客さんは、別の`販売`として購入する。
* お客さんは、購入した野菜（`販売明細`）のうち、1つの野菜を1以上から購入した数量まで返品できる。
  * `販売明細`は値オブジェクトであるため、返品が発生した時、既存の`販売明細`すべて削除して、新たな`販売明細`を作成する。
    * `販売`はエンティティであるため、必要に応じて更新する。
  * 返品の数量は１以上とする。
  * `販売明細`に記録されている数量から返品した数量を減じた数量を、新たな`販売明細`の数量に記録する。
  * ただし、`販売明細`数量が0になる場合は、当該`販売明細`を記録しない。
  * この結果、`販売明細`がひとつもない場合、購入した野菜がすべて返品されたとして、当該`販売`を削除する。
  * この結果、`販売明細`がひとつ以上存在する場合、それらの小計を合計した金額を`販売`の`合計販売金額`に記録する。
* お客さんは、購入した野菜をすべて返品することで、購入をキャンセルできる。
  * 当該`販売`を削除する。
* 八百屋に野菜の在庫が無限にあるとする（欠品なし、話を簡単にするために`在庫`の概念を持たない。）。

## コンポーネント

上位コンポーネントから下位コンポーネントに向かって次の通り示す。
なお、下位コンポーネントは上位コンポーネントに依存して、上位コンポーネントは下位コンポーネントに依存しない。

* domain
* usecase
* infrastructure
* controller
* web

これを、ワークスペースを構成することで制約する。

```bash
cargo new --bin web
cargo new --lib controller
cargo new --lib infrastructure
cargo new --lib usecase
cargo new --lib domain
```

## リクエスト

```bash
# ヘルスチェック
curl http://localhost:8001/health-check

# 野菜をすべて取得
curl http://localhost:8001/api/vegetables

# 野菜を登録
curl -X POST -H 'Content-Type: application/json' -d '{"id": "953c73a3-0c55-4e72-8288-bbd69b8b70a4", "name": "トマト", "unitPrice": 100}' http://localhost:8001/api/vegetables

# 野菜をIDを指定して取得
curl http://localhost:8001/api/vegetables/953c73a3-0c55-4e72-8288-bbd69b8b70a4

# 野菜を更新
curl -X PUT -H 'Content-Type: application/json' -d '{"id": "953c73a3-0c55-4e72-8288-bbd69b8b70a4", "name": "キュウリ", "unitPrice": 30}' http://localhost:8001/api/vegetables
```
