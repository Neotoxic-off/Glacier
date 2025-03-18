import os
import csv
import logging

from datetime import datetime

from src.handlers.file import FileHandler
from src.handlers.signature import SignatureHandler
from src.constants.logs import *

DATE: str = datetime.today().strftime('%Y-%m-%d %H-%M-%S')

logging.basicConfig(
    level=logging.INFO,
    format="%(asctime)s [%(levelname)s] %(message)s",
    datefmt="%H:%M:%S",
    handlers=[
        logging.FileHandler(f"/app/logs/{DATE}.log"),
        logging.StreamHandler()
    ]
)

class Core:
    def __init__(self):
        self._load_environment()
        self.report_path = f"report/{DATE}.csv"
        self.file_handler: FileHandler = FileHandler(self.storage_dir)
        self.signature_handler: SignatureHandler = SignatureHandler(self.db_url)
        self.files_status: dict = {}

    def run(self):
        logging.info(INIT_COLD_STORAGE)

        self._verify_files()
        self._display_files_status()
        self._write_report()

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

    def _verify_files(self) -> None:
        logging.info(VERIFYING_FILES)

        files: list = os.listdir(self.file_handler.storage_dir)
        signature: str = None
        file_path: str = None
        current_signature: str = None

        for file_name in files:
            signature = self.signature_handler.load_signature(file_name)
            file_path = os.path.join(self.file_handler.storage_dir, file_name)
            current_signature = self.signature_handler.generate_signature(file_path)

            if not signature:
                self.signature_handler.save_signature(file_name, current_signature)
                self.files_status[file_name] = "initialized"
            elif current_signature != signature:
                self.files_status[file_name] = "corrupted"
            else:
                self.files_status[file_name] = "valid"

    def _display_files_status(self):
        file_status: tuple = None
        status: dict = {
            "initialized": (logging.warning, FILE_SAVED),
            "valid": (logging.info, FILE_VALIDATED),
            "corrupted": (logging.error, FILE_CORRUPTED)
        }

        for file in self.files_status.keys():
            file_status = status.get(self.files_status.get(file))
            file_status[0](file_status[1].format(file))

    def _write_report(self):
        logging.info(GENERATING_REPORT)

        with open(self.report_path, 'w', newline='') as report_file:
            writer = csv.writer(report_file)
            writer.writerow(["file name", "status"])
            for file in self.files_status.keys():
                writer.writerow([file, self.files_status.get(file)])

        logging.info(f"📄 Report saved to: {self.report_path}")
