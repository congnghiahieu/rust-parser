#! /bin/env bash

git clone https://github.com/tokio-rs/axum.git tests/projects/axum
git clone https://github.com/actix/actix-web.git tests/projects/actix-web
git clone https://github.com/yewstack/yew.git tests/projects/yew
git clone https://github.com/SeaQL/sea-query.git tests/projects/sea-query
git clone https://github.com/tokio-rs/tokio.git tests/projects/tokio
git clone https://github.com/SeaQL/sea-orm.git tests/projects/sea-orm
git clone https://github.com/rwf2/Rocket.git tests/projects/Rocket
git clone https://github.com/serde-rs/json.git tests/projects/json
git clone https://github.com/serde-rs/serde.git tests/projects/serde
git clone https://github.com/DioxusLabs/dioxus.git tests/projects/dioxus
git clone https://github.com/ratatui/ratatui.git tests/projects/ratatui
git clone https://github.com/bevyengine/bevy.git tests/projects/bevy
git clone https://github.com/alacritty/alacritty tests/projects/alacritty
git clone https://github.com/sharkdp/bat tests/projects/bat
git clone https://github.com/BurntSushi/ripgrep tests/projects/ripgrep
git clone https://github.com/starship/starship tests/projects/starship
git clone https://github.com/sharkdp/fd tests/projects/fd
git clone https://github.com/cloudflare/pingora tests/projects/pingora
git clone https://github.com/iced-rs/iced tests/projects/iced
git clone https://github.com/zed-industries/zed tests/projects/zed

# Remove all files except .rs and .toml
find tests/projects -type f ! \( -name "*.rs" -o -name "*.toml" \) -exec rm -f {} \;
