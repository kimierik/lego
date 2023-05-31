FROM rust:latest

RUN mkdir -p /home/app/src



#copy only needed files
COPY ./src /home/app/src
COPY ./Cargo.toml /home/app
COPY ./Cargo.lock /home/app
COPY ./rust-toolchain.toml /home/app
COPY ./index.html /home/app
COPY ./trunk.toml /home/app

#install the nightly tool chain and wasm target
RUN rustup toolchain install nightly
RUN rustup target add wasm32-unknown-unknown --toolchain nightly


#install trunk
RUN cargo install --locked trunk

WORKDIR /home/app/

EXPOSE 8080

#currently the ip is set manually in the trunk . toml file. and will not work if you have multiple containers running
CMD ["trunk", "--config", "trunk.toml", "serve"]
