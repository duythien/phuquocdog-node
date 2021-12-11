#!/usr/bin/env bash
set -e
DOCKER_SCAN_SUGGEST=false docker build -f ./docker/Dockerfile -t phuquocdog/node:latest .