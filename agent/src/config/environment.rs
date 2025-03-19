use std::env;

pub struct Environment {
    pub storage_directory: String,
    pub encryption_key: String,
    pub database_url: String,
    pub database_name: String,
    pub database_collection: String,
}

impl Environment {
    pub fn new() -> Result<Self, env::VarError> {
        let storage_directory = env::var("STORAGE_DIRECTORY")?;
        let encryption_key = env::var("ENCRYPTION_KEY")?;
        let database_user = env::var("DATABASE_USER")?;
        let database_password = env::var("DATABASE_PASSWORD")?;
        let database_host = env::var("DATABASE_HOST")?;
        let database_port = env::var("DATABASE_PORT")?;
        let database_name = env::var("DATABASE_NAME")?;
        let database_collection = env::var("DATABASE_COLLECTION")?;
        
        let database_url = format!(
            "mongodb://{}:{}@{}:{}/{}?authSource=admin",
            database_user, database_password, database_host, database_port, database_name
        );

        Ok(Self {
            storage_directory,
            encryption_key,
            database_url,
            database_name,
            database_collection,
        })
    }
}
