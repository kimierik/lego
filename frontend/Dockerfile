FROM rust:latest

RUN mkdir -p /home/app/src


#copy only needed files
COPY ./src /home/app/src
COPY ./Cargo.toml /home/app
COPY ./Cargo.lock /home/app
COPY ./rust-toolchain.toml /home/app
COPY ./index.html /home/app
COPY ./trunk.toml /home/app
COPY ./container_start_script.sh /home/app

#install the nightly tool chain and wasm target
RUN rustup toolchain install nightly
RUN rustup target add wasm32-unknown-unknown --toolchain nightly

#install trunk
RUN cargo install --locked trunk

WORKDIR /home/app/

#custom start script for the container
#this is done so we can properly set the ip
RUN chmod +x ./container_start_script.sh

EXPOSE 8080

CMD ["./container_start_script.sh"]
