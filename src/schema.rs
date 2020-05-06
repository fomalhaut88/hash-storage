table! {
    block (id) {
        id -> Unsigned<Integer>,
        public_key -> Varchar,
        data_key -> Varchar,
        data_block -> Text,
        signature -> Varchar,
        secret -> Varchar,
    }
}
