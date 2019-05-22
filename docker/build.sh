#!/bin/sh

cargo build --release

cp ../target/release/mbudget .

cp -r ../static .

cp -r ../templates .

tar -cf mbudget.tar mbudget config.toml static templates

docker build -t mbudget .

rm -rf ./static
rm -rf ./templates
rm -rf ./mbudget
rm -rf ./mbudget.tar

docker rm -f -v mbudget

docker run --name mbudget --restart unless-stopped \
	-p 8080:8080 -d \
	mbudget

sudo firewall-cmd --zone=public --add-port=8080/tcp --permanent

sudo firewall-cmd --reload

docker container ls