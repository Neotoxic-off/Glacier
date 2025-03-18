import os
import hashlib
import json

STORAGE_DIR = '/app/data'
CHECKSUM_FILE = '/app/data/checksums.json'

os.makedirs(STORAGE_DIR, exist_ok=True)

def generate_checksum(file_path):
    hasher = hashlib.sha256()
    with open(file_path, 'rb') as f:
        while chunk := f.read(8192):
            hasher.update(chunk)
    return hasher.hexdigest()

def save_file(file_name, content):
    file_path = os.path.join(STORAGE_DIR, file_name)
    with open(file_path, 'w') as f:
        f.write(content)

    checksum = generate_checksum(file_path)

    if os.path.exists(CHECKSUM_FILE):
        with open(CHECKSUM_FILE, 'r') as cf:
            checksums = json.load(cf)
    else:
        checksums = {}

    checksums[file_name] = checksum
    with open(CHECKSUM_FILE, 'w') as cf:
        json.dump(checksums, cf)
    print(f"✅ File '{file_name}' saved and checksum generated.")

def verify_files():
    if not os.path.exists(CHECKSUM_FILE):
        print("⚠️ No checksum file found! Verification skipped.")
        return
    
    with open(CHECKSUM_FILE, 'r') as cf:
        checksums = json.load(cf)
    
    for file_name, original_checksum in checksums.items():
        file_path = os.path.join(STORAGE_DIR, file_name)
        if not os.path.exists(file_path):
            print(f"❌ File '{file_name}' is missing!")
            continue
        
        current_checksum = generate_checksum(file_path)
        if current_checksum == original_checksum:
            print(f"✅ File '{file_name}' integrity verified.")
        else:
            print(f"❗ File '{file_name}' is corrupted!")

if __name__ == "__main__":
    print("📦 Cold Storage")
    print("1. Saving test files...")
    save_file('data1.json', '{"name": "cold_storage"}')
    save_file('data2.csv', 'id,value\n1,test')
    
    print("\n2. Verifying files...")
    verify_files()
