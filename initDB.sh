#!/bin/bash

rm DB.db
touch DB.db
cargo install sqlx-cli --features sqlite
sqlx migrate run --database-url sqlite://DB.db
cargo sqlx prepare --database-url sqlite://DB.db
