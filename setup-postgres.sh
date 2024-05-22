#!/bin/bash

echo "PLUTO__DATABASE=postgresql://local:local@localhost/local" > .env
echo "PLUTO__SECRET=$(openssl rand -hex 16)" >> .env
docker compose -f postgres.compose.yaml up --force-recreate -dV

