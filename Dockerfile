# syntax=docker/dockerfile:1

FROM nixos/nix:latest AS build

# Install packages using Nix
RUN nix-env -iA nixpkgs.gcc nixpkgs.openssl nixpkgs.openssl.dev nixpkgs.pkg-config nixpkgs.bun nixpkgs.tailwindcss_4 nixpkgs.rustc nixpkgs.cargo

WORKDIR /app

# Copy source and asset directories
COPY . .

ENV PKG_CONFIG_PATH=/nix/var/nix/profiles/default/lib/pkgconfig

# Build assets (adjust these commands as needed for your project)
RUN bun install
RUN echo "PUBLIC_PRODUCTION=true" >> .env

# Build Rust app
RUN cargo build --release

# --- Runner stage ---
FROM nixos/nix:latest AS runner

WORKDIR /app

# Install runtime dependencies (if any)
RUN nix-env -iA nixpkgs.openssl

# Copy built binary and assets from build stage
COPY --from=build /app/target/release/site-v4 /app/site-v4
COPY --from=build /app/assets /app/assets
COPY --from=build /app/templates /app/templates
COPY --from=build /app/compiled_assets /app/compiled_assets

EXPOSE 3000
CMD ["/app/site-v4"]
