import logging
from src.core import Core

if __name__ == "__main__":
    logging.basicConfig(
        level=logging.INFO,
        format="%(asctime)s [%(levelname)s] %(message)s",
        handlers=[
            logging.FileHandler("/app/logs/cold_storage.log"),
            logging.StreamHandler()
        ]
    )

    logging.info("📦 Cold Storage System Booting Up")
    storage = Core()

    logging.info("✅ Starting file verification...")
    storage.verify_files()

    logging.info("✅ System ready and monitoring for new files.")
