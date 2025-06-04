# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview
D&D Campaign Generator - An AI-powered web application that helps Dungeon Masters create complete D&D campaigns using Anthropic's Claude API. Generates interconnected NPCs, locations, quest hooks, and encounters with rich storytelling elements.

## Commands
All commands use the `just` command runner. Key development commands:

**Setup:**
- `just install` - Install dependencies and setup database
- `just dev` - Start Docker services (PostgreSQL, Hasura)

**Development:**
- `just backend` - Run Rust backend server (port 3001)
- `just frontend` - Run Next.js frontend (port 3000)
- `just codegen` - Generate TypeScript types from GraphQL schema
- `just db-setup` - Run database migrations
- `just db-reset` - Reset database completely

**Testing & Building:**
- `just test` - Run all tests
- `just build` - Production build
- `just deploy` - Deploy with Docker

**Utilities:**
- `just logs` - View service logs
- `just psql` - Access PostgreSQL CLI

## Architecture

**Backend (Rust + Axum):**
- REST API server handling campaign creation and AI generation
- Uses SQLx for type-safe database operations
- Async content generation with status tracking via database polling
- Models in `src/models/` match database schema exactly

**Frontend (Next.js 14):**
- TypeScript with App Router
- GraphQL client (URQL) with generated types
- Zustand for state management
- Multi-step campaign wizard with real-time progress

**Database (PostgreSQL):**
- 6 main tables: campaigns, npcs, locations, quest_hooks, encounters, location_npcs
- Foreign key relationships with proper indexing
- JSONB fields for flexible AI-generated content

**GraphQL Layer (Hasura):**
- Auto-generates API from database schema
- Real-time subscriptions for campaign status updates
- Console at http://localhost:8080

## Type Safety Flow
Database schema → Rust models → GraphQL schema → TypeScript types
Use `just codegen` after schema changes to regenerate frontend types.

## Development Workflow
1. Database migrations define schema changes
2. Update Rust models to match schema
3. Hasura auto-tracks new tables/relationships
4. Run `just codegen` to update frontend types
5. Backend polls campaign status in database for async generation
6. Frontend subscribes to GraphQL for real-time updates

## Environment Setup
Required environment variables in `.env`:
- `ANTHROPIC_API_KEY` - Claude API key for content generation
- `DATABASE_URL` - PostgreSQL connection string
- `HASURA_ADMIN_SECRET` - Hasura admin access

## Content Generation
AI content generation is async:
1. Campaign created with "generating" status
2. Backend processes generation in background
3. Updates status and content in database
4. Frontend polls via GraphQL subscriptions
5. Status progresses: creating → generating → completed → error

## Code Patterns
- Use `sqlx::query_as!` macros for type-safe database queries
- Error handling with custom `AppError` type and proper HTTP status codes
- React components follow card-based design for content display
- State management with Zustand stores for campaign data