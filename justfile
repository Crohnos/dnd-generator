# D&D Campaign Generator - Development Commands

# Default recipe to display help information
default:
  @just --list

# Install all dependencies and setup database
install:
  @echo "Installing backend dependencies..."
  cd backend && cargo build
  @echo "Installing frontend dependencies..."
  cd frontend && npm install
  @echo "Setting up database..."
  just db-setup
  @echo "Installation complete!"

# Start development environment
dev:
  @echo "Starting development environment..."
  docker-compose up -d
  @echo "Waiting for services to be ready..."
  sleep 10
  @echo "Services started!"
  @echo "PostgreSQL: localhost:5432"
  @echo "Hasura Console: http://localhost:8080"
  @echo ""
  @echo "Run backend with: cd backend && cargo run"
  @echo "Run frontend with: cd frontend && npm run dev"

# Initialize database with migrations  
db-setup:
  @echo "Setting up database..."
  docker-compose up -d postgres
  @echo "Waiting for PostgreSQL to be ready..."
  sleep 10
  @echo "Running main schema migration..."
  docker-compose exec -T postgres psql -U postgres -d dnd_campaigns -f /docker-entrypoint-initdb.d/schema.sql 2>/dev/null || echo "Schema already applied"
  @echo "Starting Hasura..."
  docker-compose up -d hasura
  @echo "Waiting for Hasura to be ready..."
  sleep 10
  @echo "Database setup complete!"

# Reset database completely
db-reset:
  @echo "Resetting database..."
  docker-compose down -v
  docker-compose up -d postgres
  @echo "Waiting for PostgreSQL to be ready..."
  sleep 10
  @echo "Starting Hasura..."
  docker-compose up -d hasura
  @echo "Waiting for Hasura to be ready..."
  sleep 10
  @echo "Reloading Hasura metadata..."
  curl -s -X POST http://localhost:8080/v1/metadata \
    -H "x-hasura-admin-secret: myadminsecretkey" \
    -H "Content-Type: application/json" \
    -d '{"type": "reload_metadata", "args": {}}' > /dev/null || echo "Metadata reload failed (will retry)"
  sleep 2
  @echo "Database reset complete!"

# Generate GraphQL types
codegen:
  @echo "Generating GraphQL types..."
  cd frontend && npm run codegen
  @echo "GraphQL types generated!"

# Production build
build:
  @echo "Building for production..."
  @echo "Building backend..."
  cd backend && cargo build --release
  @echo "Building frontend..."
  cd frontend && npm run build
  @echo "Production build complete!"

# Run all tests
test:
  @echo "Running backend tests..."
  cd backend && cargo test
  @echo "Running frontend tests..."
  cd frontend && npm test
  @echo "All tests complete!"

# Clean all build artifacts and containers
clean:
  @echo "Cleaning up..."
  docker-compose down -v
  rm -rf backend/target
  rm -rf frontend/.next
  rm -rf frontend/node_modules
  @echo "Cleanup complete!"

# Start backend development server
backend:
  cd backend && cargo run

# Start frontend development server
frontend:
  cd frontend && npm run dev

# View logs for all services
logs:
  docker-compose logs -f

# View logs for specific service
log service:
  docker-compose logs -f {{service}}

# Check status of all services
status:
  docker-compose ps

# Run database migrations manually
migrate:
  docker-compose exec -T postgres psql -U postgres -d dnd_campaigns -f /docker-entrypoint-initdb.d/001_initial.sql
  docker-compose exec -T postgres psql -U postgres -d dnd_campaigns -f /docker-entrypoint-initdb.d/002_npcs.sql
  docker-compose exec -T postgres psql -U postgres -d dnd_campaigns -f /docker-entrypoint-initdb.d/003_locations.sql
  docker-compose exec -T postgres psql -U postgres -d dnd_campaigns -f /docker-entrypoint-initdb.d/004_quests_encounters.sql
  docker-compose exec -T postgres psql -U postgres -d dnd_campaigns -f /docker-entrypoint-initdb.d/005_sample_data.sql
  docker-compose exec -T postgres psql -U postgres -d dnd_campaigns -f /docker-entrypoint-initdb.d/006_world_building.sql
  docker-compose exec -T postgres psql -U postgres -d dnd_campaigns -f /docker-entrypoint-initdb.d/007_character_building.sql
  docker-compose exec -T postgres psql -U postgres -d dnd_campaigns -f /docker-entrypoint-initdb.d/008_entity_system.sql
  docker-compose exec -T postgres psql -U postgres -d dnd_campaigns -f /docker-entrypoint-initdb.d/009_locations_enhanced.sql
  docker-compose exec -T postgres psql -U postgres -d dnd_campaigns -f /docker-entrypoint-initdb.d/010_items_system.sql
  docker-compose exec -T postgres psql -U postgres -d dnd_campaigns -f /docker-entrypoint-initdb.d/011_social_systems.sql
  docker-compose exec -T postgres psql -U postgres -d dnd_campaigns -f /docker-entrypoint-initdb.d/012_migration_cleanup.sql
  docker-compose exec -T postgres psql -U postgres -d dnd_campaigns -f /docker-entrypoint-initdb.d/013_additional_systems.sql
  docker-compose exec -T postgres psql -U postgres -d dnd_campaigns -f /docker-entrypoint-initdb.d/014_migration_summary.sql

# Access PostgreSQL CLI
psql:
  docker-compose exec postgres psql -U postgres -d dnd_campaigns

# Apply Hasura metadata
hasura-apply:
  cd hasura && hasura metadata apply --admin-secret myadminsecretkey --endpoint http://localhost:8080

# Export Hasura metadata
hasura-export:
  cd hasura && hasura metadata export --admin-secret myadminsecretkey --endpoint http://localhost:8080

# Production deployment
deploy:
  @echo "Building production images..."
  docker-compose -f docker-compose.prod.yml build
  @echo "Starting production services..."
  docker-compose -f docker-compose.prod.yml up -d
  @echo "Production deployment complete!"