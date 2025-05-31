# Publishing Hunter Report Generator to Docker Hub

This guide will walk you through the steps to publish your Hunter Report Generator application to Docker Hub.

## Prerequisites

- Docker Desktop installed on your machine
- A Docker Hub account (create one at [https://hub.docker.com/signup](https://hub.docker.com/signup) if you don't have one)

## Step 1: Install Docker (if not already installed)

Download and install Docker Desktop from [https://www.docker.com/products/docker-desktop/](https://www.docker.com/products/docker-desktop/)

After installation, verify Docker is working by running:

```bash
docker --version
```

## Step 2: Log in to Docker Hub

Open your terminal and log in to Docker Hub:

```bash
docker login
```

Enter your Docker Hub username and password when prompted.

## Step 3: Update the Dockerfile (Optional)

The Dockerfile is already optimized for production with a multi-stage build process, security features, and proper permissions. You can review it to make any additional customizations if needed.

## Step 4: Build the Docker Image

Build the Docker image with your Docker Hub username:

```bash
docker build -t YOUR_USERNAME/hunter-report-app:latest .
```

Replace `YOUR_USERNAME` with your actual Docker Hub username.

## Step 5: Push the Image to Docker Hub

Push the built image to Docker Hub:

```bash
docker push YOUR_USERNAME/hunter-report-app:latest
```

## Step 6: Verify the Image Works

Run the image locally to verify it works correctly:

```bash
docker run -d -p 8089:8089 --name hunter-report-test YOUR_USERNAME/hunter-report-app:latest
```

Access the application at [http://localhost:8089/hunterReport](http://localhost:8089/hunterReport)

To stop the test container:

```bash
docker stop hunter-report-test && docker rm hunter-report-test
```

## Step 7: Add Image Description (Optional)

For better discoverability and documentation:

1. Go to [https://hub.docker.com/repository/docker/YOUR_USERNAME/hunter-report-app](https://hub.docker.com/repository/docker/YOUR_USERNAME/hunter-report-app)
2. Click on "Manage Repository"
3. Add a description, readme, and other metadata

## Step 8: Using the Published Image

Once published, anyone can use your image with:

```bash
docker pull YOUR_USERNAME/hunter-report-app:latest
docker run -d -p 8089:8089 YOUR_USERNAME/hunter-report-app:latest
```

## Optional: Create a Shell Script for Publishing

For easier publishing in the future, you can use the `publish_docker.sh` script included in your project. Before using it:

1. Edit the script to set your Docker Hub username:
   ```bash
   nano publish_docker.sh
   ```
   
2. Change the `DOCKER_USERNAME` variable to your Docker Hub username
   
3. Make the script executable:
   ```bash
   chmod +x publish_docker.sh
   ```
   
4. Run the script:
   ```bash
   ./publish_docker.sh
   ```

## Troubleshooting

- **Error: Docker command not found**: Make sure Docker Desktop is installed and running
- **Error: Authentication failed**: Verify your Docker Hub credentials
- **Error: Permission denied**: Try running with sudo (Linux/Mac) or as Administrator (Windows)
- **Error: Port already in use**: Change the port mapping (e.g., `-p 8090:8089`)

## Additional Resources

- [Docker Hub Documentation](https://docs.docker.com/docker-hub/)
- [Docker Build Documentation](https://docs.docker.com/engine/reference/commandline/build/)
- [Docker Push Documentation](https://docs.docker.com/engine/reference/commandline/push/) 