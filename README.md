# PiWeatherShield

## Purpose

The PiWeatherShield is designed to create a simple home weather station using a breadboard and low-cost sensors connected to a Raspberry Pi. This weather station measures the temperature, humidity, and air pressure in the room where the Raspberry Pi is located, making it perfect for indoor monitoring, which is sufficient in many use cases.

The shield is easy to solder an can be integrated with the [Home Environment Monitor](https://github.com/RobertGawron/HomeEnvironmentMonitor), adding real-time environmental data to the broader monitoring system.

![Picture of the device](./Documentation/Pictures/device_21_09_2024.jpg)

## Hardware

This project is straightforward and includes the following I2C components:

* **BME280:** Temperature, Humidity, and Pressure sensor.
* **Si7021:** Temperature and Humidity sensor.
* **SSD1306:** 0.91-inch 128x32 OLED display.

![Circuit of the device](./Documentation/Circuit/07_09_2024.png)


Tools: KiCad.

[More info.](./Hardware/README.md)

## Installation

To simplify the setup, the project heavily relies on Docker, making deployment and management easier. This either simplifies or complicates things, as it's a relatively small project, but it's a great way to learn containerization.

### Initial Setup
* Install Raspberry Pi OS on an SD card using the [official Raspberry Pi Imager](https://www.raspberrypi.com/software/). Note: you don't need an image with desktop support.
* Configure Wi-Fi on your Raspberry Pi in order to be able to connect to it via ssh.
* Enable I2C on the Raspberry Pi to allow communication with I2C devices (TODO: clarification of the steps).
* [Install Docker on your PC](https://docs.docker.com/engine/install/).

Now we are ready to create a container where we will build containers with our app that we will deploy on Raspberry Pi, and our Raspberry Pi is ready for it.

### Setup Docker container to cross-build result Docker Raspberry Pi images

Most PCs run on x86 architecture, but Raspberry Pi uses ARM, which is why cross-compilation is needed. We will do it inside a Docker container.

In the root directory (where this README.md is located) run:
* Pull Docker-in-Docker Image
  
  ```docker pull docker:dind```
  
* Build the Docker Image for Development
  
  ```docker-compose build```

* Start the docker container
  
  ```docker-compose run --service-ports dev /bin/bash```
  
  Note: you will be logged into the container.

* Set Up Docker Buildx

  ```/usr/local/bin/SetupBuildx.sh```

Now we are ready to build the images that we will send to the Raspberry Pi. The configuration is done inside the container, meaning that we don't pollute our machine with extra installations. It is replicable (the steps can be reused by someone) and automated.

### Build the image that will be sent to Raspberry Pi

```buildx build --platform linux/arm/v7 -f Dockerfile.pi3b -t pi-weather .```

That's it for now!

