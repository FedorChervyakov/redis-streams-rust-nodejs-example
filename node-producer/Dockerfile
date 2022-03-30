FROM node:16

COPY package*.json ./

# If you are building your code for production
# RUN npm ci --only=production
RUN npm install

COPY . .
