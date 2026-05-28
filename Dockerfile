# Base image
FROM vsc-rust_typescript-127167942dfdd9e1d02ee2383d115ff162d3a1bacd066117963f10d6a15504bb-features

# Install Node.js LTS
RUN curl -fsSL https://deb.nodesource.com/setup_lts.x | bash - \
    && apt-get update \
    && apt-get install -y nodejs \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Set working directory to your Rust API folder
WORKDIR /

# Copy project files
COPY . .

# Expose ports
EXPOSE 6969 5173

WORKDIR /rust-api
RUN cargo vendor > /vendor
RUN cargo build --release

WORKDIR /nuxt
RUN npm install -g serve
RUN npm install nuxt

# Start the Rust API
WORKDIR /
COPY entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh
ENTRYPOINT ["/entrypoint.sh"]
