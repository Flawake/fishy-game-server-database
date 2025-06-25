#!/usr/bin/env bash

docker build -t ghcr.io/flawake/fishy-game-backend-rust:latest .
docker push ghcr.io/flawake/fishy-game-backend-rust:latest
