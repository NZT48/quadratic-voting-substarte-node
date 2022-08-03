# Quadratic Voting Pallet

The Quadratic voting pallet is simple implementation of quadratic voting system. 
Users are able to propose something by providing hash or short string, 
then other users can vote with different amount of votes to express their opinion about proposal. 
Proposal voting results will be always stored and accessible on the blockchain.

## Overview

Quadratic voting is a collective decision-making procedure which involves individuals 
allocating votes to express the degree of their preferences, rather than just the direction of their preferences. 
By doing so, quadratic voting seeks to address issues of voting paradox and majority rule.

The quadratic cost function has the unique property that people purchase votes directly proportionally
to the strength of their preferences

## Interface

### Dispatchable Functions

General spending/proposal protocol:
- `propose` - Create a proposal for voting using quadratic voting system.
- `vote_aye` - Vote for proposal at proposal index with one or more votes.
- `vote_nay` - Vote against proposal at proposal index with one or more votes.
- `unreserve` - Unreserve tokens after voting period is ended.

## Usage

Start node:
```sh
./target/release/node-template --dev
```
* Access the polkadot.js interface at: https://polkadot.js.org/apps/?rpc=ws://127.0.0.1:9944#/explorer
* Create identity for different users
* Create proposal with first user
* Vote aye with first and second user
* Vote nay with third user
* Wait for voting period to finish and then unlock you tokens

Run tests:
```sh
cargo test -p pallet-quadratic-voting
```

Run benchmarks:
```sh
cargo test -p pallet-quadratic-voting --features runtime-benchmarks
```

## Todo

[] Fix and implement benchmarking for missing functions
[] Implement fees for proposal creation to prevent spamming
[] Expand to have a more complex proposal system where users can vote on multiple things at once, and have to consider how they want to distribute their votes across them.