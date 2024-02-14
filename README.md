# marinade-bond-cli-rs

```shell
Usage: marinade-bond-cli-rs --url <RPC_URL> --fee-payer <KEYPAIR> --rent-payer <KEYPAIR> --identity-address <PUBKEY> --vote-address <PUBKEY>

Options:
      --url <RPC_URL>              
      --fee-payer <KEYPAIR>        
      --rent-payer <KEYPAIR>       
      --identity-address <PUBKEY>  
      --vote-address <PUBKEY>      
```

Right now, this program can only initialize (create) your bond account.

Two instructions are created in the transaction:
1. Transfer from FEE_PAYER to RENT_PAYER 2700480 lamports
2. Init bond account

The FEE_PAYER is NOT passed to the second instruction.
Therefore, it is absolutely safe for your VALIDATOR_IDENTITY to be the FEE_PAYER.

### Example

```shell
solana-keygen new --no-bip39-passphrase -o 1.json
marinade-bond-cli-rs --url https://api.mainnnet-beta.solana.com --rent-payer 1.json --fee-payer /path/to/Diman2GphWLwECE3swjrAEAJniezpYLxK1edUydiDZau.json --identity-address Diman2GphWLwECE3swjrAEAJniezpYLxK1edUydiDZau --vote-address voteRnv6PBzmiGP8NicWtQiqEJTwKKq2SxtqtdLUJjd

Transaction sent successfully. Signature: 4fWQ3fbZwM2fArhSFZZCicjZiRG6TLEhN8fv9qLjERTd3Khp86xC31HkaoYgf5KXi1eVG93KHacVcGUhPACWvS6b
```