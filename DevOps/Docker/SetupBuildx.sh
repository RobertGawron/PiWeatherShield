#!/bin/bash

# Set up QEMU
docker run --rm --privileged multiarch/qemu-user-static --reset -p yes

# Set up Buildx
docker buildx create --use
docker buildx inspect --bootstrap

