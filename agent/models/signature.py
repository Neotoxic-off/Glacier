from sqlalchemy import Column, String, Text
from sqlalchemy.orm import declarative_base

Base = declarative_base()

class Signature(Base):
    __tablename__ = 'signatures'

    file_name = Column(String, primary_key=True)
    signature = Column(Text, nullable=False)
