FROM node:alpine

WORKDIR /app

# Copy package files and install dependencies
COPY package*.json ./
RUN npm install --production

# Copy source code
COPY index.js ./

# Expose the application port
EXPOSE 3000

# Start the application
CMD ["npm", "start"]
