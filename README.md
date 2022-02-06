# Overengineered Web Server

This is used as the main web server for overengineered.one.

Overengineered uses Rocket for the web backend and a yew app for the front end.

# User Guides

Currently rust is required to be installed. A docker image is coming.

## Baremetal installation and running

A simple shell script is included to build and run the server.
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
Use git to update to remote then run 

