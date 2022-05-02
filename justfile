localnet-validator:
	solana-test-validator -r --ledger localnet/ledger

localnet-init:
	solana airdrop 1 localnet/admin.json -u localhost
	solana airdrop 1 localnet/user.json -u localhost

devnet-airdrop:
	solana airdrop 2 localnet/admin.json -u devnet
	solana airdrop 1 localnet/user.json -u devnet

test:
	cd program; cargo test
	cd program; cargo test-bpf

localnet-deploy: test
	cd program; cargo build-bpf
	solana program deploy ./target/deploy/counter.so -u localhost --program-id localnet/program.json

devnet-deploy: test
	cd program; cargo build-bpf
	solana program deploy ./target/deploy/counter.so -u testnet --program-id localnet/program.json --keypair localnet/admin.json --upgrade-authority localnet/admin.json

client:
    cd client; npm install
    cd client; npm run run