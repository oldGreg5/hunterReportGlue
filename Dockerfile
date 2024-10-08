FROM node:alpine
LABEL authors="oldgreg5"

# Set the working directory
WORKDIR /usr/src/app

# Copy package.json and package-lock.json
COPY package*.json ./

# Install dependencies
RUN npm install

# Copy the application files
COPY . .

# Expose the port
EXPOSE 8089

# Start the application
ENTRYPOINT [ "node", "testNode.js" ]
