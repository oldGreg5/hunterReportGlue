# Hunter Report Generator

Render description of different items from file,
then user picks desired ones and they will be combined and put in clipboard.

## Docker Setup

This application is dockerized for easy deployment. For detailed Docker instructions, see [DOCKER.md](DOCKER.md).

Quick start:
```bash
# Build and run with Docker Compose
docker-compose up -d

# Access the application
# Open http://localhost:8089/hunterReport in your browser
```

## Development Setup

### Build bundle on changes
```bash
# Install dependencies
npm install

# Watch for changes and rebuild bundle
npm run build

# Or with watchify
watchify ./scripts/hunterReport.js -o ./scripts/bundle.js -v
```

### Start server
```bash
# With nodemon (auto-restart on changes)
npm run dev

# With Node.js
npm start

# With PM2
pm2 start testNode.js --watch
```

### Make PM2 start with system
```bash
# Once app is running, check PM2 status
pm2 status

# Set up startup script
pm2 startup

# Save current process list
pm2 save
```

### Access the application
```
http://localhost:8089/hunterReport
```

Note: For remote access, port forwarding has to be done to the machine hosting the app.

## Scripts

The following npm scripts are available:
- `npm start` - Start the application
- `npm run dev` - Start the application with nodemon (auto-restart)
- `npm run build` - Build the bundle.js file
- `npm run docker:build` - Build the Docker image
- `npm run docker:run` - Run the Docker container
- `npm run docker:stop` - Stop and remove the Docker container
- `npm run docker:compose:up` - Start with Docker Compose
- `npm run docker:compose:down` - Stop with Docker Compose