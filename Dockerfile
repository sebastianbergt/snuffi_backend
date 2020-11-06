FROM ubuntu:20.04 
# focal-20201008, focal, latest
# WORKDIR /home/sbe/prog/rust3/snuffi/deploy
# ADD ./static ./static

ADD ./static ./static
ADD ./Rocket.toml ./Rocket.toml
ADD ./target/release/snuffi ./snuffi
EXPOSE 80
ENTRYPOINT ./snuffi