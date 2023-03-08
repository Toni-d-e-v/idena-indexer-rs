

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
