from sqlalchemy import Column, String, Text
from sqlalchemy.orm import declarative_base

Base = declarative_base()

class Checksum(Base):
    __tablename__ = 'checksums'

    file_name = Column(String, primary_key=True)
    checksum = Column(Text, nullable=False)
