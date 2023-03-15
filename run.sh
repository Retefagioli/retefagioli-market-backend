#!/usr/bin/env sh

set -e

PROJECT_NAME="retefagioli-market-backend"
VERSION="0.0.1-SNAPSHOT"

if [ -z "$(pgrep docker)" ]
then
  echo "[!] ERROR: Start docker before running this script."
  exit 1
fi

echo "[+] Building the application JAR..."
./mvnw clean package -DskipTests
echo "[+] Copying JAR to src/main/docker"
cp ./target/$PROJECT_NAME-$VERSION.jar ./src/main/docker
cd ./src/main/docker

echo "[+] Building the images"
docker compose build

echo "[+] Deleting previous docker images..."

docker image prune -f
docker compose up --detach
echo "[+] Application running on http://localhost:8080/ ..."
docker compose ps --all
docker port application-market 8080/tcp 
