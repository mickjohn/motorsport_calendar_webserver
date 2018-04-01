#!/bin/bash
set -e

MAJOR_VERSION="1"
MINOR_VERSION="$(($(cat minor_version.txt) + 1))"
echo $MINOR_VERSION > minor_version.txt
VERSION="${MAJOR_VERSION}.${MINOR_VERSION}"
NAME="mscweb"
FULL_NAME="motorsport_calendar_webserver"

WORKING_DIR="/home/mick/Programs/rust/motorsport_calendar_webserver/PACKAGE"

if [ -d "$WORKING_DIR" ];then
  echo "$WORKING_DIR already exists."
  echo "Delete it? y/n"
  read DELETE_DIR
  if [ "${DELETE_DIR}x" = "yx" ];then
    echo "Deleting $WORKING_DIR"
    rm -rf "${WORKING_DIR}"
  else
    exit 1
  fi
fi

echo "Creating files..."

mkdir $WORKING_DIR
cd $WORKING_DIR

PACKAGE="${NAME}_${VERSION}"
mkdir "${PACKAGE}"
cd "$PACKAGE"

WEB_DIR="var/${FULL_NAME}_www/static"
mkdir -p "${WEB_DIR}"
cp -r "../../static/"* "${WEB_DIR}"

TEMPLATE_DIR="var/${FULL_NAME}_www/templates"
mkdir -p "${TEMPLATE_DIR}"
cp -r "../../templates/"* "${TEMPLATE_DIR}"

CONF_DIR="etc/${FULL_NAME}"
mkdir -p "${CONF_DIR}"
echo "---
api_url: http://localhost:8000/events
static_content_dir: /${WEB_DIR}
template_directory: /${TEMPLATE_DIR}" > "${CONF_DIR}/config.yml"

BIN_DIR="usr/local/bin/"
mkdir -p "${BIN_DIR}"
cp "../../target/release/motorsport_calendar_webserver" "${BIN_DIR}"

SERVICE_DIR="lib/systemd/system/"
mkdir -p "${SERVICE_DIR}"
echo "[Unit]
Description=${FULL_NAME}
[Service]
Environment=ROCKET_PORT=8080
Environment=ROCKET_WORKERS=10
Type=simple
ExecStart=/${BIN_DIR}/${FULL_NAME} -c /${CONF_DIR}/config.yml
[Install]
WantedBy=multi-user.target" > "${SERVICE_DIR}${FULL_NAME}d.service"

CONTROL_DIR="DEBIAN/"
mkdir -p "${CONTROL_DIR}"
chmod -R 0755 "${CONTROL_DIR}"
echo "Package: ${NAME}
Version: ${VERSION}
Section: base
Priority: optional
Architecture: amd64
Depends: 
Maintainer: Your Name <you@email.com>
Description: Hello World
 When you need some sunshine, just run this
 small program!" > "${CONTROL_DIR}control"

echo "Building package..."
cd ..
dpkg-deb --build "${PACKAGE}"

echo "Moving package up a directory..."
mv "${PACKAGE}.deb" ..

echo "Deleting ${WORKING_DIR}"
cd ..
rm -rf "${WORKING_DIR}"
echo "Finished :D"
