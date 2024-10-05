# Development Environment Setup and Deployment

## Tools You Will Need:

* [Raspberry Pi Imager](https://www.raspberrypi.com/software/)
* [Docker](https://www.docker.com/products/docker-desktop/)

Not a lot of of tools are needed, that's the power of devops!

# Seting up Rasberry Pi

This section is only for setting up a fresh SD card. If your Raspberry Pi is already configured, and you can connect to it remotely via SSH, you can skip this.

* Flash Raspberry Pi OS onto the SD Card
Use Raspberry Pi Imager to install the OS (choose the Lite version for faster installation).
During setup, select Advanced Options to configure your username, password, and Wi-Fi access.
![Pi Image to be flashed](Documentation/Pictures/os.png)
* [(Optional) Backup Your SD Card](https://www.makeuseof.com/how-to-back-up-your-raspberry-pi-sd-card-on-windows/
)
* Find the Raspberry Pi’s IP Address. I've used Wireshark:
    * Power off your Raspberry Pi.
    * Start Wireshark and use the capture filter: bootp or dhcp to capture DHCP requests.
    * Power on your Raspberry Pi. The IP address will appear in the Wireshark logs.
    ![Finding IP of Rasberry Pi in local network](./Documentation/Pictures/find_pi_address.png)

4. Connect via SSH
Connect to the Raspberry Pi using SSH using credential you set-up while flashing the SD card:
```ssh <username>@<raspberry_pi_ip_address>```

### Notes:
If the Wi-Fi setup fails:
* Manually create a wpa_supplicant.conf file and place it in the root folder of the SD card.
Windows users: ensure there is no hidden .txt extension.
* To find your Wi-Fi SSID (network name) and password on a Windows PC, you can [follow these instructions](https://www.hellotech.com/guide/for/how-to-find-wifi-password-windows-10).

## Setting up your local development environment

The objective is to be able to build the Docker image of the application locally.

In the root directory (one step up to where this README.md is located) run:
* Pull Docker-in-Docker Image
  
  ```docker pull docker:dind```
  
* Build the Docker Image for Development
  
  ```docker-compose build```

* Start the docker container
  
  ```docker-compose run --service-ports dev```
  
Note: you will be logged into the container.

Now we are ready to build the images that we will send to the Raspberry Pi:

```docker buildx build --platform linux/arm/v7 -f Dockerfile.pi3b -t pi-weather .```


## Configuring Raspberry Pi and Deploying Application Docker Image

The goal is to automatically set up the Raspberry Pi and deploy an application using Ansible.

* Update inventory.ini with Raspberry Pi IP Address
* To enable secure communication with the Raspberry Pi, we’ll use SSH key-based authentication.
    * Run the following command to generate an SSH key pair (if you don’t already have one):
    ```ssh-keygen -t rsa -b 2048 -f /workspace/Devops/.ssh/id_rsa```
    * Set Correct Permissions for SSH Key
    ```
    chmod 600 /root/.ssh/id_rsa
    chmod 700 /root/.ssh/
    ssh-add /root/.ssh/id_rsa
    ```
    * Copy the public key to the Raspberry Pi to allow passwordless login.
    ```ssh-copy-id robert@<raspberry_pi_ip_address>```
    Replace <raspberry_pi_ip_address> with the actual IP address of your Raspberry Pi.
* Set Ansible Configuration
```export ANSIBLE_CONFIG=/workspace/Ansible/ansible.cfg```
* Run the Ansible Playbook
```
cd Ansible
ansible-playbook -i inventory.ini setup_pi.yml
```
