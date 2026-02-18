# Stage 1: Build React App
FROM node:20-alpine as builder
WORKDIR /app
COPY package*.json ./
RUN npm install
COPY . .
# We need to adjust vite config for this docker build to output to dist inside the container
# or we can just use the default output.
# Let's override the output dir to be local for simplicity
RUN npm run build -- --outDir dist

# Stage 2: Serve with Deno
FROM denoland/deno:alpine
WORKDIR /app
# Copy built assets
COPY --from=builder /app/dist ./dist
COPY server.ts .

# Allow network access and read access
CMD ["run", "--allow-net", "--allow-read", "server.ts"]
