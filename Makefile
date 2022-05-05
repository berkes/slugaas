DIR := ${CURDIR}

all:
	cargo build --release

test:
	rm -rf /tmp/slugaas_db
	cargo test --all
	hurl --test tests/*hurl
