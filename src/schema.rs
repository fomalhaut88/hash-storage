table! {
    block (id) {
        id -> Unsigned<Integer>,
        public_key -> Varchar,
        data_group -> Varchar,
        data_key -> Varchar,
        data_block -> Mediumtext,
        data_version -> Varchar,
        signature -> Varchar,
        secret -> Varchar,
    }
}
