FROM mcr.microsoft.com/devcontainers/rust:1-1-bullseye

# without this, cargo files are partially owned / only writable by root, causing
# issues in later cargo build commands etc
USER vscode

# add diesel_cli
RUN cargo install diesel_cli --no-default-features --features postgres