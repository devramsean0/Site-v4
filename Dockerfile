# syntax=docker/dockerfile:1

FROM golang:1.21.0 AS go_build
WORKDIR /build/go
# Copy only go.mod and go.sum first for better caching
COPY tools/nr-station-parser/go.mod tools/nr-station-parser/go.sum ./nr-station-parser/
RUN go mod download -C ./nr-station-parser
# Copy source code after dependencies are cached
COPY tools ./
RUN CGO_ENABLED=1 go build -C ./nr-station-parser -o ../nr-station-parser -v

FROM nixos/nix:latest AS rs_build

ARG PUBLIC_WEBSOCKET_PROTOCOL
ENV PUBLIC_WEBSOCKET_PROTOCOL=$PUBLIC_WEBSOCKET_PROTOCOL

# Install packages using Nix (this layer will be cached)
RUN nix-env -iA nixpkgs.gcc nixpkgs.openssl nixpkgs.openssl.dev nixpkgs.pkg-config nixpkgs.bun nixpkgs.tailwindcss_4 nixpkgs.rustc nixpkgs.cargo

WORKDIR /app

ENV PKG_CONFIG_PATH=/nix/var/nix/profiles/default/lib/pkgconfig

# Copy package manager files first for better caching
COPY package.json bun.lock ./
COPY Cargo.toml Cargo.lock ./

# Install JS dependencies (cached if package files unchanged)
RUN bun install

# Download Rust dependencies (cached if Cargo files unchanged)
RUN cargo fetch

# Copy source and asset directories
COPY . .

# Build assets
RUN echo "PUBLIC_PRODUCTION=true" >> .env

# Build Rust app (using pre-downloaded dependencies)
RUN cargo build --release --workspace

# --- Runner stage ---
FROM nixos/nix:latest AS runner

WORKDIR /app

# Install runtime dependencies (if any)
RUN nix-env -iA nixpkgs.openssl nixpkgs.sqlite

# Copy built binary and assets from build stage
COPY --from=rs_build /app/target/release/site-v4 /app/site-v4
COPY --from=rs_build /app/target/release/importer /app/importer
COPY --from=rs_build /app/assets /app/assets
COPY --from=rs_build /app/templates /app/templates
COPY --from=rs_build /app/compiled_assets /app/compiled_assets

# Copy go deps
COPY --from=go_build /build/go/nr-station-parser/nr-station-parser /app/dist/nr-station-parser

ENV NR_STATION_PARSER_DIST=/app/dist/nr-station-parser

EXPOSE 3000
CMD ["/app/site-v4"]