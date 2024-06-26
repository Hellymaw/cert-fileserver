FROM rust:1.78

WORKDIR /usr/src/cert-fileserver
COPY . .

RUN cargo install --path .

CMD [ "cert-fileserver" ]