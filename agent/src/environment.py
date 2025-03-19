import os

class Environment:
    def __init__(self):
        self.storage_directory = os.environ.get("STORAGE_DIRECTORY")
        self.encryption_key = os.environ.get("ENCRYPTION_KEY")
        self.database_user = os.environ.get("DATABASE_USER")
        self.database_password = os.environ.get("DATABASE_PASSWORD")
        self.database_host = os.environ.get("DATABASE_HOST")
        self.database_port = os.environ.get("DATABASE_PORT")
        self.database_name = os.environ.get("DATABASE_NAME")
        self.database_url: str = "postgresql://{}:{}@{}:{}/{}".format(
            self.database_user,
            self.database_password,
            self.database_host,
            self.database_port,
            self.database_name
        )
