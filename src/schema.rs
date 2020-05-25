table! {
    block (id) {
        id -> Unsigned<Integer>,
        public_key -> Varchar,
        data_key -> Varchar,
        data_block -> Mediumtext,
        signature -> Varchar,
        secret -> Varchar,
    }
}
