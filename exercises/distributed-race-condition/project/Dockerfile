FROM node:alpine

WORKDIR /app

COPY package.json ./
RUN npm install --production
COPY index.js ./
COPY seed.js ./

EXPOSE 8888

CMD ["node", "index.js"]
