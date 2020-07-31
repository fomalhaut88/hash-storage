docker stop hash-storage-app
docker rm hash-storage-app
docker build -t hash-storage --build-arg DATABASE_URL=/usr/src/app/tmp/sqlite.db .
docker run -it -p 8000:8000 --name hash-storage-app --volume /path/to/database/folder:/usr/src/app/tmp --restart=always -d hash-storage
