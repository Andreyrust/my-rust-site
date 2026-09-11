#!/usr/bin/env bash
# 1. Скачиваем и устанавливаем чистый Rust
curl --proto '=https' --tlsv1.2 -sSf https://rustup.rs | sh -s -- -y

# 2. Активируем переменные окружения Rust
. $HOME/.cargo/env

# 3. Устанавливаем правильную версию CLI Dioxus
cargo install dioxus-cli --version 0.5.6

# 4. Собираем финальный сайт на Rust
dx build --release
