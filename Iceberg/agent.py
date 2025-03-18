import os
import hashlib
import json
import csv
import time

class ChecksumHandler:
    def __init__(self, storage_dir):
        self.storage_dir = storage_dir

    def generate_checksum(self, file_path):
        hasher = hashlib.sha256()
        with open(file_path, 'rb') as f:
            while chunk := f.read(8192):
                hasher.update(chunk)
        return hasher.hexdigest()

    def save_checksum(self, file_name, checksum):
        checksum_file = os.path.join(self.storage_dir, f'{file_name}.checksum.json')
        with open(checksum_file, 'w') as cf:
            json.dump({file_name: checksum}, cf)

    def load_checksum(self, file_name):
        checksum_file = os.path.join(self.storage_dir, f'{file_name}.checksum.json')
        if os.path.exists(checksum_file):
            with open(checksum_file, 'r') as cf:
                return json.load(cf).get(file_name)
        return None

class FileHandler:
    def __init__(self, storage_dir):
        self.storage_dir = storage_dir
        os.makedirs(self.storage_dir, exist_ok=True)

    def save_file(self, file_name, content):
        file_path = os.path.join(self.storage_dir, file_name)
        with open(file_path, 'w') as f:
            f.write(content)
        return file_path

    def read_file(self, file_name):
        file_path = os.path.join(self.storage_dir, file_name)
        with open(file_path, 'r') as f:
            return f.read()

    def file_exists(self, file_name):
        return os.path.exists(os.path.join(self.storage_dir, file_name))

class ColdStorage:
    def __init__(self, storage_dir='/app/glacier'):
        self.storage_dir = storage_dir
        self.file_handler = FileHandler(storage_dir)
        self.checksum_handler = ChecksumHandler(storage_dir)

    def save_file(self, file_name, content):
        file_path = self.file_handler.save_file(file_name, content)
        checksum = self.checksum_handler.generate_checksum(file_path)
        self.checksum_handler.save_checksum(file_name, checksum)
        print(f"✅ File '{file_name}' saved and checksum generated.")

    def verify_files(self):
        files = os.listdir(self.storage_dir)
        for file_name in files:
            if file_name.endswith('.checksum.json'):
                continue

            checksum = self.checksum_handler.load_checksum(file_name)
            if not checksum:
                print(f"⚠️ No checksum found for '{file_name}'")
                continue

            file_path = os.path.join(self.storage_dir, file_name)
            current_checksum = self.checksum_handler.generate_checksum(file_path)

            if current_checksum == checksum:
                print(f"✅ File '{file_name}' integrity verified.")
            else:
                print(f"❗ File '{file_name}' is corrupted!")

    def process_csv(self, file_name):
        if file_name.endswith('.csv') and self.file_handler.file_exists(file_name):
            print(f"📌 Parsing CSV file: {file_name}")
            content = self.file_handler.read_file(file_name)
            reader = csv.DictReader(content.splitlines())
            for row in reader:
                print(row)

    def auto_detect_and_process(self):
        print("🔍 Watching for new files...")
        existing_files = set(os.listdir(self.storage_dir))
        while True:
            current_files = set(os.listdir(self.storage_dir))
            new_files = current_files - existing_files
            for file_name in new_files:
                if file_name.endswith('.checksum.json'):
                    continue

                print(f"🆕 New file detected: {file_name}")
                content = self.file_handler.read_file(file_name)
                self.save_file(file_name, content)
                if file_name.endswith('.csv'):
                    self.process_csv(file_name)
            existing_files = current_files
            time.sleep(5)

if __name__ == "__main__":
    storage = ColdStorage()
    
    print("📦 Cold Storage Initialized")
    storage.verify_files()
    storage.auto_detect_and_process()
