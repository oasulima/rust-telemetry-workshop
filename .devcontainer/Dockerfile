FROM ubuntu:latest
RUN apt update 
RUN apt -y upgrade 
RUN apt -y install \
    git curl build-essential pkg-config libssl-dev \
    subversion subversion-tools

USER ubuntu
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y