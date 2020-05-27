docker stop hash-storage-app
docker rm hash-storage-app
docker build -t hash-storage --build-arg DATABASE_URL=mysql://hash_storage:gooriruc2Al4eeSh@172.17.0.1:3306/hash_storage .
docker run -it -p 8000:8000 --name hash-storage-app --restart=always -d hash-storage
