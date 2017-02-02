#!/bin/bash

RUST_LOG="debug" # error|warning|info|debug
ROCKET_ENV="production" # dev|stage|prod

server="./motorsport_calendar_webserver"
logdir="logs"

export RUST_LOG
export ROCKET_ENV

echo "Starting..."
  $server                \
  > "${logdir}/stdout.log" \
  2> "${logdir}/stderr.log"\
  &

echo "Server starting in background"
