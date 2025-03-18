import os
import csv
import logging

from datetime import datetime

from src.handlers.file import FileHandler
from src.handlers.checksum import ChecksumHandler
from src.constants.logs import *

class Core:
    def __init__(self):
        self._load_environment()

        self.report_path = "report/{}.csv".format(
            datetime.today().strftime('%Y-%m-%d')
        )
        self.file_handler: FileHandler = FileHandler(self.storage_dir)
        self.checksum_handler: ChecksumHandler = ChecksumHandler(self.db_url)
        self.failed_files: list[str] = []
        self.success_files: list[str] = []

        logging.info(INIT_COLD_STORAGE)

    def _load_environment(self):
        self.storage_dir: str = os.getenv('STORAGE_DIR')
        self.db_user: str = os.getenv('DB_USER')
        self.db_password: str = os.getenv('DB_PASSWORD')
        self.db_host: str = os.getenv('DB_HOST')
        self.db_port: str = os.getenv('DB_PORT')
        self.db_name: str = os.getenv('DB_NAME')
        self.db_url: str = "postgresql://{}:{}@{}:{}/{}".format(
            self.db_user,
            self.db_password,
            self.db_host,
            self.db_port,
            self.db_name
        )

    def verify_files(self) -> None:
        logging.info(VERIFYING_FILES)

        files = os.listdir(self.file_handler.storage_dir)

        for file_name in files:
            checksum: str | None = self.checksum_handler.load_checksum(file_name)
            file_path: str = os.path.join(self.file_handler.storage_dir, file_name)
            current_checksum: str = self.checksum_handler.generate_checksum(file_path)

            if not checksum:
                logging.warning(NO_CHECKSUM_FOUND.format(file_name))
                self.checksum_handler.save_checksum(file_name, current_checksum)
            elif current_checksum != checksum:
                logging.error(FILE_CORRUPTED.format(file_name))
                self.failed_files.append(file_name)
            else:
                logging.info(FILE_VALIDATED.format(file_name))
                self.success_files.append(file_name)

        logging.info(GENERATING_REPORT)

        with open(self.report_path, 'w', newline='') as report_file:
            writer = csv.writer(report_file)
            writer.writerow(["File Name", "Status"])
            for file in self.success_files:
                writer.writerow([file, "Success"])
            for file in self.failed_files:
                writer.writerow([file, "Failed"])

        logging.info(f"📄 Report saved to: {self.report_path}")
