#!/bin/bash

arch=$(uname -m)

[ "$arch" = "x86_64" ] && wget https://github.com/tailwindlabs/tailwindcss/releases/download/v3.4.1/tailwindcss-linux-x64 -O tailwindcss;
[ "$arch" = "aarch64" ] && wget https://github.com/tailwindlabs/tailwindcss/releases/download/v3.4.1/tailwindcss-linux-arm64 -O tailwindcss;

chmod +x tailwindcss