# ethaccinfo.g.khassanov.xyz

Ethereum account info viewer.
It can show all the data from account transactions, Ether and ERC-20 tokens balances at specified date.

It utilizes [etherscan.io](https://etherscan.io/) API for data and built with NuxtJS and Rust.

## How to run the project

The project contains frontend and backend parts.

### Frontend

You can run it locally in developer mode. It requires `NodeJS` and `npm`.

```console
cd frontend
npm install
npm run dev
```

Now open http://localhost:3000 in brower to use the app.

You can replace `https://backend.ethaccinfo.g.khassanov.xyz` in `frontend/nuxt.config.ts` with your local or remote backend instance address.

### Backend

It is Rust's [hyper](https://github.com/hyperium/hyper) based tiny HTTP proxy for [etherscan.io](https://etherscan.io/) API requests authentication.

You can run it locally. It requres `cargo`.

```console
cd backend
export API_KEY=<YOUR_ETHERSCAN_API_KEY>
cargo run
```

Now it listens on port `8100`.
