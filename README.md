## TODO
- [x] Genesis block + basic Block/Blockchain structs
- [x] SHA256 hashing (index, timestamp, data, previous_hash)
- [ ] Proof of Work (add nonce field, mine until hash meets difficulty target)
- [ ] Chain validation (verify each block's previous_hash matches, recompute hash to detect tampering)
- [ ] Interactive loop (accept multiple blocks per run instead of exiting after one)
- [ ] Persist chain to disk (save/load blocks as JSON so state survives between runs)
- [ ] Unit tests (hash correctness, tampering detection, genesis block integrity)
- [ ] CLI commands (e.g. add, list, validate) instead of a single linear flow
