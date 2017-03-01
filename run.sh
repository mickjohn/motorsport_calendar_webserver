#!/bin/bash

RUST_LOG="info" # error|warning|info|debug
ROCKET_ENV="production" # dev|stage|prod
ROCKET_THREADS=10

server="./motorsport_calendar_webserver"
logdir="logs"

export RUST_LOG
export ROCKET_ENV
export ROCKET_THREADS

echo "Starting..."
  $server                  \
  > "${logdir}/stdout.log" \
  2> "${logdir}/stderr.log"\
  &

echo "Server starting in background"
