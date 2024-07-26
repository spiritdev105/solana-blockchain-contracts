# dApp on Solana

Run validator

```bash
solana-test-validator
```

Build the contract

```bash
anchor build
```

Deploy the contract
```bash
anchor deploy
```
Note: If the deployment fails, topup the size using ``solana program extend``


Generate key-pair string

```bash
cargo run -p keygen
```


Get config

```bash
solana config get
```


Configure localhost
```bash
solana config set --url localhost
```

Key Generation
```bash
solana-keygen new
```


Airdrop SOL

```bash
solana airdrop 2
```

Balance check

```bash
solana balance
```

Address

```bash
solana address
```

Deploy Rust Program

```bash
solana program deploy ./target/deploy/hello_world.so
```

Show programs

```bash
solana program show --programs
```


Show account info

```bash
solana account account-id
```

Anchor test

```bash
anchor test --skip-local-validator
```

Anchor key list

```bash
anchor keys list
```

More bytes to the contract using ``solana program extend``

```bash
solana program extend AcfG5w5GjU95Ho6imbra9xBW5gvM5ujNdKN7PjNvtkfe 30000 --url "http://localhost:8899" -k ~/.config/solana/id.json
```
