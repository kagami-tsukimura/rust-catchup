# Rust

## Overview

<details><summary>[Rust Overview]</summary>

### Rust とは

- 2015 年にリリースされた静的型付け言語
- C/C++の代替言語として期待
- コンパイルして Binary 生成
- バックエンド開発にも使用（Golang, Node より高速）
- 難易度高い

### C/C++ との違い

- 次世代システム開発言語
  - 従来は C/C++のみ
- メモリ管理の安全性を保証
  - 所有権による保証
  - C/C++はメモリ自由度が高すぎてバグの温床

### 動的型付け言語と静的型付け言語

- 動的型付け言語

  - データ型が実行時に動的に決まる
  - 実行時に型の整合性チェックで型安全でない・低速
    - 学習コストが低い
    - e.g. Python, JavaScript, Ruby, php

- 静的型付け言語
  - データ型を事前に決める
  - コンパイル時に型の整合性チェックで型安全・高速
    - 学習コストが高い
    - e.g. C, C++, TypeScript, Rust

### GC

- 使わなくなったデータのメモリを自動で確保・解放
  - メモリ管理を意識しなくて良い
  - 実行時に低速
- C/C++, Rust は GC がない
  - 高速だがメモリバグが発生しやすい

### メモリ安全性

- C/C++
  - 手動メモリ管理
- Rust
  - 所有権モデル
    - 高速かつメモリ安全性を担保
    - ※コンパイルが通る必要はある

### Rust のユースケース

- Web アプリケーションのバックエンド
- WebAssembly(WASM)
  - JS から呼び出し可
- 組み込み

### 学習メリット

- 一生使える言語（システムプログラミング言語）
  - 同タイプの C/C++息が長い言語のため
  - 習得が困難で差別化
  - コンピュータシステムの仕組みに触れられる（e.g. メモリ管理）

</details>

## 環境構築 ~ 実行

<details><summary>[Rust Env & Run]</summary>

### Rust のインストール

```bash
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

### Rust 環境作成

```bash
cargo new <環境名>
```

### 拡張機能

- Rust(Extension Pack)
- CodeLLDB

### Auto Formatter

1. `Ctrl + Shift + P `
2. `settings` > Enter
3. Copy & Paste

```json
  "[rust]": {
    "editor.defaultFormatter": "rust-lang.rust-analyzer",
    "editor.formatOnSave": true
  },
```

### 実行

- build & run

```bash
cargo run
```

- build only

```bash
cargo build
```

- compile check

```bash
cargo check
```

</details>
