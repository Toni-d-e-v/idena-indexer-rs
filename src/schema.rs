// @generated automatically by Diesel CLI.

diesel::table! {
    blocks (hash) {
        coinbase -> Text,
        flags -> Text,
        hash -> Text,
        height -> Int4,
        identityroot -> Text,
        ipfscid -> Text,
        isempty -> Bool,
        offlineaddress -> Text,
        parenthash -> Text,
        root -> Text,
        timestamp -> Int4,
        transactions -> Text,
    }
}

diesel::table! {
    transactions (hash_) {
        epoch -> Int4,
        blockheight -> Int4,
        blockhash -> Text,
        hash_ -> Text,
        type_ -> Text,
        timestamp_ -> Text,
        from_ -> Text,
        to_ -> Text,
        amount -> Text,
        tips -> Text,
        maxfee -> Text,
        fee -> Text,
        size -> Int4,
        nonce -> Int4,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    blocks,
    transactions,
);
