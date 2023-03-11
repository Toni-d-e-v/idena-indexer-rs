# idena-indexer-rs
- Donate: 0xa15de4839ed11ac66a6ff0a4e58fe90d99e67b3d (IDENA)


Idena blockchain indexer built in rust and explorer in react.js
- You can use this indexer to build your own explorer or to query data from the blockchain


## Features
- Fast and lightweight indexer built in rust
- Indexes all blocks,
- PostgreSQl + diesel ORM
### Calls
```
/block/{hash_block}
/block/height/{height}
/lastblock
/last100blocks
/tx/{hash_tx}
/account/{address}
/epoch
```
## Roadmap
- [x] API for querying data
- [x] Index all blocks
- [x] Index all transactions
- [x] UI 
- [ ] Websocket API for subscribing to new blocks
- [ ] Websocket API for subscribing to new transactions
- [ ] Websocket API for subscribing to new accounts
