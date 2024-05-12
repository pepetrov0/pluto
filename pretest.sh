#!/bin/bash

docker compose -f postgres.compose.yaml up --force-recreate -dV
rm -rf database.db database.db-wal database.db-shm

