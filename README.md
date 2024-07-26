# dApp on Solana

pubkey: CHDriqdpVGcYGR9hKbxGtBUiUh7umMEa4gViotcYgsnN

Run validator

```bash
solana-test-validator
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

More bytes to the contract

```bash
solana program extend FcVz5svHC4h2JZFuGNnwCk5gfWz8bnEjoMhYUaTqm9d3 30000 --url "http://localhost:8899" -k ~/.config/solana/id.json
```
