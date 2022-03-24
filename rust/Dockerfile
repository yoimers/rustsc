FROM rust:latest
WORKDIR /rustscraping
RUN apt-get update -y
RUN apt-get upgrade -y
RUN apt-get install -y mariadb-client
RUN cargo init
RUN cargo install diesel_cli --no-default-features --features mysql