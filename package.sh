#!/bin/bash
set -x
set -e

DIR="/tmp/motorsport_calendar_webserver_package/webserver"

TAR_FILE="$(pwd)/webserver.tar.bz2"

mkdir -p "$DIR"
mkdir "$DIR/logs"

cp -r "static/" "$DIR"
cp -r "templates/" "$DIR"
cp -r "config.yml" "$DIR"
cp -r "run.sh" "$DIR"
cp "target/release/motorsport_calendar_webserver" "$DIR"

cd "$DIR"
cd ".."
tar cvfj "${TAR_FILE}" *

rm -rf "$DIR"
