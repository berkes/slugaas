DIR := ${CURDIR}

include .env

all:
	cargo build --release

test:
	rm -rf ${SSS_DB_LOCATION}
	cargo test --all
	hurl --test tests/*hurl
