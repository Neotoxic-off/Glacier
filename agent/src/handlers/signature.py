import hashlib
import logging
from sqlalchemy import create_engine
from sqlalchemy.orm import sessionmaker
from models.signature import Base, Signature
from src.constants.logs import *

class SignatureHandler:
    def __init__(self, db_url: str):
        self.db_url: str = db_url
        self.engine = None
        self.Session = None
        self._connect()
        self.hasher: hashlib._Hash = hashlib.sha256()

    def _connect(self) -> None:
        logging.info(CONNECTING_DB)

        self.engine = create_engine(self.db_url)
        Base.metadata.create_all(self.engine)
        self.Session = sessionmaker(bind=self.engine)
    
        logging.info(CONNECTED_DB)

    def generate_signature(self, file_path: str) -> str:
        logging.info(GENERATING_SIGNATURE.format(file_path))

        with open(file_path, 'rb') as file:
            file_data = file.read()

        return hashlib.sha256(file_data).hexdigest()

    def save_signature(self, file_name: str, signature: str) -> None:
        logging.info(SAVING_SIGNATURE.format(file_name))

        with self.Session() as session:
            signature_entry: Signature = session.query(Signature).filter_by(file_name=file_name).first()
            if signature_entry:
                signature_entry.signature = signature
            else:
                session.add(Signature(file_name=file_name, signature=signature))
            session.commit()

    def load_signature(self, file_name: str) -> str | None:
        logging.info(LOADING_SIGNATURE.format(file_name))

        signature_entry: Signature = None

        with self.Session() as session:
            signature_entry = session.query(Signature).filter_by(file_name=file_name).first()
            if signature_entry:
                return signature_entry.signature
            else:
                return None
