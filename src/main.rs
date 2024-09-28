#[tokio::main]
async fn main() {
    libsql::Builder::new_local("test.db")
        .build()
        .await
        .inspect_err(|e| eprintln!("{:?}", e))
        .unwrap()
        .connect()
        .inspect_err(|e| eprintln!("{:?}", e))
        .unwrap();

    geni::migrate_database(
        "sqlite://./test.db".to_string(), // Database URL
        None,                             // Database Token
        "migrations".to_string(),         // Migration Table
        "./migrations".to_string(),       // Migration Folder
        "schema.sql".to_string(),         // Schema File
        Some(30),                         // Wait timeout for the database to be ready
        false,                            // Dump Schema
    )
    .await
    .unwrap();
}
