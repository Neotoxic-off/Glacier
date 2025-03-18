import os
import time
import hashlib
import logging
from sqlalchemy import create_engine
from sqlalchemy.orm import sessionmaker
from models.checksum import Base, Checksum
from src.constants.logs import *

class ChecksumHandler:
    def __init__(self, db_url: str):
        self.db_url: str = db_url
        self.engine = None
        self.Session = None
        self._connect()
        self.hasher: hashlib._Hash = hashlib.sha256()

    def _connect(self) -> None:
        logging.info(CONNECTING_DB)

        sleep_time: float = os.environ.get("LOADING_WAIT", 10.0)

        time.sleep(sleep_time)

        self.engine = create_engine(self.db_url)
        Base.metadata.create_all(self.engine)
        self.Session = sessionmaker(bind=self.engine)
    
        logging.info(CONNECTED_DB)

    def generate_checksum(self, file_path: str) -> str:
        logging.info(GENERATING_CHECKSUM.format(file_path))

        with open(file_path, 'rb') as f:
            for chunk in iter(lambda: f.read(8192), b""):
                self.hasher.update(chunk)

        return self.hasher.hexdigest()

    def save_checksum(self, file_name: str, checksum: str) -> None:
        logging.info(SAVING_CHECKSUM.format(file_name))

        with self.Session() as session:
            checksum_entry: Checksum = session.query(Checksum).filter_by(file_name=file_name).first()
            if checksum_entry:
                checksum_entry.checksum = checksum
            else:
                session.add(Checksum(file_name=file_name, checksum=checksum))
            session.commit()

    def load_checksum(self, file_name: str) -> str | None:
        logging.info(LOADING_CHECKSUM.format(file_name))

        checksum_entry: Checksum = None

        with self.Session() as session:
            checksum_entry = session.query(Checksum).filter_by(file_name=file_name).first()
            if checksum_entry:
                return checksum_entry.checksum
            else:
                return None
