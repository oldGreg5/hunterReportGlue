# Hunter Report Generator - Docker Setup

This document provides instructions for running the Hunter Report Generator application using Docker.

## Prerequisites

- Docker installed on your system
- Docker Compose installed on your system

## Running the Application with Docker Compose

1. Clone the repository:
```bash
git clone https://github.com/oldGreg5/hunterReportGlue.git
cd hunterReportGlue
```

2. Build and start the application:
```bash
docker-compose up -d
```

3. Access the application in your browser:
```
http://localhost:8089/hunterReport
```

4. Stop the application:
```bash
docker-compose down
```

## Running the Application with Docker (without Docker Compose)

1. Build the Docker image:
```bash
docker build -t hunter-report-app .
```

2. Run the container:
```bash
docker run -d -p 8089:8089 --name hunter-report-app hunter-report-app
```

3. Access the application in your browser:
```
http://localhost:8089/hunterReport
```

4. Stop the container:
```bash
docker stop hunter-report-app
docker rm hunter-report-app
```

## Volume Mounting

The Docker Compose configuration mounts the local `data` directory to the container's data directory. This allows you to:

- Modify the data files without rebuilding the container
- Persist data across container restarts

## Environment Variables

- `NODE_ENV`: Set to "production" by default. You can change this to "development" for debugging.

## Troubleshooting

- If you encounter port conflicts, change the host port in the `docker-compose.yml` file:
  ```yaml
  ports:
    - '8088:8089'  # Change 8089 to another port
  ```

- To view logs:
  ```bash
  docker-compose logs -f app
  ```

- To restart the application:
  ```bash
  docker-compose restart app
  ``` 