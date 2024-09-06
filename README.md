# PiWeatherShield

## Purpose

The PiWeatherShield is designed to create a simple and affordable weather station using a breadboard and low-cost sensors connected to a Raspberry Pi. This weather station measures the temperature, humidity, and air pressure in the room where the Raspberry Pi is located, making it perfect for indoor monitoring, which is sufficient in many use cases.

The shield is easy to solder an can be integrated with the [Home Environment Monitor](https://github.com/RobertGawron/HomeEnvironmentMonitor), adding real-time environmental data to the broader monitoring system.

## Installation

To simplify setup, the project uses Docker, making it easy to deploy and manage. Docker ensures that all dependencies and configurations are handled automatically, allowing you to focus on getting the weather data up and running.

<!--
Steps:
Clone the Repository:

bash
Copier le code
git clone https://github.com/YourUsername/PiWeatherShield.git
cd PiWeatherShield
Build and Run with Docker: Ensure that Docker is installed on your Raspberry Pi. If not, you can install it with the following command:

bash
Copier le code
curl -fsSL https://get.docker.com -o get-docker.sh
sudo sh get-docker.sh
Then, use Docker to build and run the project:

bash
Copier le code
sudo docker-compose up -d
Access the Data: Once the Docker container is running, it will begin collecting real-time temperature, humidity, and air pressure data from the connected sensors.
-->
