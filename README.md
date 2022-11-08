On-chain hackathon dApp on Rust / NEAR Blockchain

Example app built during the [NEAR Web4 Online Hackathon](https://web4-hackathon.near.page/)
Both the web3 logic and the web2 user interface stored and executed on a blockchain using (Web4 Protocol)[https://web4.near.page/]. 

Application features:
- On-chain storage of all hackathon submissions
- Preserved prize pool deposited by the hackathon owner and stored in the contract
- Submissions deadline stored in the contract
- Hackathon participant has an option to update the application before the deadline
- The owner of the hackathon marks the participant as winner and automatically transfers his part of the prize pool

All updates of the contract (new applications, rewards, etc) state became visible on a UI. 

Video tutorial: https://youtu.be/tcBjZFWdMxI

## Quick Start
To run this project:

1. Run `./build.sh` to compile wasm binary fine
2. Register testnet account using https://wallet.testnet.near.org/create (for example, `contract.testnet`)
3. Store your contract_id in the environment variable `export CONTRACT_ID=contract.testnet`
4. Deploy contract to NEAR Blockchain: `near deploy $CONTRACT_ID --wasmFile=./res/web4.wasm -f`
5. Check your contract online on https://$CONTRACT_ID.testnet.page (for example contract.testnet.page)
6. Register account on a mainnet and deploy your contract there, yor app will be available on https://$CONTRACT_ID.near.page


[![Open in Gitpod](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/#https://github.com/zavodil/web4-hackathon-example)

## Prerequisites
Ensure `near-cli` is installed by running:

```
near --version
```

If needed, install `near-cli`:

```
npm install near-cli -g
```

Ensure `Rust` is installed by running:

```
rustc --version
```

If needed, install `Rust`:

```
curl https://sh.rustup.rs -sSf | sh
```

Run the compiler

```
./build.sh
```
