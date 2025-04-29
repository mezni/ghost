# Use official Node image
FROM node:18-alpine

# Set working directory
WORKDIR /app

# Copy package.json and package-lock.json first
COPY package*.json ./

# Install dependencies including react-router-dom
RUN npm install react-router-dom

# Install rest of dependencies (React, Vite, etc.)
RUN npm install

# Copy the rest of the source code
COPY . .

# Build the React app
RUN npm run build

# Use a lightweight server to serve the build
FROM nginx:alpine
COPY --from=0 /app/dist /usr/share/nginx/html

# Expose port
EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
