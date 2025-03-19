# Glacier
❄️ Glacier provides a Docker-based solution for securely backing up files with built-in integrity checking and offline storage behavior.

## Environment
```INI
# CONFIGURATION
STORAGE_DIRECTORY=/glacier
ENCRYPTION_KEY=this_is_the_key

# DATABASE
DATABASE_USER=username
DATABASE_PASSWORD=password
DATABASE_HOST=storage
DATABASE_PORT=27017
DATABASE_NAME=glacier
DATABASE_COLLECTION=signature
```

## Todo
- Move from sha256 to tree hash
- Validate file nodes from tree hash
