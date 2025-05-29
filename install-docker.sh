#!/bin/bash
# Docker installation script for Ubuntu

echo "Installing Docker and Docker Compose..."
sudo apt-get update
sudo apt-get install -y docker.io docker-compose

echo "Starting Docker service..."
sudo systemctl start docker
sudo systemctl enable docker

echo "Creating docker group if it doesn't exist..."
sudo groupadd -f docker

echo "Adding user to docker group..."
sudo usermod -aG docker $USER

echo "Docker version:"
docker --version

echo "Docker Compose version:"
docker-compose --version

echo ""
echo "Installation complete!"
echo "IMPORTANT: You need to log out and back in for group changes to take effect."
echo "Or run: newgrp docker"