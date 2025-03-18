import os
import logging
from src.constants.logs import *

class FileHandler:
    def __init__(self, storage_dir: str):
        self.storage_dir: str = storage_dir
        os.makedirs(self.storage_dir, exist_ok=True)
        logging.info(STORAGE_DIR_SET.format(storage_dir))

    def save_file(self, file_name: str, content: str) -> str:
        file_path: str = os.path.join(self.storage_dir, file_name)
        logging.info(SAVING_FILE.format(file_name))
        with open(file_path, 'w') as f:
            f.write(content)
        return file_path

    def read_file(self, file_name: str) -> str:
        file_path: str = os.path.join(self.storage_dir, file_name)
        logging.info(READING_FILE.format(file_name))
        with open(file_path, 'r') as f:
            return f.read()

    def file_exists(self, file_name: str) -> bool:
        exists: bool = os.path.exists(os.path.join(self.storage_dir, file_name))
        logging.info(FILE_EXISTS.format(file_name, exists))
        return exists
