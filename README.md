# substrate-contracts-node

This repository contains Substrate's [`node-template`](https://github.com/paritytech/substrate/tree/master/bin/node-template)
configured to include Substrate's [`pallet-contracts`](https://github.com/paritytech/substrate/tree/master/frame/contracts)
‒ a smart contract module.

This repository is tracking Substrate's `master`.

## Installation

Follow the [official installation steps](https://docs.substrate.io/v3/getting-started/installation/)
to set up all Substrate prerequisites.

Afterwards you can install this node via

```bash
cargo install contracts-node --git https://github.com/paritytech/substrate-contracts-node.git --force --locked
```

The `--locked` flag makes the installation use the same versions
as the `Cargo.lock` in those repositories ‒ ensuring that the last
known-to-work version of the dependencies are used.

The latest confirmed working Substrate commit which will then be used is
[249dbbba6a1a277a3098c2a5b302645da16451ad](https://github.com/paritytech/substrate/tree/249dbbba6a1a277a3098c2a5b302645da16451ad).

## Usage

To run a local dev node execute
```
substrate-contracts-node --dev --tmp
```
The `--tmp` implies that a new chain will be created each time the command
is executed. If you want to persist chain state across runs leave it out.

### Show only Errors and Contract Debug Output

To have only errors and contract debug output show up on the console you can
supply `-lerror,runtime::contracts=debug` when starting the node.

Important: Debug output is only printed for RPC calls or off-chain tests ‒ not for transactions!

See our FAQ for more details:
[How do I print something to the console from the runtime?](https://paritytech.github.io/ink-docs/faq/#how-do-i-print-something-to-the-console-from-the-runtime).

## Connect with Polkadot-JS Apps Front-end

Once the node template is running locally, you can connect to it with the **Polkadot-JS Apps**
frontend to interact with your chain.
[Click here](https://polkadot.js.org/apps/#/explorer?rpc=ws://localhost:9944) to connect the frontend
to your local node.
