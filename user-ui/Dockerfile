FROM node:latest as builder
WORKDIR /data
RUN yarn config set registry https://registry.npmmirror.org/
COPY . .
RUN yarn
RUN yarn build

FROM alpine:latest
WORKDIR /var/www/html
COPY --from=builder /data/dist .
