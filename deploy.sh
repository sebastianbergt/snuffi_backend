docker kill snuffi_container
docker rm snuffi_container

docker build --tag snuffi:0.1 .
docker run \
    -v ~/measurement:/measurement \
    -v ~/private:/private \
    --publish 80:80 --name snuffi_container snuffi:0.1 

# --detach