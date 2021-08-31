# substrate-contracts-node

This repository contains Substrate's [`node-template`](https://github.com/paritytech/substrate/tree/master/bin/node-template)
configured to include Substrate's [`pallet-contracts`](https://github.com/paritytech/substrate/tree/master/frame/contracts)
‒ a smart contract module.

This repository is tracking Substrate's `master`.

## Installation

Follow the [official installation steps](https://substrate.dev/docs/en/knowledgebase/getting-started/) 
to set up all Substrate prerequisites.

Afterwards you can install this node via

```bash
cargo install substrate-contracts-node --git https://github.com/paritytech/substrate-contracts-node.git --force
```

### Build Failure?

Since this repository is tracking Substrate's `master` it might not yet have been adjusted to support
the super-recent Substrate changes.

Thus if you get an error when building/installing, add the cargo `--locked` flag. The installation process
will then use the same versions as the `Cargo.lock` in this repository to ensure that the
most recent working version of Substrate will be used.

The latest confirmed working Substrate commit which will then be used is
[852bab073407b65b5e3e461baaa0541c4e0bc3d6](https://github.com/paritytech/substrate/tree/852bab073407b65b5e3e461baaa0541c4e0bc3d6).

## Usage

To run a local dev node execute
```
substrate-contracts-node --dev --tmp
```
The `--tmp` implies that a new chain will be created each time the command
is executed. If you want to persist chain state across runs leave it out.