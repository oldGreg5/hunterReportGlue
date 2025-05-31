#!/bin/zsh

# Hunter Report Generator Docker Hub Publishing Script
# This script builds and pushes the Docker image to Docker Hub

# Colors for terminal output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration - CHANGE THESE VALUES
DOCKER_USERNAME="oldgreg5"  # Replace with your Docker Hub username
IMAGE_NAME="hunter-report-glue"
TAG="latest"
FULL_IMAGE_NAME="${DOCKER_USERNAME}/${IMAGE_NAME}:${TAG}"

echo "${BLUE}=== Hunter Report Generator - Docker Hub Publishing ===${NC}"
echo "${BLUE}=== Using Docker username: ${DOCKER_USERNAME} ===${NC}"

# Check if Docker is installed
if ! command -v docker &> /dev/null; then
    echo "Docker is not installed or not in PATH. Please install Docker first."
    exit 1
fi

# Check if user is logged in to Docker Hub
echo "${BLUE}=== Checking Docker Hub login status ===${NC}"
docker info | grep -q "Username"
if [ $? -ne 0 ]; then
    echo "${BLUE}=== Please log in to Docker Hub ===${NC}"
    docker login
    if [ $? -ne 0 ]; then
        echo "Failed to log in to Docker Hub. Exiting."
        exit 1
    fi
fi

# Build the Docker image with platform specified
echo "${BLUE}=== Building Docker image: ${FULL_IMAGE_NAME} ===${NC}"
docker build --platform linux/amd64 -t ${FULL_IMAGE_NAME} .
if [ $? -ne 0 ]; then
    echo "Docker build failed. Exiting."
    exit 1
fi

# Push the image to Docker Hub
echo "${BLUE}=== Pushing image to Docker Hub ===${NC}"
docker push ${FULL_IMAGE_NAME}
if [ $? -ne 0 ]; then
    echo "Failed to push image to Docker Hub. Exiting."
    exit 1
fi

echo "${GREEN}=== Successfully published ${FULL_IMAGE_NAME} to Docker Hub! ===${NC}"
echo "${BLUE}=== You can view your image at: https://hub.docker.com/r/${DOCKER_USERNAME}/${IMAGE_NAME} ===${NC}"

# Optional: Run the image locally to verify it works
echo "${BLUE}=== Would you like to run the image locally to verify it works? (y/n) ===${NC}"
read -r run_locally

if [[ $run_locally =~ ^[Yy]$ ]]; then
    echo "${BLUE}=== Running container locally on port 8089 ===${NC}"
    docker run -d -p 8089:8089 --name ${IMAGE_NAME}-test ${FULL_IMAGE_NAME}
    echo "${GREEN}=== Container started! Access it at http://localhost:8089/hunterReport ===${NC}"
    echo "${BLUE}=== To stop the container, run: docker stop ${IMAGE_NAME}-test && docker rm ${IMAGE_NAME}-test ===${NC}"
fi

echo "${GREEN}=== Done! ===${NC}" 