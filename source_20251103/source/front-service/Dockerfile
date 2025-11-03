# Use lightweight nginx base image
FROM nginx:alpine

# Set working directory
WORKDIR /usr/share/nginx/html

# Remove default nginx content
RUN rm -rf ./*

# Copy adminlte UI files (make sure this folder is inside the same directory as Dockerfile)
COPY adminlte/ /usr/share/nginx/html/

# Replace default nginx config with our custom config
COPY nginx.conf /etc/nginx/conf.d/default.conf

# Expose HTTP port
EXPOSE 80

# Run nginx in foreground
CMD ["nginx", "-g", "daemon off;"]
