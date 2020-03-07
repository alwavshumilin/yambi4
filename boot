#!/bin/sh
CONFIG=guniconf.py
if [ $# -eq 0 ]
  then
    echo "No arguments supplied. Using default config"
else
  CONFIG=$1
fi

gunicorn engine.app:boot -c "$CONFIG"
