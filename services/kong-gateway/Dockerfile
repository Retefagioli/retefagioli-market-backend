FROM kong:alpine

RUN mkdir -p "/usr/local/kong/declarative"
COPY kong.yml /usr/local/kong/declarative/kong.yml
EXPOSE 8000 8443 8001 8444 9080 9081
