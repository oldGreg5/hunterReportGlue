# --- Build Stage: Frontend (WASM) ---
FROM rust:1.88-slim AS frontend-builder

# Install wasm-pack
RUN apt-get update && apt-get install -y curl pkg-config libssl-dev && \
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

WORKDIR /usr/src/app
COPY . .

WORKDIR /usr/src/app/frontend
RUN wasm-pack build --target web --out-dir ../static/hunterReport/pkg

# --- Build Stage: Backend ---
FROM rust:1.88-slim AS backend-builder

WORKDIR /usr/src/app
COPY . .
# Copy the built frontend from the previous stage
COPY --from=frontend-builder /usr/src/app/static/hunterReport/pkg ./static/hunterReport/pkg

RUN cargo build --release --bin hunter_report_backend

# --- Final Runtime Stage ---
FROM debian:bookworm-slim

# Install necessary runtime libraries (for docx-rs or other dependencies if needed)
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

# Create a non-root user
RUN groupadd -r nodeapp && useradd -r -g nodeapp nodeapp

# Copy binary and static assets
COPY --from=backend-builder /usr/src/app/target/release/hunter_report_backend ./hunter_report_backend
COPY --from=backend-builder /usr/src/app/static ./static
COPY --from=backend-builder /usr/src/app/data ./data

# Set proper permissions
RUN chown -R nodeapp:nodeapp /usr/src/app

USER nodeapp

# Expose the port used in backend/src/main.rs (8081)
EXPOSE 8081

# Start the application
CMD ["./hunter_report_backend"]
