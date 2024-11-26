#!/bin/bash
SCRIPT_PATH=$(dirname $(realpath -s $0))
echo "Stopping environment..."
docker compose -p data-loader -f $SCRIPT_PATH/docker-compose.yaml stop