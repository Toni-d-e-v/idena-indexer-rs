# idena-indexer-rs

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
```
## Roadmap
- [x] API for querying data
- [x] Index all blocks
- [x] Index all transactions
- [x] UI 
- [ ] Websocket API for subscribing to new blocks
- [ ] Websocket API for subscribing to new transactions
- [ ] Websocket API for subscribing to new accounts
