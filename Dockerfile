FROM nginx:alpine
COPY index.html /usr/share/nginx/html/
COPY pkg /usr/share/nginx/html/pkg
ENV HOST 0.0.0.0
