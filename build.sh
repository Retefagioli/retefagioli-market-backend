#!/usr/bin/env sh

set -e

PROJECT_NAME="retefagioli-market-backend"
VERSION="0.0.1-SNAPSHOT"

echo "[+] Building the application JAR..."
./mvnw clean package -DskipTests
echo "[+] Copying JAR to src/main/docker"
cp ./target/$PROJECT_NAME-$VERSION.jar ./src/main/docker
cd ./src/main/docker

