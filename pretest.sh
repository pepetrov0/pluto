#!/bin/bash

docker compose -f local.compose.yaml up --force-recreate -dV
rm -rf database.db database.db-wal database.db-shm

