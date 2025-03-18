import os
import logging
from src.constants.logs import *

class FileHandler:
    def __init__(self, storage_dir: str): 
        self.storage_dir: str = storage_dir

        logging.info(STORAGE_DIR_SET.format(storage_dir))

    def _prepare_file(self, file_name: str):
        return os.path.join(self.storage_dir, file_name)

    def save_file(self, file_name: str, content: str) -> str:
        logging.info(SAVING_FILE.format(file_name))

        file_path: str = self._prepare_file(file_name)

        with open(file_path, 'w') as f:
            f.write(content)
        return file_path

    def read_file(self, file_name: str) -> str:
        logging.info(READING_FILE.format(file_name))

        content: str = None
        file_path: str = self._prepare_file(file_name)

        with open(file_path, 'r') as f:
            content = f.read()
        return content
