install:
    cargo install --locked rustls-cert-gen

gen-certs:
    rustls-cert-gen --output certs/ --san localhost
