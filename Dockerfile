FROM rust

RUN rustup component add rustfmt
RUN cargo install sea-orm-cli
