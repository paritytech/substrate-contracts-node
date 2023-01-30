# substrate-contracts-node

This repository contains Substrate's [`node-template`](https://github.com/paritytech/substrate/tree/master/bin/node-template)
configured to include Substrate's [`pallet-contracts`](https://github.com/paritytech/substrate/tree/master/frame/contracts)
‒ a smart contract module.

This repository is tracking Substrate's
[`polkadot-v0.9.37`](https://github.com/paritytech/substrate/tree/polkadot-v0.9.37) branch.
When new Polkadot releases come out we may update to those.

_This repository contains a couple of modifications that make it unsuitable
for a production deployment, but a great fit for development and testing:_

* The unstable features of the [`pallet-contracts`](https://github.com/paritytech/substrate/tree/master/frame/contracts)
  are enabled by default (see the [`runtime/Cargo.toml`](https://github.com/paritytech/substrate-contracts-node/blob/main/runtime/Cargo.toml)).
* The consensus algorithm has been switched to `manual-seal` in
  [#42](https://github.com/paritytech/substrate-contracts-node/pull/42).
  Hereby blocks are authored immediately at every transaction, so there
  is none of the typical six seconds block time associated with `grandpa` or `aura`.
* _If no CLI arguments are passed the node is started in development mode
  by default._
* A custom logging filter is applied by default that hides block production noise
  and prints the contracts debug buffer to the console.
* _With each start of the node process the chain starts from genesis ‒ so no
  chain state is retained, all contracts will be lost! If you want to retain
  chain state you have to supply a `--base-path`._
* For `pallet_contracts::Config` we increased the allowed contract sizes. This
  avoids running into `CodeTooLarge` when uploading contracts during development.
  See the comment in [`runtime/src/lib.rs`](https://github.com/paritytech/substrate-contracts-node/blob/main/runtime/src/lib.rs)
  for more details.

If you are looking for a node suitable for production see these configurations:

* [Substrate Node Template](https://github.com/paritytech/substrate/tree/master/bin/node-template)
* [Substrate Cumulus Parachain Template](https://github.com/paritytech/cumulus/tree/master/parachain-template)
* [Contracts Parachain Configuration for Rococo](https://github.com/paritytech/cumulus/tree/master/parachains/runtimes/contracts/contracts-rococo)

## Installation

### Download Binary

The easiest way is to download a binary release from [our releases page](https://github.com/paritytech/substrate-contracts-node/releases)
and just execute `./substrate-contracts-node`.

### Build Locally

Follow the [official installation steps](https://docs.substrate.io/v3/getting-started/installation/)
to set up all Substrate prerequisites.

Afterwards you can install this node via

```bash
cargo install contracts-node --git https://github.com/paritytech/substrate-contracts-node.git
```

## Usage

To run a local dev node execute

```bash
substrate-contracts-node
```

A new chain in temporary directory will be created each time the command is executed. This is the
default for this node. If you want to persist chain state across runs you need to
specify a directory with `--base-path`.

See our FAQ for more details:
[How do I print something to the console from the runtime?](https://paritytech.github.io/ink-docs/faq/#how-do-i-print-something-to-the-console-from-the-runtime).

## Connect with frontend

Once the node template is running locally, you can connect to it with frontends like [Contracts UI](https://contracts-ui.substrate.io/#/?rpc=ws://127.0.0.1:9944) or [Polkadot-JS Apps](https://polkadot.js.org/apps/#/explorer?rpc=ws://localhost:9944) and interact with your chain.

## How to upgrade to new Polkadot release

- [ ] Check Substrate's [`node-template`](https://github.com/paritytech/substrate/commits/master/bin/node-template)
      for new commits between the new polkadot release branch and the one this repository is currently synced with.
      The current branch is mentioned in this readme.
- [ ] Apply each commit that happened in this `node-template` folder since the last sync.
- [ ] Check commits for [`pallet-contracts`](https://github.com/paritytech/substrate/tree/master/frame/contracts)
      since the last time someone synchronized this repository with Substrate
      in order to not miss any important changes.
- [ ] Execute `diener update -s --branch my_polkadot_release_branch`. Manually upgrade the
      `pallet-assets-chain-extension` dependency to the latest release branch and then
      `cargo update -p palle-contracts` for this repository. The specific crate which is mentioned
      here is actually not important: since Substrate uses git references for its Substrate
      dependencies it means that once one package is updated all are.
- [ ] Increment the minor version number in `node/Cargo.toml` and `runtime/Cargo.toml`.
- [ ] Execute `cargo run --release -- --tmp`. If successful, it should produce blocks
      and a new, up to date, `Cargo.lock` will be created.
- [ ] Update this readme with the hash of the Substrate `master` commit
      with which you synchronized. The hash appears two times in this
			readme.
- [ ] Create a PR with the changes, have it reviewed and merged.
- [ ] Replace `XX` in this command with your incremeted version number and execute it:
      `git checkout main && git pull && git tag v0.XX.0 && git push origin v0.XX.0`.
			This will push a new tag with the version number to this repository.
- [ ] We have set this repository up in a way that tags à la `vX.X.X` trigger
      a CI run that creates a GitHub draft release. You can observe CI runs on
      [GitLab](https://gitlab.parity.io/parity/mirrors/substrate-contracts-node/-/pipelines).
      This draft release will contain a binary for Linux and Mac and appear
      under [Releases](https://github.com/paritytech/substrate-contracts-node/releases).
      Add a description in the style of "Synchronized with Substrate commit
      [c0ee2a](https://github.com/paritytech/substrate/tree/c0ee2adaa54b22ee0df5d1592cd0430961afd95c)."
      and publish it.
