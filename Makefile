build:
	rm db1.sqlite
	rm db2.sqlite
	cargo sqlx db create --database-url sqlite:db1.sqlite
	cargo sqlx db create --database-url sqlite:db2.sqlite
	cargo sqlx prepare --database-url sqlite:db1.sqlite
	cargo sqlx prepare --database-url sqlite:db2.sqlite
	cargo run