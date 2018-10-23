FROM ubuntu:18.04

RUN mkdir -p "/var/motorsport_calendar_webserver_www/static" \
  && mkdir -p "/var/motorsport_calendar_webserver_www/templates" \
  && apt-get update \
  && apt-get install libssl1.0.0 -y

WORKDIR /etc/motorsport_calendar_webserver
COPY docker_config.yml .

WORKDIR /usr/bin
COPY target/release/motorsport_calendar_webserver .

WORKDIR /var/motorsport_calendar_webserver_www/
COPY static static
COPY templates templates

WORKDIR /usr/bin
COPY target/release/motorsport_calendar_webserver .

EXPOSE 8080
ENV ROCKET_ADDRESS="0.0.0.0"
ENV ROCKET_PORT="8080"
CMD ["./motorsport_calendar_webserver", "-c", "/etc/motorsport_calendar_webserver/docker_config.yml"]
