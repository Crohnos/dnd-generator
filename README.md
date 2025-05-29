# D&D Campaign Generator

An AI-powered web application for generating complete Dungeons & Dragons campaigns, including NPCs, locations, quest hooks, and interconnected storylines.

## Features

- 🎲 AI-powered campaign generation using Anthropic Claude
- 👥 Rich NPC creation with personalities, motivations, and secrets
- 🗺️ Interconnected locations with atmospheric descriptions
- 📜 Dynamic quest hooks with difficulty levels
- ⚡ Real-time generation progress tracking
- 🎨 Dark theme UI optimized for DMs

## Technology Stack

- **Frontend**: Next.js 14+ with TypeScript, Tailwind CSS, Zustand
- **Backend**: Rust with Axum web framework, SQLx
- **Database**: PostgreSQL 16
- **GraphQL**: Hasura for real-time subscriptions
- **AI**: Anthropic Claude API
- **Build**: Justfile for automation
- **Container**: Docker & Docker Compose

## Prerequisites

- Docker and Docker Compose
- Rust (latest stable)
- Node.js 18+ and npm
- Just command runner (`cargo install just`)
- Anthropic API key

## Quick Start

1. **Clone the repository**
   ```bash
   git clone https://github.com/yourusername/dnd-campaign-generator.git
   cd dnd-campaign-generator
   ```

2. **Set up environment variables**
   ```bash
   cp .env.example .env
   # Edit .env and add your Anthropic API key
   ```

3. **Install dependencies and setup**
   ```bash
   just install
   ```

4. **Start development environment**
   ```bash
   just dev
   ```

5. **Start backend and frontend servers**
   
   In separate terminals:
   ```bash
   # Terminal 1 - Backend
   just backend

   # Terminal 2 - Frontend
   just frontend
   ```

6. **Access the application**
   - Frontend: http://localhost:3000
   - Hasura Console: http://localhost:8080
   - Backend API: http://localhost:3001

## Development Commands

```bash
just               # Show all available commands
just dev           # Start Docker services
just install       # Install all dependencies
just db-setup      # Initialize database
just db-reset      # Reset database
just codegen       # Generate GraphQL types
just build         # Production build
just test          # Run all tests
just clean         # Clean all artifacts
just logs          # View service logs
just status        # Check service status
```

## Project Structure

```
dnd-campaign-generator/
├── frontend/          # Next.js application
│   ├── src/
│   │   ├── app/      # App router pages
│   │   ├── components/
│   │   ├── lib/      # Utilities
│   │   ├── stores/   # Zustand stores
│   │   └── generated/ # GraphQL types
│   └── package.json
├── backend/           # Rust API server
│   ├── src/
│   │   ├── models/   # Data models
│   │   ├── handlers/ # HTTP handlers
│   │   └── services/ # Business logic
│   └── Cargo.toml
├── database/          # PostgreSQL migrations
├── hasura/           # GraphQL metadata
├── docker-compose.yml
├── justfile          # Development commands
└── README.md
```

## Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `DATABASE_URL` | PostgreSQL connection string | postgres://postgres:postgres@localhost:5432/dnd_campaigns |
| `ANTHROPIC_API_KEY` | Your Anthropic API key | Required |
| `PORT` | Backend server port | 3001 |
| `HASURA_ADMIN_SECRET` | Hasura admin secret | myadminsecretkey |
| `NEXT_PUBLIC_GRAPHQL_URL` | GraphQL endpoint for frontend | http://localhost:8080/v1/graphql |
| `NEXT_PUBLIC_API_URL` | Backend API URL for frontend | http://localhost:3001 |

## Database Schema

The application uses PostgreSQL with the following main tables:
- `campaigns` - Campaign metadata and settings
- `npcs` - Non-player characters with personalities
- `locations` - Places in the campaign world
- `quest_hooks` - Adventure opportunities
- `encounters` - Combat and role-play encounters
- `location_npcs` - Many-to-many relationships

## Troubleshooting

### Services won't start
```bash
just clean
just install
just dev
```

### Database connection issues
```bash
just db-reset
```

### Port conflicts
Check if ports 3000, 3001, 5432, or 8080 are in use:
```bash
lsof -i :3000
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests: `just test`
5. Submit a pull request

## License

MIT License - see LICENSE file for details