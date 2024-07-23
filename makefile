.PHONY: build start debug stop help

## 启动妈妈们
proxy:
	cargo run --release --bin solana-lite-rpc-quic-forward-proxy -- --proxy-listen-addr 127.0.0.1:11111 --identity-keypair ~/.config/solana/validator-keypair.json

## 编译
lite-rpc:
	cargo run --release --bin lite-rpc



debug-proxy:
	RUST_LOG=debug cargo run --release --bin solana-lite-rpc-quic-forward-proxy -- --proxy-listen-addr 127.0.0.1:11111 --identity-keypair ~/.config/solana/validator-keypair.json

debug-lite-rpc:
	RUST_LOG=debug cargo run --release --bin lite-rpc
