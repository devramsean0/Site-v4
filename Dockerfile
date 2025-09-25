# syntax=docker/dockerfile:1

FROM golang:1.21.0 AS go_build
WORKDIR /build/go
COPY tools .
RUN go mod download -C ./nr-station-parser
RUN CGO_ENABLED=1 go build -C ./nr-station-parser -o ../nr-station-parser -v

FROM nixos/nix:latest AS rs_build

# Install packages using Nix
RUN nix-env -iA nixpkgs.gcc nixpkgs.openssl nixpkgs.openssl.dev nixpkgs.pkg-config nixpkgs.bun nixpkgs.tailwindcss_4 nixpkgs.rustc nixpkgs.cargo

WORKDIR /app

# Copy source and asset directories
COPY . .

ENV PKG_CONFIG_PATH=/nix/var/nix/profiles/default/lib/pkgconfig

# build assets (adjust these commands as needed for your project)
RUN bun install
RUN echo "PUBLIC_PRODUCTION=true" >> .env

# build Rust app
RUN cargo build --release

# --- Runner stage ---
FROM nixos/nix:latest AS runner

WORKDIR /app

# Install runtime dependencies (if any)
RUN nix-env -iA nixpkgs.openssl

# Copy built binary and assets from build stage
COPY --from=rs_build /app/target/release/site-v4 /app/site-v4
COPY --from=rs_build /app/assets /app/assets
COPY --from=rs_build /app/templates /app/templates
COPY --from=rs_build /app/compiled_assets /app/compiled_assets

# Copy go deps
COPY --from=go_build /build/go/nr-station-parser/nr-station-parser /app/dist/nr-station-parser

ENV NR_STATION_PARSER_DIST=/app/dist/nr-station-parser

EXPOSE 3000
CMD ["/app/site-v4"]
