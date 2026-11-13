.PHONY: setup test qemu-smoke check fmt clippy agent-status

setup:
	rustup target add x86_64-unknown-linux-gnu aarch64-unknown-linux-gnu || true
	cargo fetch

test:
	cargo test --workspace

check:
	cargo check --workspace

fmt:
	cargo fmt --all

clippy:
	cargo clippy --workspace -- -D warnings

qemu-smoke:
	@echo "QEMU smoke test — running asari-boot stub"
	cargo run -p asari-boot -- --platform qemu --smoke || true

agent-status:
	@echo "=== Asari-OS Feature Status ==="
	@cat agents/feature_list.json
