FROM node:16.14.2-alpine
RUN apk add --no-cache make g++

WORKDIR /usr/src/app

COPY . .

RUN npm run build

CMD ["npm","run","start:prod"]