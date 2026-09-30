#!/bin/bash
# cert/ 存在確認
if [ ! -d cert/ ]; then
  mkdir cert
  echo "証明書フォルダ作成"
fi

if [ -e cert/server.key ] && [ -e cert/server.cert ]; then
  echo "証明書はすでに存在します"
  exit 0
fi

if openssl req -x509 -newkey rsa:4096 -keyout "cert/server.key" -out "cert/server.cert" -days 365 -nodes -subj "/C=JP/ST=Tokyo/L=Tokyo/O=grpc/CN=localhost"; then
  echo "証明書の作成に成功しました"
else
  echo "証明書の作成に失敗しました"
  exit 1
fi
