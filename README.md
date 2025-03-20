# Glacier
❄️ Glacier provides a Docker-based solution for securely backing up files with built-in integrity checking and offline storage behavior.

## Environment
```INI
# CONFIGURATION
STORAGE_DIRECTORY=/glacier

# MUST BE 32 CHARACTERS LONG
ENCRYPTION_KEY=00000000000000000000000000000000

# DATABASE
DATABASE_USER=username
DATABASE_PASSWORD=password
DATABASE_HOST=storage
DATABASE_PORT=27017
DATABASE_NAME=glacier
DATABASE_COLLECTION=signature
```

## Research
- https://vivekshuk.la/tech/aes-encryption-rust
