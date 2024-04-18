#!/bin/bash

echo "PLUTO__DATABASE=sqlite:database.db" > .env
rm -rf database.db database.db-wal database.db-shm
