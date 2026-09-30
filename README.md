# ros2-grpc-bridge

React の Web クライアントと ROS 2 を gRPC-Web で接続するブリッジです。
サーバーは Rust（tonic / rclrs）、クライアントは React / Vite を使用します。

## 必要な環境

- Linux / ROS 2 Jazzy（`/opt/ros/jazzy` を想定）
- Rust / Cargo（Rust 2024 edition 対応）
- Node.js 22.12 以降の対応バージョンと npm
- Protocol Buffers コンパイラー `protoc`
- ROS 2 の Rust ビルド環境（rclrs / rosidl_runtime_rs に必要なライブラリ・ビルド設定）
- TLS 証明書の作成に使う `openssl`（開発環境用）

ROS 2 の標準インストールだけで Rust 用の生成物が揃うとは限りません。
特に `main` の Cargo.toml は `/opt/ros/jazzy/share/sensor_msgs/rust` を参照します。
DynamicMessage 版でも、利用するメッセージ型の ROS パッケージと C introspection
type-support ライブラリが必要です。カスタム型を使う場合は、その workspace も source してください。

## ディレクトリ構成

```text
proto/robot.proto        gRPC のサービス・メッセージ定義
src/server/             Rust サーバー
src/client/             React クライアント
cert/server.cert        TLS サーバー証明書（ローカルで用意）
cert/server.key         TLS 秘密鍵（ローカルで用意）
```

以下の `cd` の例は、特記がなければリポジトリのルートから実行します。

## TLS の準備（DynamicMessage 版）
プロジェクトルート`./gen_cert.sh`を実行してTLS証明書が`cert/`に生成されます。

`App.tsx` は `import.meta.env.VITE_GRPC_URL` を読み込み、未設定ならエラーを出します。
`.env.local` はクライアントの `.gitignore` の `*.local` で除外されています。

- `.env` / `.env.local` を変更したら Vite を再起動します。
- 本番ビルドでは値が JavaScript に埋め込まれるため、変更後に再ビルドが必要です。
- `VITE_` 付きの値はブラウザへ公開されます。秘密情報は置きません。
- この変数は Web の接続先のみを設定します。Rust の待受ポートや TLS は変更しません。

詳細: [Vite の環境変数](https://vite.dev/guide/env-and-mode)

## proto の生成

### クライアント（TypeScript）

初回セットアップ、および `proto/robot.proto` の変更後に実行します。

```bash
cd src/client
npm ci
mkdir -p src/gen
npm run gen
```

`protoc` と npm でインストールしたプラグインを使い、`src/gen/proto/` に
メッセージ型と Connect のサービス定義を生成します。生成ファイルは Git 管理対象外です。
ブランチ切替で proto が変わった場合も再生成してください。

### サーバー（Rust）

`cargo build` / `cargo run` 時に `src/server/build.rs` が `proto/` 内の `.proto` を
コンパイルします。Rust コードと reflection 用 descriptor は Cargo の `OUT_DIR` に出力されます。
別途生成コマンドを実行する必要はありません。こちらも `protoc` が必要です。

## サーバーの起動

```bash
source /opt/ros/jazzy/setup.bash
# カスタムメッセージ等がある場合は、その workspace の install/setup.bash も source
cd src/server
cargo run
```

待受は `0.0.0.0:50051` です。`main` は HTTP、DynamicMessage 版は TLS で待ち受けます。
`0.0.0.0` は待受用アドレスなので、ブラウザの接続先には実際のホスト名・IP を指定します。

## クライアントの起動

proto 生成後、別ターミナルで実行します。

```bash
cd src/client
npm run dev
```

同じ PC からアクセスする開発用です。表示された URL を開いてください。
通常のポートは `5173` ですが、使用中の場合は変わることがあります。
DynamicMessage 版は HTTPS、`main` は HTTP です。

別 PC やスマートフォンから LAN 経由で開く場合（DynamicMessage 版）:

```bash
cd src/client
npm run network
```

これは `vite --host` を実行します。例えば `https://ros-bridge.local:5173` で開きます。
`main` には `network` script がないため、代わりに `npm run dev -- --host` を使います。
必要に応じてファイアウォールで Web のポートと `50051/TCP` を許可してください。
`--host` は Web サーバーを LAN に公開する指定であり、gRPC 接続先は変更しません。

Vite の開発サーバーは開発・動作確認用です。ビルドは `npm run build` で行い、成果物は
`src/client/dist/` に生成されます。`server.https` は本番配信サーバーの TLS を設定しません。
詳細: [Vite のサーバー設定](https://vite.dev/config/server-options)

## 動作確認・トラブルシューティング

- `protoc: command not found`: Protocol Buffers コンパイラーをインストールします。
- `src/gen/proto/...` が見つからない: `src/client` で `npm run gen` を実行します。
- 証明書ファイルが見つからない: `cert/` のファイル名と起動ディレクトリを確認します。
- ブラウザの証明書エラー: URL と証明書の名前・IP、接続端末の CA 信頼設定を確認します。
- 接続できない: HTTP/HTTPS、接続先、ポート、ファイアウォールを確認します。
- ROS 型が解決できない: メッセージパッケージと type-support のインストール、source を確認します。

ブラウザから送信した後、同じ ROS 環境のターミナルで topic を確認できます。

```bash
ros2 topic list --no-daemon --spin-time 5
ros2 topic echo /joy sensor_msgs/msg/Joy
```

Wi-Fi 切替後、通常の `ros2 topic list` だけで topic が見えなくなる場合は
`ros2 daemon stop` を実行してから再試行します。次の CLI 操作で daemon が再起動します。
DynamicMessage 版の `/joy` 折返しと異なり、`main` は `/test` を購読するため、
`/joy` に送信しただけではブラウザの受信表示は更新されません。
