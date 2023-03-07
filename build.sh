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

if [ -z "$(docker images | grep 17-jre-alpine)" ]
then
  echo "[+] Installing docker image eclipse-temurin:17-jre-alpine"
  docker pull eclipse-temurin:17-jre-alpine
fi


docker compose up --force-recreate --build -d

echo "[+] Application running on http://localhost:8080/ ..."
echo "[+] Deleting previous docker images..."

docker image prune -f
