#!/bin/sh

echo "Installing protoc..."
sudo apt update
sudo apt install -y unzip
PROTOC_VERSION=$(curl -s "https://api.github.com/repos/protocolbuffers/protobuf/releases/latest" | grep -Po '"tag_name": "v\K[0-9.]+')
curl -Lo protoc.zip "https://github.com/protocolbuffers/protobuf/releases/latest/download/protoc-${PROTOC_VERSION}-linux-x86_64.zip"
sudo unzip -q protoc.zip bin/protoc -d /usr/local
sudo chmod a+x /usr/local/bin/protoc
rm -rf protoc.zip

echo "Installing Substrate Contracts Node..."
SUBSTRATE_VERSION=$(curl -s "https://api.github.com/repos/paritytech/substrate-contracts-node/releases/latest" | grep -Po '"tag_name": "v\K[0-9.]+')
curl -Lo substrate.tar.gz "https://github.com/paritytech/substrate-contracts-node/releases/download/v${SUBSTRATE_VERSION}/substrate-contracts-node-linux.tar.gz"
mkdir substrate-temp
tar -xzf substrate.tar.gz -C substrate-temp
sudo mv substrate-temp/artifacts/substrate-contracts-node-linux/substrate-contracts-node /usr/local/bin/
sudo chmod a+x /usr/local/bin/substrate-contracts-node
rm -rf substrate.tar.gz substrate-temp

echo "Installing cargo-contract CLI tool..."
cargo install --force --locked cargo-contract --version 2.0.0-rc

