#!/bin/sh

set -e

IMAGE_NAME="exsus-labs/market"

if [ -z "$(docker images | grep nginx | grep alpine)" ]
then 
    echo '[+] Installing image nginx:alpine'
    docker image pull
fi

echo '[+] Building Dockerfile...'
docker build -t $IMAGE_NAME .
echo '[+] Running docker image...'
docker run -t $IMAGE_NAME:latest
