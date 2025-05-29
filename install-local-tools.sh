#!/bin/bash
# Install local development tools for the DnD Generator project

set -e

echo "Installing local development tools..."

# Update package list
sudo apt-get update

# Install PostgreSQL client tools
echo "Installing PostgreSQL client..."
sudo apt-get install -y postgresql-client

# Install Node.js and npm if not already installed
if ! command -v node &> /dev/null; then
    echo "Installing Node.js..."
    curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
    sudo apt-get install -y nodejs
fi

# Install Rust if not already installed
if ! command -v cargo &> /dev/null; then
    echo "Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
fi

# Install Docker if not already installed
if ! command -v docker &> /dev/null; then
    echo "Docker not found. Please run ./install-docker.sh first"
    exit 1
fi

# Install just (command runner)
if ! command -v just &> /dev/null; then
    echo "Installing just..."
    curl --proto '=https' --tlsv1.2 -sSf https://just.systems/install.sh | bash -s -- --to /usr/local/bin
fi

# Create helpful aliases
echo "Creating helpful aliases..."
cat >> ~/.bashrc << 'EOF'

# DnD Generator aliases
alias dnd-db="docker-compose exec postgres psql -U postgres -d dnd_campaigns"
alias dnd-db-local="psql -h localhost -p 5432 -U postgres -d dnd_campaigns"
EOF

echo "Installation complete!"
echo "Run 'source ~/.bashrc' to load the new aliases"
echo ""
echo "Available commands:"
echo "  dnd-db         - Connect to PostgreSQL via Docker"
echo "  dnd-db-local   - Connect to PostgreSQL directly (password: postgres)"