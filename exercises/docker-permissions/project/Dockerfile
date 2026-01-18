FROM node:alpine

WORKDIR /app

COPY package.json ./
RUN npm install --production
COPY index.js ./

RUN adduser -D appuser
USER appuser

EXPOSE 8888

CMD ["npm", "start"]
