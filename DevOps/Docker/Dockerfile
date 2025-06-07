FROM ubuntu:24.04

# Avoid interactive prompts during package installation
ENV DEBIAN_FRONTEND=noninteractive

# ---- system packages ----
RUN apt-get update && apt-get install -y --no-install-recommends \
        curl ca-certificates gnupg build-essential pkg-config \
        gcc-arm-linux-gnueabihf libc6-dev-armhf-cross \
        libssl-dev vim libsdl2-dev tree && \
    rm -rf /var/lib/apt/lists/*

# ---- non-root user ----
ARG UID=1000
ARG GID=1000

# More robust user creation that handles all cases
RUN set -ex; \
    # Try to create the group, ignore if it exists
    groupadd -g ${GID} appgroup || echo "Group ${GID} already exists"; \
    # Check if user with UID exists
    if id -u ${UID} >/dev/null 2>&1; then \
        echo "User with UID ${UID} already exists, renaming to appuser"; \
        # Get the existing username
        EXISTING_USER=$(getent passwd ${UID} | cut -d: -f1); \
        # Get the existing group id
        EXISTING_GID=$(id -g ${UID}); \
        # Rename the user
        usermod -l appuser ${EXISTING_USER}; \
        # Change primary group if needed
        if [ "${EXISTING_GID}" != "${GID}" ]; then \
            usermod -g ${GID} appuser; \
        fi; \
        # Ensure home directory is correct
        usermod -d /home/appuser -m appuser || true; \
    else \
        echo "Creating new user appuser with UID ${UID}"; \
        useradd -m -u ${UID} -g appgroup -s /bin/bash appuser; \
    fi; \
    # Verify the user exists
    id appuser

# ---- environment (set before switching user) ----
ENV CARGO_HOME=/home/appuser/.cargo
ENV PATH=${CARGO_HOME}/bin:${PATH}

# Switch to appuser
USER appuser

# ---- Rust toolchain ----
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y --default-toolchain 1.77.0 && \
    rustup component add rustfmt clippy llvm-tools-preview && \
    rustup target add armv7-unknown-linux-gnueabihf && \
    cargo install cargo-tarpaulin --locked --version 0.27.0

# Ensure cargo env is properly set up in bashrc
RUN echo 'export CARGO_HOME=/home/appuser/.cargo' >> ~/.bashrc && \
    echo 'export PATH="$CARGO_HOME/bin:$PATH"' >> ~/.bashrc

WORKDIR /workspace
CMD ["bash"]