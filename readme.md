$ docker build -t url-mapper-rs:v1 .
$ docker run --rm --net host -it url-mapper-rs:v1


docker-compose up --force-recreate --build

docker-compose up