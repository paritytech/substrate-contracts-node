# substrate-contracts-node

[![Open in GitHub Codespaces](https://github.com/codespaces/badge.svg)](https://codespaces.new/paritytech/substrate-contracts-node)

This repository contains Substrate's [`node-template`](https://github.com/paritytech/substrate/tree/master/bin/node-template)
configured to include Substrate's [`pallet-contracts`](https://github.com/paritytech/substrate/tree/master/frame/contracts)
‒ a smart contract module.

_This repository contains a couple of modifications that make it unsuitable
for a production deployment, but a great fit for development and testing:_

* The unstable features of the [`pallet-contracts`](https://github.com/paritytech/substrate/tree/master/frame/contracts)
  are enabled by default (see the [`runtime/Cargo.toml`](https://github.com/paritytech/substrate-contracts-node/blob/main/runtime/Cargo.toml)).
* The consensus algorithm has been switched to `manual-seal` in
  [#42](https://github.com/paritytech/substrate-contracts-node/pull/42).
  Hereby blocks are authored immediately at every transaction, so there
  is none of the typical six seconds block time associated with `grandpa` or `aura`.
  * By default, either manual or instant seal does not result in block finalization unless the `engine_finalizeBlock` 
    RPC is executed. However, it is possible to configure the finalization of sealed blocks to occur after a certain 
    amount of time by setting the `--finalize-delay-sec` option to a specific value, which specifies the number of seconds 
    to delay before finalizing the blocks. The default value is 1 second.
    ```shell
    ./target/release/substrate-contracts-node --finalize-delay-sec 5
    ```
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

Follow the [official installation steps](https://docs.substrate.io/install/) to set up all Substrate prerequisites.

Afterwards you can install this node via

```bash
cargo install contracts-node
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

We can have two types of releases:

* Internal release: This type of release does not involve releasing the crates on crates.io. It involves using Git
  references in the Cargo.toml dependencies. We utilize this type of release for faster iteration when we don't want
  to wait for the substrate crates to be released.

* Crate release: This is the preferable type of release, which involves specifying crate versions in the Cargo.toml
  dependencies and releasing the crates on crates.io..

- [ ] Check Substrate's [`solochain-template`](https://github.com/paritytech/polkadot-sdk/tree/master/templates/solochain),
      for new commits between the new polkadot release branch and the one this repository is currently synced with.
      The current branch is mentioned in the last release.
- [ ] Apply each commit that happened in this `solochain-template` folder since the last sync.
- [ ] Check [`parachain-template`](https://github.com/paritytech/polkadot-sdk/tree/master/templates/parachain)
      and apply each commit that has occurred in its folder since the last sync.
- [ ] Check commits for [`pallet-contracts`](https://github.com/paritytech/polkadot-sdk/tree/master/substrate/frame/contracts)
      since the last time someone synchronized this repository with Substrate
      in order to not miss any important changes.
- [ ] (Crate release only) Execute `psvm -p ./Cargo.toml -v X.X.X`, to update the dependencies to the required versions.
      Replace `X.X.X` with the requested Polkadot release version.
- [ ] (Internal release only)  Manually update the dependencies in Cargo.toml to the required Git SHA versions.
- [ ] Increment the minor version number in `Cargo.toml` and `node/Cargo.toml`.
- [ ] Execute `cargo run --release`. If successful, it should produce blocks
      and a new, up to date, `Cargo.lock` will be created.
- [ ] Create a PR with the changes, have it reviewed.
- [ ] (Crate release only) Upload crates to `crates.io` using the commands below, replacing `XX` with your incremented
      version number:
      `cargo release 0.XX.0 -v --no-tag --no-push -p contracts-node-runtime -p contracts-parachain-runtime --execute`
      `cargo release 0.XX.0 -v --no-tag --no-push -p contracts-node --execute`
      Note: Before uploading, perform a dry run to ensure that it will be successful.
- [ ] Merge the release PR branch.
- [ ] Set the tag and run the following commands to push the tag. The tag must contain a message, otherwise the github action won't be able to create a release:
  
```bash
TAG="v0.XX.0"
git checkout main
git pull
git tag -a ${TAG} -m "${TAG}"
git push origin ${TAG}
```


- [ ] After tag is pushed CI creates a GitHub draft release.
      This draft release will contain a binary for Linux and Mac and appear
      under [Releases](https://github.com/paritytech/substrate-contracts-node/releases).
      Add a description in the style of "Synchronized with [`polkadot-v1.8.0`](https://github.com/paritytech/polkadot-sdk/tree/release-polkadot-v1.8.0) branch."
      and publish it.
