

CREATE TABLE blocks (
    coinbase TEXT NOT NULL,
    flags TEXT NOT NULL,
    hash TEXT NOT NULL,
    height INTEGER NOT NULL,
    identityRoot TEXT NOT NULL,
    ipfsCid TEXT NOT NULL,
    isEmpty BOOLEAN NOT NULL,
    offlineAddress TEXT NOT NULL,
    parentHash TEXT NOT NULL,
    root TEXT NOT NULL,
    timestamp INTEGER NOT NULL,
    transactions TEXT NOT NULL,
    PRIMARY KEY (hash)
);

CREATE TABLE transactions (
    epoch INTEGER NOT NULL,
    blockHeight INTEGER NOT NULL,
    blockHash TEXT NOT NULL,
    hash_ TEXT NOT NULL,
    type_ TEXT NOT NULL,
    timestamp_ TEXT NOT NULL,
    from_ TEXT NOT NULL,
    to_ TEXT NOT NULL,
    amount TEXT NOT NULL,
    tips TEXT NOT NULL,
    maxFee TEXT NOT NULL,
    fee TEXT NOT NULL,
    size INTEGER NOT NULL,
    nonce INTEGER NOT NULL,
    PRIMARY KEY (hash_)
);