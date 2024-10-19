# Dropnote Chatty
Dropnote Chatty is a reference implementation of a Rust CosmWasm smart contract which integrates with the Dropnote protocol.

In brief, by emitting events of the correct format, your smart contract can participate in the Dropnote protocol, designating messages for specific users or broadcasting announcements to all subscribers.

*Note:* This currently still requires some additional features to be implemented in the [Dropnote Indexer library & reference implementation](https://github.com/kiruse/dropnote-indexer).

By itself, Chatty v1 simply allows users to broadcast messages with the `sender` field set to their own address, acting as a proxy.

Chatty v1.1 adds support for storing messages within the contract itself, enabling persistent on-chain storage of messages. However, this requires additional features to be implemented in the Dropnote Indexer. Specifically, the Indexer currently only checks two locations for Dropnote events: The transaction memo, and the transaction event logs. The plan is to, eventually, introduce the concept of "storage references", which would instruct indexers to look up the actual contents of a message elsewhere, e.g. on IPFS, on Arweave, or by a smart query on a smart contract.
