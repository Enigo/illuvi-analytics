#!/bin/bash
SCRIPT_PATH=$(dirname $(realpath -s $0))
echo "Starting environment..."
docker compose -p data-loader -f $SCRIPT_PATH/docker-compose.yaml up -d --build
echo
printf "Waiting for DB"
while ! curl http://localhost:5432/ 2>&1 | grep '52' > /dev/null ; do
    printf "."
    sleep 1
done
echo
echo "Everything is up and running"