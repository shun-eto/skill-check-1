## 前提

- Rust での開発
- Docker 環境が Local に構築されているっこと
- 出力結果をログとして出力する

## 課題

> Sample1
>
> ```plaintext
> endpoint = localhost:3000
> debug = true
> log.file = /var/log/console.log
> ```
>
> to
>
> ```json
> {
>   "endpoint": "localhost:3000",
>   "debug": true,
>   "log": {
>     "file": "/var/log/console.log"
>   }
> }
> ```

> Sample2
>
> ```plaintext
> {
>   "endpoint": "localhost:3000",
>   "log": {
>     "file": "/var/log/console.log",
>     "name": "default.log",
>   }
> }
> ```
>
> to
>
> ```json
> {
>   "endpoint": "localhost:3000",
>   "log": {
>     "file": "/var/log/console.log",
>     "name": "default.log"
>   }
> }
> ```

**Sample1**と**Sample2**を満たすプログラムの開発

## 実行コマンド

※ 実行環境は v27 系で確認済み

```
docker compose up
```
