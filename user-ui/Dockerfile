FROM node:18.6.0
WORKDIR /data
RUN yarn config set registry https://registry.npmmirror.org/
COPY . .
RUN yarn
RUN yarn build
