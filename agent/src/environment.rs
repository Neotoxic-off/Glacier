use std::env;

pub struct Environment {
    pub storage_directory: String,
    pub encryption_key: String,
    pub database_user: String,
    pub database_password: String,
    pub database_host: String,
    pub database_port: String,
    pub database_name: String,
    pub database_url: String,
    pub database_collection: String
}

impl Environment {
    pub fn new() -> Self {
        let storage_directory: String = env::var("STORAGE_DIRECTORY").expect("STORAGE_DIRECTORY not set");
        let encryption_key: String = env::var("ENCRYPTION_KEY").expect("ENCRYPTION_KEY not set");
        let database_user: String = env::var("DATABASE_USER").expect("DATABASE_USER not set");
        let database_password = env::var("DATABASE_PASSWORD").expect("DATABASE_PASSWORD not set");
        let database_host: String = env::var("DATABASE_HOST").expect("DATABASE_HOST not set");
        let database_port: String = env::var("DATABASE_PORT").expect("DATABASE_PORT not set");
        let database_name: String = env::var("DATABASE_NAME").expect("DATABASE_NAME not set");
        let database_collection: String = env::var("DATABASE_COLLECTION").expect("DATABASE_COLLECTION not set");
        let database_url: String = format!(
            "mongodb://{}:{}@{}:{}/{}?authSource=admin",
            database_user, database_password, database_host, database_port, database_name
        );

        Self {
            storage_directory,
            encryption_key,
            database_user,
            database_password,
            database_host,
            database_port,
            database_name,
            database_url,
            database_collection
        }
    }
}
