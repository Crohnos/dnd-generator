# COMPREHENSIVE D&D CAMPAIGN GENERATOR - FULL IMPLEMENTATION PROMPT

  You are tasked with building a complete AI-powered D&D Campaign Generator web application. This is a complex
  full-stack project that must be implemented in discrete, testable phases. **CRITICAL**: Stop and wait for
  feedback/approval after completing each step before proceeding to the next.

  ## PROJECT OVERVIEW

  Build a web application that generates complete D&D campaigns using AI, including NPCs, locations, quest hooks, and
  interconnected storylines. The system must handle real-time generation progress, data persistence, and rich content
  display.

  ## MANDATORY TECHNOLOGY STACK

  - **Frontend**: Next.js 14+ with TypeScript, Tailwind CSS, Zustand state management
  - **Backend**: Rust with Axum web framework, SQLx for database operations
  - **Database**: PostgreSQL with proper migrations and relationships
  - **GraphQL API**: Hasura for real-time data access and subscriptions
  - **AI Integration**: Anthropic Claude API for content generation
  - **Build System**: Justfile for command automation
  - **Containerization**: Docker and Docker Compose for development environment
  - **Type Safety**: GraphQL Code Generator for end-to-end type safety

  ## IMPLEMENTATION PHASES

  ### PHASE 1: PROJECT FOUNDATION & ENVIRONMENT SETUP

  **Objective**: Create the complete project structure and development environment

  **Deliverables**:
  1. Root directory structure with all necessary folders
  2. Docker Compose configuration for development
  3. Justfile with all automation commands
  4. Environment configuration files
  5. Basic README with setup instructions

  **Specific Implementation Requirements**:

  1. **Directory Structure** (create exactly as specified):
  ```
  dnd-campaign-generator/
  ├── docker-compose.yml
  ├── docker-compose.prod.yml
  ├── justfile
  ├── .env.example
  ├── .gitignore
  ├── README.md
  ├── frontend/
  │   ├── package.json
  │   ├── next.config.js
  │   ├── tailwind.config.js
  │   ├── postcss.config.js
  │   ├── tsconfig.json
  │   ├── codegen.yml
  │   └── src/
  │       ├── app/
  │       │   ├── layout.tsx
  │       │   ├── page.tsx
  │       │   ├── globals.css
  │       │   └── campaigns/
  │       ├── components/
  │       ├── lib/
  │       ├── stores/
  │       └── generated/
  ├── backend/
  │   ├── Cargo.toml
  │   ├── Dockerfile
  │   └── src/
  │       ├── main.rs
  │       ├── lib.rs
  │       ├── config.rs
  │       ├── models/
  │       ├── handlers/
  │       └── services/
  ├── database/
  │   └── migrations/
  └── hasura/
      ├── config.yaml
      └── metadata/
  ```

  2. **Docker Compose Configuration**:
     - PostgreSQL 16 with health checks
     - Hasura GraphQL Engine v2.36.0
     - Proper volume mounts for development
     - Environment variable configuration
     - Network configuration for service communication

  3. **Justfile Commands** (implement all these commands):
     - `install`: Install all dependencies and setup database
     - `dev`: Start development environment
     - `db-setup`: Initialize database with migrations
     - `db-reset`: Reset database completely
     - `codegen`: Generate GraphQL types
     - `build`: Production build
     - `test`: Run all tests
     - `clean`: Clean all build artifacts and containers

  4. **Environment Configuration**:
     - `.env.example` with all required variables
     - Clear documentation of each environment variable
     - Security considerations for production

  **Testing Criteria**:
  - `just install` completes without errors
  - `just dev` starts all services (PostgreSQL, Hasura)
  - Database connection successful at localhost:5432
  - Hasura console accessible at localhost:8080
  - All environment variables properly documented
  - Directory structure matches specification exactly

  **STOP HERE**: Verify all services start correctly and wait for approval before proceeding.

  ---

  ### PHASE 2: DATABASE SCHEMA & MIGRATIONS

  **Objective**: Create complete database schema with proper relationships and constraints

  **Deliverables**:
  1. PostgreSQL migration files for all tables
  2. Proper foreign key relationships
  3. Indexes for performance optimization
  4. Sample data for testing
  5. Database validation and constraints

  **Specific Implementation Requirements**:

  1. **Migration Files** (create in exact order):
     - `001_initial.sql`: Core campaigns table with trigger functions
     - `002_npcs.sql`: NPCs table with relationships
     - `003_locations.sql`: Locations and location_npcs junction table
     - `004_quests_encounters.sql`: Quest hooks and encounters tables

  2. **Table Specifications**:

     **campaigns table**:
     - id: SERIAL PRIMARY KEY
     - name: TEXT NOT NULL
     - setting: TEXT
     - themes: TEXT[] DEFAULT '{}'
     - player_characters: JSONB DEFAULT '[]'
     - status: TEXT DEFAULT 'generating' CHECK (status IN ('generating', 'ready', 'error'))
     - metadata: JSONB DEFAULT '{}'
     - created_at, updated_at: TIMESTAMPTZ with auto-update trigger

     **npcs table**:
     - id: SERIAL PRIMARY KEY
     - campaign_id: INTEGER NOT NULL REFERENCES campaigns(id) ON DELETE CASCADE
     - name: TEXT NOT NULL
     - role: TEXT
     - description: TEXT
     - personality: JSONB DEFAULT '{}'
     - stats: JSONB DEFAULT '{}'
     - secret_info: TEXT
     - created_at, updated_at: TIMESTAMPTZ

     **locations table**:
     - id: SERIAL PRIMARY KEY
     - campaign_id: INTEGER NOT NULL REFERENCES campaigns(id) ON DELETE CASCADE
     - name: TEXT NOT NULL
     - type: TEXT
     - description: TEXT
     - connections: INTEGER[] DEFAULT '{}'
     - properties: JSONB DEFAULT '{}'
     - created_at, updated_at: TIMESTAMPTZ

     **location_npcs table** (many-to-many junction):
     - id: SERIAL PRIMARY KEY
     - location_id: INTEGER NOT NULL REFERENCES locations(id) ON DELETE CASCADE
     - npc_id: INTEGER NOT NULL REFERENCES npcs(id) ON DELETE CASCADE
     - relationship_type: TEXT DEFAULT 'resident'
     - UNIQUE(location_id, npc_id)

     **quest_hooks table**:
     - id: SERIAL PRIMARY KEY
     - campaign_id: INTEGER NOT NULL REFERENCES campaigns(id) ON DELETE CASCADE
     - title: TEXT NOT NULL
     - description: TEXT
     - difficulty: TEXT DEFAULT 'medium'
     - reward: TEXT
     - related_npc_ids: INTEGER[] DEFAULT '{}'
     - related_location_ids: INTEGER[] DEFAULT '{}'
     - status: TEXT DEFAULT 'available' CHECK (status IN ('available', 'active', 'completed'))

     **encounters table**:
     - id: SERIAL PRIMARY KEY
     - campaign_id: INTEGER NOT NULL REFERENCES campaigns(id) ON DELETE CASCADE
     - location_id: INTEGER REFERENCES locations(id) ON DELETE SET NULL
     - title: TEXT NOT NULL
     - description: TEXT
     - difficulty: TEXT DEFAULT 'medium'
     - creatures: JSONB DEFAULT '[]'
     - environmental_factors: TEXT

  3. **Required Indexes**:
     - idx_npcs_campaign_id ON npcs(campaign_id)
     - idx_locations_campaign_id ON locations(campaign_id)
     - idx_location_npcs_location_id ON location_npcs(location_id)
     - idx_location_npcs_npc_id ON location_npcs(npc_id)
     - idx_quest_hooks_campaign_id ON quest_hooks(campaign_id)
     - idx_encounters_campaign_id ON encounters(campaign_id)

  4. **Sample Data**: Create comprehensive sample campaign with:
     - 1 complete campaign ("The Shattered Crown")
     - 3-4 NPCs with full personality data
     - 3-4 locations with properties and connections
     - 3-4 quest hooks linking NPCs and locations
     - Proper relationships in junction tables

  **Testing Criteria**:
  - All migrations run successfully via `sqlx migrate run`
  - Foreign key constraints work properly
  - Sample data inserts without errors
  - Can query relationships between all tables
  - Indexes created and functioning
  - Database triggers update timestamps correctly
  - `just db-reset` and `just db-setup` work flawlessly

  **STOP HERE**: Verify database schema is complete and all relationships work before proceeding.

  ---

  ### PHASE 3: RUST BACKEND SERVICE FOUNDATION

  **Objective**: Build the complete Rust backend with database integration and basic HTTP endpoints

  **Deliverables**:
  1. Complete Rust application structure
  2. Database connection and configuration
  3. Basic HTTP endpoints for campaigns
  4. Error handling and logging
  5. Health check endpoint

  **Specific Implementation Requirements**:

  1. **Cargo.toml Dependencies** (exact versions required):
  ```toml
  [dependencies]
  tokio = { version = "1.40", features = ["full"] }
  axum = { version = "0.7", features = ["json", "headers"] }
  tower = "0.5"
  tower-http = { version = "0.5", features = ["cors", "trace"] }
  sqlx = { version = "0.8", features = ["runtime-tokio", "postgres", "json", "chrono", "uuid"] }
  serde = { version = "1.0", features = ["derive"] }
  serde_json = "1.0"
  reqwest = { version = "0.12", features = ["json"] }
  anyhow = "1.0"
  thiserror = "1.0"
  dotenvy = "0.15"
  tracing = "0.1"
  tracing-subscriber = { version = "0.3", features = ["env-filter"] }
  uuid = { version = "1.10", features = ["v4", "serde"] }
  chrono = { version = "0.4", features = ["serde"] }
  ```

  2. **Application Structure**:

     **src/main.rs**:
     - Complete Axum server setup with CORS
     - Tracing/logging configuration
     - Database connection with health checks
     - Route definitions for all endpoints
     - Graceful error handling

     **src/config.rs**:
     - Environment variable configuration
     - Database URL, Anthropic API key, port settings
     - Validation of required environment variables

     **src/models/** (create separate files):
     - `campaign.rs`: Campaign structs, create requests, generated content models
     - `npc.rs`: NPC models with personality traits
     - `location.rs`: Location models with properties
     - `mod.rs`: Module exports

     **src/handlers/** (create separate files):
     - `health.rs`: Health check endpoint
     - `campaign.rs`: All campaign-related endpoints
     - `mod.rs`: Handler module exports

     **src/services/** (create separate files):
     - `database.rs`: Database operations and queries
     - `anthropic.rs`: Anthropic API client (placeholder for Phase 4)
     - `mod.rs`: Service module exports

  3. **Required HTTP Endpoints**:
     - `GET /health`: Health check with database connectivity
     - `POST /api/campaigns`: Create new campaign
     - `GET /api/campaigns/:id`: Get campaign with all related data
     - `POST /api/campaigns/:id/generate`: Trigger AI generation (placeholder)
     - `POST /api/campaigns/:id/encounters`: Generate random encounters (placeholder)

  4. **Database Service Methods**:
     - `create_campaign()`: Insert new campaign record
     - `get_campaign_by_id()`: Fetch campaign with NPCs, locations, quests
     - `update_campaign_status()`: Update generation status
     - `save_generated_content()`: Save AI-generated content (prepare for Phase 4)

  5. **Error Handling**:
     - Custom error types for different failure modes
     - Proper HTTP status codes (400, 404, 500)
     - Structured error responses
     - Database connection error handling

  **Testing Criteria**:
  - `cargo build` compiles without warnings
  - `cargo test` passes all unit tests
  - Server starts on configured port (default 3001)
  - `GET /health` returns 200 with JSON response
  - `POST /api/campaigns` creates campaigns successfully
  - `GET /api/campaigns/:id` returns complete campaign data
  - Database queries work with proper error handling
  - CORS headers present for frontend integration
  - Logging works at appropriate levels

  **STOP HERE**: Verify all endpoints work and database integration is solid before proceeding.

  ---

  ### PHASE 4: ANTHROPIC AI INTEGRATION

  **Objective**: Implement complete AI content generation using Anthropic Claude API

  **Deliverables**:
  1. Anthropic API client with proper error handling
  2. Structured prompt engineering for campaign generation
  3. JSON parsing and validation of AI responses
  4. Asynchronous content generation with status updates
  5. Content persistence to database

  **Specific Implementation Requirements**:

  1. **Anthropic Client Implementation**:
     - HTTP client using reqwest with proper headers
     - API key authentication
     - Rate limiting consideration
     - Retry logic for transient failures
     - Proper error types for API failures

  2. **Content Generation Models** (define exact JSON structures):

  ```rust
  #[derive(Debug, Serialize, Deserialize)]
  pub struct GeneratedCampaignContent {
      pub npcs: Vec<GeneratedNPC>,
      pub locations: Vec<GeneratedLocation>,
      pub quest_hooks: Vec<GeneratedQuestHook>,
      pub plot_summary: String,
      pub central_conflict: String,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct GeneratedNPC {
      pub name: String,
      pub role: String,
      pub description: String,
      pub personality: NPCPersonality,
      pub secret_info: Option<String>,
  }

  #[derive(Debug, Serialize, Deserialize)]
  pub struct NPCPersonality {
      pub traits: Vec<String>,
      pub motivation: String,
      pub fears: Option<Vec<String>>,
      pub connections: Option<Vec<String>>,
  }

  // Similar detailed structures for locations and quests
  ```

  3. **Prompt Engineering** (exact prompt template):
     - Campaign parameters injection (name, setting, themes, player characters)
     - Specific instructions for JSON output format
     - Requirements for interconnected content (8-12 NPCs, 6-10 locations, 5-8 quests)
     - Instructions for personality depth and secret information
     - Consistency requirements across generated elements

  4. **Generation Process**:
     - Update campaign status to 'generating'
     - Send structured prompt to Anthropic API
     - Parse and validate JSON response
     - Transform AI response to database models
     - Save all content in single database transaction
     - Update campaign status to 'ready' or 'error'
     - Handle partial failures gracefully

  5. **Database Integration**:
     - `save_generated_campaign()` method with transaction handling
     - Batch insert operations for NPCs, locations, quests
     - Create NPC-location relationships from AI response
     - Update campaign metadata with generation timestamp
     - Proper rollback on any failure

  6. **Error Handling**:
     - API quota exceeded errors
     - Invalid JSON response handling
     - Partial content generation failures
     - Database transaction failures
     - Network connectivity issues

  **Testing Criteria**:
  - Anthropic API integration works with test prompt
  - Generated content follows exact JSON schema
  - Content is properly saved to database with relationships
  - Campaign status updates correctly throughout process
  - Error cases are handled gracefully (invalid API key, quota exceeded)
  - `POST /api/campaigns/:id/generate` triggers complete generation
  - Generated campaigns have all required elements (NPCs, locations, quests)
  - Relationships between NPCs and locations are created correctly

  **STOP HERE**: Verify AI integration works end-to-end before proceeding to frontend.

  ---

  ### PHASE 5: NEXT.JS FRONTEND FOUNDATION

  **Objective**: Create the complete Next.js application with TypeScript, Tailwind, and basic routing

  **Deliverables**:
  1. Next.js 14 application with App Router
  2. Tailwind CSS configuration and global styles
  3. TypeScript configuration
  4. Basic page structure and routing
  5. Component architecture foundation

  **Specific Implementation Requirements**:

  1. **Package.json Dependencies** (exact versions):
  ```json
  {
    "dependencies": {
      "next": "14.0.4",
      "react": "^18.2.0",
      "react-dom": "^18.2.0",
      "urql": "^4.0.6",
      "@urql/next": "^1.1.0",
      "graphql": "^16.8.1",
      "zustand": "^4.4.7",
      "lucide-react": "^0.263.1",
      "clsx": "^2.0.0"
    },
    "devDependencies": {
      "@types/node": "^20.10.5",
      "@types/react": "^18.2.45",
      "@types/react-dom": "^18.2.18",
      "@graphql-codegen/cli": "^5.0.0",
      "@graphql-codegen/typescript": "^4.0.1",
      "@graphql-codegen/typescript-operations": "^4.0.1",
      "@graphql-codegen/typescript-urql": "^4.0.0",
      "autoprefixer": "^10.4.16",
      "eslint": "^8.56.0",
      "eslint-config-next": "14.0.4",
      "postcss": "^8.4.32",
      "tailwindcss": "^3.4.0",
      "typescript": "^5.3.3"
    }
  }
  ```

  2. **Configuration Files**:

     **next.config.js**:
     - App Router configuration
     - TypeScript strict mode
     - Environment variable exposure

     **tailwind.config.js**:
     - Custom D&D theme colors (dnd-purple: #8B5CF6, dnd-gold: #F59E0B)
     - Component class definitions
     - Dark theme as default

     **tsconfig.json**:
     - Strict TypeScript configuration
     - Path aliases for imports (@/ for src/)
     - Next.js specific settings

  3. **Global Styles** (src/app/globals.css):
     - Tailwind directives
     - Custom component classes:
       - `.btn-primary`: Purple buttons with hover states
       - `.btn-secondary`: Gray buttons with hover states
       - `.card`: Dark card styling with borders
       - `.input`: Dark input styling with focus states
     - Dark theme base styles

  4. **Application Structure**:

     **src/app/layout.tsx**:
     - Root layout with navigation
     - URQL Provider setup
     - Font configuration (Inter)
     - Navigation bar with D&D branding

     **src/app/page.tsx**:
     - Landing page with hero section
     - Feature cards highlighting AI capabilities
     - Call-to-action buttons

     **src/app/campaigns/page.tsx**:
     - Campaign listing page
     - Empty state for no campaigns
     - Campaign cards with status indicators

     **src/app/campaigns/new/page.tsx**:
     - Campaign creation wizard container

     **src/app/campaigns/[id]/page.tsx**:
     - Campaign detail view container

     **src/app/campaigns/[id]/generating/page.tsx**:
     - Real-time generation progress page

  5. **Component Foundation**:
     - Create placeholder components for:
       - `CampaignWizard`
       - `NPCCard`
       - `LocationCard`
       - `QuestHookCard`
     - Basic TypeScript interfaces for all data types

  6. **State Management Setup**:
     - Zustand store for campaign creation wizard
     - State for form steps, player characters, themes
     - Actions for updating wizard state

  **Testing Criteria**:
  - `npm run dev` starts development server without errors
  - All pages render without TypeScript errors
  - Navigation works between all routes
  - Tailwind classes compile correctly
  - Custom theme colors are applied
  - Responsive design works on mobile and desktop
  - TypeScript strict mode passes
  - ESLint runs without errors

  **STOP HERE**: Verify basic Next.js app works and all routes are accessible before proceeding.

  ---

  ### PHASE 6: HASURA GRAPHQL INTEGRATION

  **Objective**: Configure Hasura for GraphQL API and integrate with frontend

  **Deliverables**:
  1. Complete Hasura metadata configuration
  2. GraphQL schema exposure for all tables
  3. Frontend GraphQL client setup
  4. Type-safe GraphQL code generation
  5. Basic queries and mutations working

  **Specific Implementation Requirements**:

  1. **Hasura Metadata Configuration**:

     Create exact metadata files:
     - `hasura/metadata/databases/default/tables/campaigns.yaml`
     - `hasura/metadata/databases/default/tables/npcs.yaml`
     - `hasura/metadata/databases/default/tables/locations.yaml`
     - `hasura/metadata/databases/default/tables/quest_hooks.yaml`
     - `hasura/metadata/databases/default/tables/location_npcs.yaml`

  2. **Table Permissions**:
     - Select permissions for all tables (public for MVP)
     - Insert permissions for campaigns table
     - Update permissions for campaign status
     - Proper relationship configurations

  3. **GraphQL Relationships**:
     - campaigns → npcs (one-to-many)
     - campaigns → locations (one-to-many)
     - campaigns → quest_hooks (one-to-many)
     - locations ↔ npcs (many-to-many via location_npcs)

  4. **Frontend GraphQL Setup**:

     **src/lib/urql.ts**:
     - URQL client configuration
     - Hasura endpoint connection
     - Admin secret header (development only)
     - Subscription support (WebSocket)

     **codegen.yml**:
     - GraphQL Code Generator configuration
     - Hasura schema introspection
     - TypeScript + URQL plugin configuration
     - Output to src/generated/graphql.tsx

  5. **GraphQL Operations** (create in src/graphql/):

     **campaigns.graphql**:
     ```graphql
     query GetCampaigns {
       campaigns(order_by: { created_at: desc }) {
         id
         name
         setting
         themes
         status
         created_at
       }
     }

     query GetCampaign($id: Int!) {
       campaigns_by_pk(id: $id) {
         id
         name
         setting
         themes
         status
         metadata
         npcs { ... }
         locations { ... }
         quest_hooks { ... }
       }
     }

     mutation CreateCampaign($input: campaigns_insert_input!) {
       insert_campaigns_one(object: $input) {
         id
         name
         status
       }
     }

     subscription CampaignProgress($id: Int!) {
       campaigns_by_pk(id: $id) {
         id
         status
         metadata
       }
     }
     ```

  6. **Code Generation Integration**:
     - Add codegen script to package.json
     - Configure justfile to run codegen
     - Generate typed hooks for all operations
     - Ensure types match backend Rust models

  **Testing Criteria**:
  - Hasura console loads at localhost:8080
  - All database tables visible in GraphQL schema
  - Relationships work correctly in GraphQL explorer
  - `npm run codegen` generates types without errors
  - URQL client connects to Hasura successfully
  - Basic queries return data from sample campaign
  - TypeScript compilation passes with generated types
  - GraphQL operations work in browser network tab

  **STOP HERE**: Verify GraphQL integration is complete and type-safe before proceeding.

  ---

  ### PHASE 7: CAMPAIGN CREATION WIZARD

  **Objective**: Build the complete multi-step campaign creation wizard with form validation

  **Deliverables**:
  1. Multi-step wizard component with progress tracking
  2. Form validation and state management
  3. Player character management
  4. Theme selection interface
  5. Integration with backend API

  **Specific Implementation Requirements**:

  1. **Wizard State Management** (Zustand store):

  ```typescript
  interface CampaignState {
    name: string;
    setting: string;
    themes: string[];
    playerCharacters: PlayerCharacter[];
    currentStep: number;

    // Actions for each field
    setName: (name: string) => void;
    setSetting: (setting: string) => void;
    setThemes: (themes: string[]) => void;
    setPlayerCharacters: (characters: PlayerCharacter[]) => void;
    setCurrentStep: (step: number) => void;
    reset: () => void;
  }
  ```

  2. **Wizard Steps** (exactly 4 steps):
     - Step 0: Basic Information (campaign name)
     - Step 1: Player Characters (add/edit/remove characters)
     - Step 2: World & Themes (setting description + theme selection)
     - Step 3: Review & Generate (summary + generate button)

  3. **Player Character Interface**:
  ```typescript
  interface PlayerCharacter {
    name: string;
    class: string;
    race: string;
    background?: string;
    personalityTraits: string[];
  }
  ```

  4. **Form Components**:

     **Step 0 - Basic Information**:
     - Campaign name input (required)
     - Real-time validation
     - Clear error states

     **Step 1 - Player Characters**:
     - Dynamic character list with add/remove
     - Dropdowns for classes (Fighter, Wizard, Rogue, Cleric, etc.)
     - Dropdowns for races (Human, Elf, Dwarf, Halfling, etc.)
     - Background text input
     - Validation: at least 1 character, all required fields filled

     **Step 2 - World & Themes**:
     - Setting description textarea (required)
     - Theme selection grid with toggle buttons
     - Predefined themes: political intrigue, war, mystery, exploration, horror, comedy, romance, apocalyptic, urban,
  wilderness, underdark, planar travel
     - Validation: setting required, at least 1 theme selected

     **Step 3 - Review & Generate**:
     - Summary of all entered information
     - Player character list display
     - Selected themes display
     - Generate button with loading state

  5. **Navigation Controls**:
     - Progress bar showing current step
     - Previous/Next buttons with proper validation
     - Disabled states when validation fails
     - Generate button only on final step

  6. **API Integration**:
     - Create campaign via backend API (not GraphQL for this step)
     - Handle success/error states
     - Redirect to generation progress page
     - Form reset after successful creation

  7. **Visual Design Requirements**:
     - Dark theme consistent with overall app
     - Purple accent colors for primary actions
     - Responsive design for mobile devices
     - Loading animations and transitions
     - Clear visual hierarchy

  **Testing Criteria**:
  - All 4 wizard steps render correctly
  - Progress bar updates properly
  - Form validation prevents invalid submissions
  - Player characters can be added/edited/removed
  - Theme selection works with visual feedback
  - Navigation between steps works correctly
  - API integration creates campaigns successfully
  - Error handling works for API failures
  - Responsive design works on mobile
  - Loading states show during API calls
  - Successful creation redirects to generation page

  **STOP HERE**: Verify complete wizard works end-to-end before proceeding.

  ---

  ### PHASE 8: CAMPAIGN DISPLAY & CONTENT VISUALIZATION

  **Objective**: Build rich campaign display pages with NPC, location, and quest visualization

  **Deliverables**:
  1. Campaign overview page with tabbed navigation
  2. NPC cards with personality traits and secrets
  3. Location cards with atmospheric descriptions
  4. Quest hook cards with difficulty indicators
  5. Real-time generation progress page

  **Specific Implementation Requirements**:

  1. **Campaign Detail Page Structure**:

     **Header Section**:
     - Campaign name and setting
     - Theme tags display
     - Back navigation to campaigns list
     - Tabbed navigation (Overview, NPCs, Locations, Quests)

     **Tab Implementation**:
     - State-driven tab switching
     - Visual active state indicators
     - Responsive tab layout
     - Icon + label for each tab

  2. **NPC Card Component**:

  ```typescript
  interface NPCCardProps {
    npc: {
      id: number;
      name: string;
      role: string;
      description: string;
      personality: {
        traits: string[];
        motivation: string;
        fears: string[];
        connections: string[];
      };
      secret_info: string;
    };
  }
  ```

     **NPC Card Features**:
     - Character avatar placeholder with role color coding
     - Name and role prominently displayed
     - Description paragraph with proper line height
     - Personality traits as colored tags
     - Motivation section
     - Fears display (if present)
     - Expandable "DM Secrets" section with eye icon toggle
     - Yellow warning styling for secret information

  3. **Location Card Component**:

  ```typescript
  interface LocationCardProps {
    location: {
      id: number;
      name: string;
      type: string;
      description: string;
      properties: {
        atmosphere: string;
        notable_features: string[];
        secrets: string[];
        connections: string[];
      };
    };
  }
  ```

     **Location Card Features**:
     - Location icon with type-based coloring
     - Name and type display
     - Atmospheric description
     - Notable features as bullet points
     - Expandable hidden secrets section
     - Connection information to other locations

  4. **Quest Hook Card Component**:

  ```typescript
  interface QuestHookCardProps {
    quest: {
      id: number;
      title: string;
      description: string;
      difficulty: 'easy' | 'medium' | 'hard';
      reward: string;
      status: 'available' | 'active' | 'completed';
    };
  }
  ```

     **Quest Card Features**:
     - Quest scroll icon
     - Title and difficulty badge (color-coded: green/yellow/red)
     - Status indicator badge
     - Description text
     - Reward information with star icon
     - Related NPCs and locations (if data available)

  5. **Generation Progress Page**:
     - Large animated icon (wand or dice)
     - Progress bar with animated fill
     - Status text updates
     - Feature highlight cards showing what's being generated
     - Automatic redirect when complete
     - Error state handling

  6. **Overview Tab Content**:
     - Plot summary display (if available in metadata)
     - Central conflict description
     - Statistics cards (X NPCs, Y Locations, Z Quests)
     - Visual summary of campaign scope

  7. **Responsive Grid Layouts**:
     - NPCs: 3 columns on desktop, 2 on tablet, 1 on mobile
     - Locations: 2 columns on desktop, 1 on mobile
     - Quests: 3 columns on desktop, 2 on tablet, 1 on mobile
     - Proper gap spacing and card sizing

  8. **Interactive Features**:
     - Hover effects on cards
     - Smooth transitions for tab switching
     - Collapsible sections for secrets
     - Loading skeletons while data fetches

  **Testing Criteria**:
  - Campaign page loads with all tabs functional
  - NPC cards display all personality information correctly
  - Secret information toggles work with proper styling
  - Location cards show all properties and features
  - Quest cards display difficulty and status correctly
  - Generation progress page shows real-time updates
  - Responsive design works across all device sizes
  - Tab navigation works smoothly
  - Loading states display properly
  - Error states handle missing data gracefully
  - All icons and colors match design specification

  **STOP HERE**: Verify all display components work correctly before proceeding.

  ---

  ### PHASE 9: REAL-TIME FEATURES & GRAPHQL SUBSCRIPTIONS

  **Objective**: Implement real-time updates for campaign generation progress

  **Deliverables**:
  1. GraphQL subscription for campaign status updates
  2. Real-time progress tracking during generation
  3. Automatic page transitions based on status changes
  4. WebSocket connection management
  5. Error handling for connection issues

  **Specific Implementation Requirements**:

  1. **GraphQL Subscription Setup**:

     **WebSocket Client Configuration**:
     - Update URQL client to include subscription exchange
     - WebSocket connection to Hasura
     - Proper authentication headers
     - Connection retry logic

  2. **Subscription Queries**:

  ```graphql
  subscription CampaignProgress($id: Int!) {
    campaigns_by_pk(id: $id) {
      id
      status
      metadata
      updated_at
    }
  }
  ```

  3. **Generation Progress Component**:
     - Subscribe to campaign status changes
     - Display different UI states based on status:
       - 'generating': Progress animation and updates
       - 'ready': Success state with redirect countdown
       - 'error': Error state with retry options
     - Simulated progress bar (since actual progress isn't available)
     - Automatic redirect to campaign page when ready

  4. **Progress Simulation**:
     - Animated progress bar that fills over time
     - Random increments to show activity
     - Different phases of generation (NPCs, Locations, Quests)
     - Visual indicators for each generation phase

  5. **Error Handling**:
     - WebSocket connection failures
     - Subscription timeout handling
     - Network disconnection recovery
     - User-friendly error messages

  6. **Campaign Status Integration**:
     - Backend updates campaign status during generation
     - Frontend reacts to status changes immediately
     - Proper cleanup of subscriptions on component unmount
     - Handle edge cases (page refresh during generation)

  **Testing Criteria**:
  - WebSocket connection establishes successfully
  - Subscription receives updates when campaign status changes
  - Progress page shows real-time updates
  - Automatic redirect works when generation completes
  - Error states display when generation fails
  - Subscription cleanup prevents memory leaks
  - Network disconnection/reconnection handled gracefully
  - Multiple users can watch same campaign generation

  **STOP HERE**: Verify real-time features work correctly before proceeding.

  ---

  ### PHASE 10: FINAL INTEGRATION & TESTING

  **Objective**: Complete end-to-end integration, comprehensive testing, and deployment preparation

  **Deliverables**:
  1. Complete end-to-end workflow testing
  2. Error handling and edge case coverage
  3. Performance optimization and caching
  4. Production deployment configuration
  5. Documentation and setup guides

  **Specific Implementation Requirements**:

  1. **End-to-End Workflow Testing**:
     - Complete campaign creation through wizard
     - AI generation process with real Anthropic API
     - Real-time progress updates via WebSocket
     - Campaign display with all content types
     - Navigation between all pages and states

  2. **Error Handling Comprehensive Coverage**:
     - Network failures during API calls
     - Invalid Anthropic API responses
     - Database connection failures
     - Malformed user input validation
     - WebSocket connection issues
     - Missing environment variables

  3. **Performance Optimizations**:
     - GraphQL query optimization
     - Image loading optimizations
     - Bundle size analysis and reduction
     - Database query indexing verification
     - API response caching strategies

  4. **Production Configuration**:

     **Environment Variables**:
     - Production Hasura configuration
     - Secure API key management
     - Database connection pooling
     - CORS configuration for production domains

     **Docker Production Setup**:
     - Multi-stage Dockerfile for frontend
     - Optimized Rust binary for backend
     - Production docker-compose.yml
     - Health checks for all services

  5. **Security Considerations**:
     - API key security (never exposed to frontend)
     - Input sanitization and validation
     - SQL injection prevention (already handled by SQLx)
     - CORS policy configuration
     - Rate limiting considerations

  6. **Documentation Requirements**:

     **README.md Updates**:
     - Complete setup instructions
     - Environment variable documentation
     - Development workflow guide
     - Production deployment steps
     - Troubleshooting common issues

     **API Documentation**:
     - All backend endpoints documented
     - Request/response examples
     - Error response formats
     - Rate limiting information

  7. **Final Testing Checklist**:
     - [ ] All justfile commands work correctly
     - [ ] Complete campaign creation workflow
     - [ ] AI generation produces valid content
     - [ ] Real-time updates work consistently
     - [ ] All responsive breakpoints tested
     - [ ] Error states display properly
     - [ ] Database relationships function correctly
     - [ ] TypeScript compilation without errors
     - [ ] ESLint passes without warnings
     - [ ] Docker containers build successfully
     - [ ] Production environment setup works

  8. **Load Testing** (basic):
     - Multiple concurrent campaign generations
     - Database performance under load
     - WebSocket connection limits
     - Frontend performance with large datasets

  **Final Testing Criteria**:
  - Complete user journey works without errors
  - All error states are user-friendly and actionable
  - Production build deploys successfully
  - Documentation is complete and accurate
  - Performance meets acceptable standards
  - Security considerations are properly implemented
  - Code quality standards are met throughout

  **FINAL CHECKPOINT**: Verify entire application works as a complete, production-ready system.

  ---

  ## CRITICAL SUCCESS FACTORS

  1. **Stop at Each Phase**: Do not proceed to the next phase until explicitly approved
  2. **Test Everything**: Each phase must pass all testing criteria before continuing
  3. **Follow Specifications Exactly**: Don't deviate from the technology stack or requirements
  4. **Handle Errors Gracefully**: Implement comprehensive error handling at every level
  5. **Maintain Type Safety**: Ensure TypeScript/Rust type safety throughout
  6. **Document Decisions**: Explain any technical decisions or trade-offs made
  7. **Optimize for Maintainability**: Write clean, well-structured code that others can understand

  Remember: This is a complex full-stack application. Take time to implement each phase thoroughly rather than rushing
  through. The success of later phases depends on the solid foundation built in earlier phases.[I
