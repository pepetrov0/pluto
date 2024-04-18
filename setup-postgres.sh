#!/bin/bash

echo "PLUTO__DATABASE=postgresql://local:local@localhost/local" > .env
docker compose -f local.compose.yaml up --force-recreate -dV

