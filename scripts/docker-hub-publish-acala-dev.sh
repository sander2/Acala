#!/usr/bin/env bash

set -e

VERSION=$(git rev-parse --short HEAD)
NODE_NAME=acala/acala-node
BUILD_ARGS="--features with-acala-runtime"

docker build -f scripts/Dockerfile . -t $NODE_NAME:$VERSION --build-arg GIT_COMMIT=${VERSION} --build-arg BUILD_ARGS="$BUILD_ARGS"
docker push $NODE_NAME:$VERSION
