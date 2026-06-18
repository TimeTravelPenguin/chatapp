install:
    cargo install --locked rustls-cert-gen

gen-certs:
    rustls-cert-gen --output certs/ --san localhost

server:
  rm -f server.db
  cargo run server

client:
   cargo run client
