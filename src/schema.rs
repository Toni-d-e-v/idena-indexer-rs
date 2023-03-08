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
