docker stop hash-storage
docker build -t hash-storage --build-arg DATABASE_URL=mysql://hash_storage:gooriruc2Al4eeSh@172.17.0.1:3306/hash_storage .
docker run -it -p 8000:8080 --name hash-storage-app --restart=always -d hash-storage
