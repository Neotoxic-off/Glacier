FROM python:3.11-alpine

WORKDIR /app

COPY cold_storage.py /app
RUN apk add --no-cache bash openssl

ENTRYPOINT ["python3", "/app/cold_storage.py"]
