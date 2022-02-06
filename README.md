# Overengineered Web Server

This is used as the main web server for overengineered.one.

Overengineered uses Rocket for the web backend and a yew app for the front end.

# User Guides

Currently rust is required to be installed. A docker image is coming.

## Instalation Requirements

* Git
* [Rust](https://www.rust-lang.org/tools/install) 
* [npm]()
* [Web Pack]()
* [Brew](https://brew.sh/)
* [Bulma CSS](https://bulma.io/documentation/customize/with-sass-cli/)
* [Sass CLI](https://sass-lang.com/install)

## Baremetal installation and running

A simple shell script is included to build both the client and the server then run the server.
Linux is currently the only supported platform.

First give execute permissions.

```
chmod +x ./serve.sh
```

Then you can build and launch.

```
./serve.sh
```

## Configuration

Environment variables can be used to set the default address and port.

```
ROCKET_ADDRESS=0.0.0.0 ROCKET_PORT=443 ./serve.sh
```

## Deploying

There is no container orchestration set up. Currently you are required to ssh into the cloud vm.
Use git to update to remote then run the serve shell script to compile. Once completed create a 
service to keep the webserver alive.

