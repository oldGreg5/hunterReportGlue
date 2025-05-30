FROM node:18-alpine AS build

# Set the working directory
WORKDIR /usr/src/app

# Copy package.json and package-lock.json
COPY package*.json ./

# Install dependencies
RUN npm install

# Copy the application files
COPY . .

# Install browserify globally for bundle generation
RUN npm install -g browserify

# Generate the bundle.js file
RUN mkdir -p scripts && browserify scripts/hunterReport.js -o scripts/bundle.js

# Use a smaller image for the final build
FROM node:18-alpine

# Create a non-root user
RUN addgroup -S nodeapp && \
    adduser -S -G nodeapp nodeapp

WORKDIR /usr/src/app

# Copy only necessary files from the build stage
COPY --from=build /usr/src/app/package*.json ./
COPY --from=build /usr/src/app/hunterReport.html ./
COPY --from=build /usr/src/app/hunterReport.css ./
COPY --from=build /usr/src/app/testNode.js ./
COPY --from=build /usr/src/app/scripts/bundle.js ./scripts/
COPY --from=build /usr/src/app/data ./data/

# Create necessary directories
RUN mkdir -p scripts data

# Install only production dependencies
RUN npm ci --only=production

# Set proper permissions
RUN chown -R nodeapp:nodeapp /usr/src/app

# Use the non-root user
USER nodeapp

# Expose the port
EXPOSE 8089

# Health check
HEALTHCHECK --interval=30s --timeout=5s --start-period=5s --retries=3 \
  CMD wget --no-verbose --tries=1 --spider http://localhost:8089/hunterReport || exit 1

# Start the application
CMD ["node", "testNode.js"]
