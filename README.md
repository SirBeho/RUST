# docker-rust

docker run --name postgres-container -e POSTGRES_USER=benjamin -e POSTGRES_PASSWORD=1192141 -e POSTGRES_DB=clientes -p 6002:5432 -d postgres


docker run --name postgres-container -e POSTGRES_USER=benjamin -e POSTGRES_PASSWORD=1192141 -e POSTGRES_DB=tienda_db -p 6001:5432 --network mi_red_docker -d postgres
