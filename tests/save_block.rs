#[cfg(test)]
mod save_block {
    use namada_prototype::utils::load_checksums;
    use namada_prototype::Database;
    use namada_prototype::Settings;
    use std::fs;
    use tendermint::block::Block;

    #[tokio::test]
    async fn a_save_block() {
        let config = Settings::new().unwrap();
        let config = config.database_config();

        let checksums_map = load_checksums().unwrap();

        let data = fs::read_to_string("./tests/blocks_vector.json").unwrap();
        let blocks: Vec<Block> = serde_json::from_str(&data).unwrap();

        let db = Database::new(config).await.unwrap();
        db.create_tables().await.unwrap();

        for block in blocks {
            db.save_block(&block, &checksums_map).await.unwrap();
        }
    }

    #[tokio::test]
    async fn b_create_indexes() {
        let config = Settings::new().unwrap();
        let config = config.database_config();

        let db = Database::new(config).await.unwrap();

        db.create_indexes().await.unwrap();
    }
}
