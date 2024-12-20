@echo off

del DB.db 2>nul
type nul > DB.db
cargo install sqlx-cli --features sqlite
cargo sqlx migrate run --database-url sqlite://DB.db
cargo sqlx prepare --database-url sqlite://DB.db
