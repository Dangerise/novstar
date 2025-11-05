rm -f data.db
rm -rf target/dx
sqlite3 data.db ".read ./data.sql"
cargo r -r -F native --bin save_db 
dx bundle -p northstar_gui -r 
