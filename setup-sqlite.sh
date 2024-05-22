#!/bin/bash

echo "PLUTO__DATABASE=sqlite:database.db" > .env
echo "PLUTO__SECRET=$(openssl rand -hex 16)" >> .env
rm -rf database.db database.db-wal database.db-shm
