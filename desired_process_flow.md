D&D Campaign Generator: Granular Process Flow
Phase 1: User Intake & Data Collection
1.1 Frontend Intake Forms (React + TailwindCSS + Zustand)
typescript// Zustand store structure for intake process
interface IntakeStore {
  // Campaign basics
  campaign: {
    name: string
    genre: string
    tone: string
    magicLevel: 'low' | 'medium' | 'high' | 'very_high'
    technologyLevel: string
    startingLevel: number
    expectedEndingLevel: number
    progressionType: 'milestone' | 'experience' | 'hybrid'
    playerCountMin: number
    playerCountMax: number
    pillarBalance: { combat: number; social: number; exploration: number }
    playerAgencyLevel: 'railroad' | 'guided' | 'sandbox' | 'full_sandbox'
    themes: string[]
  }
  
  // Player Characters
  playerCharacters: Array<{
    playerName: string
    characterName: string
    race: string
    class: string
    level: number
    backstory: string
    goals: string[]
    fears: string[]
    unfinishedBusiness: string
    mentionedNPCs: string[]
    mentionedLocations: string[]
    hometown: string
    playerType: 'actor' | 'explorer' | 'power_gamer' | 'instigator' | 'social' | 'watcher'
  }>
  
  // Important NPCs from backstories
  importantNPCs: Array<{
    name: string
    relationship: string
    associatedPC: string
    description: string
  }>
  
  // Key locations
  keyLocations: Array<{
    name: string
    type: string
    significance: string
    associatedPCs: string[]
  }>
  
  // DM world-building preferences
  worldBuildingPreferences: {
    focusAreas: string[] // Areas to develop more deeply
    desiredLocations: string[] // Specific locations DM wants
    culturalElements: string[] // Cultural aspects to explore
    worldTone: string // Overall world atmosphere
  }EditMapped out comprehensive process flow in plain language.2sThe user wants a granular process flow in plain language, not code. Let me break down the entire application flow from user input through to final display, detailing each step and the tools involved.D&D Campaign Generator: Detailed Process Flow
Phase 1: User Data Collection
Step 1: Initial Intake Forms
The user arrives at the web application and is presented with a multi-step form wizard that collects:

Campaign Overview Information

Basic campaign settings (name, genre, tone, magic level, technology level)
Gameplay preferences (progression type, player agency level, prep style)
Campaign scope (starting/ending levels, estimated sessions, player count)
Thematic balance (combat vs social vs exploration percentages)


Player Character Details

For each PC: name, race, class, level, player name
Detailed backstory information
Character goals, fears, and unfinished business
NPCs mentioned in their backstory
Locations mentioned in their backstory
Player preferences and play style


Important NPCs

Names and descriptions of key NPCs from PC backstories
Their relationships to specific PCs
Any additional context the DM wants to provide


Key Locations

Important locations mentioned in backstories
Location types and significance
Which PCs are connected to each location


DM World-Building Preferences

Desired world themes and atmosphere
Types of cultures and societies to include
Special locations or landmarks they envision
How deeply to integrate PC backstories



Step 2: Form Data Management

All form data is managed in Zustand store on the frontend
Forms include validation to ensure required fields are complete
Progress is saved locally to prevent data loss
User can navigate back and forth between form sections

Phase 2: Backend Processing
Step 3: Form Submission to Backend

When user completes all forms, data is compiled into a single JSON payload
Frontend sends POST request to Go backend server
Backend validates the incoming data structure

Step 4: Prompt Generation
The Go backend transforms the user input into a structured prompt for the Anthropic API:

World Foundation Prompt Construction

Combines campaign settings with thematic preferences
Incorporates PC backstory elements as context
Structures the request to generate cohesive world elements


Multi-Stage Generation Strategy

First prompt requests world foundation (cosmology, major conflicts, historical context)
Second prompt requests concrete entities (NPCs, locations, items) influenced by PC backstories
Third prompt requests relationships between all generated elements
Fourth prompt ensures all world elements connect to PC backstories



Step 5: Anthropic API Calls
Following the 3-call architecture pattern:

Call 1: Structure Generation

Requests JSON wrapped in XML tags for parsing
Generates IDs, names, and basic relationships
Returns hierarchical structure of campaign elements


Call 2: Description Enhancement

Takes generated structure and adds rich descriptions
Adds secrets, personality details, and deeper context
Maintains consistency with initial structure


Call 3: Plot and Story Elements

Generates story arcs connecting campaign elements
Creates potential adventure hooks based on backstory elements
Ensures world feels lived-in and connected to PCs



Phase 3: Data Processing & Storage
Step 6: Response Parsing

Backend extracts JSON from XML wrapper tags in Anthropic responses
Validates JSON structure against expected schemas
Handles any parsing errors with retry logic

Step 7: Data Transformation

Parsed JSON is mapped to Go structs matching the database schema
Relationships between entities are resolved
IDs are generated for database insertion

Step 8: Database Population
Using sqlc-generated queries:

Transaction-Based Insertion

Begin database transaction
Insert campaign record first to get campaign ID
Insert all related entities in dependency order:

Location types, races, cultures, languages
Locations (respecting parent-child hierarchy)
Characters and NPCs (with PC connections noted)
Items and equipment (especially from backstories)
Organizations and factions (with PC ties)
Relationships between all entities
Backstory elements and connections


Commit transaction if successful, rollback on error


Relationship Mapping

Character relationships are stored in junction tables
Location hierarchies are maintained through parent IDs
Organization memberships are tracked
Item ownership is assigned



Phase 4: GraphQL API Setup
Step 9: GraphQL Schema Generation

Database schema is used to generate GraphQL types
Queries are created for fetching campaign data
Mutations are set up for future editing capabilities
Subscriptions are configured for real-time updates during generation

Step 10: Data Availability

GraphQL resolvers query the PostgreSQL database using sqlc
Efficient queries with proper indexing for performance
Nested data is resolved through joins and relationship tables

Phase 5: Frontend Display
Step 11: Data Fetching
Using urql GraphQL client:

Frontend requests complete campaign data
Queries are optimized to fetch only needed fields
Data is cached for performance

Step 12: State Management

Fetched data populates Zustand stores
Stores are organized by domain (characters, locations, quests, etc.)
Computed values are derived for efficient rendering

Step 13: UI Rendering

Written Pages Mode

Campaign overview dashboard
Hierarchical navigation through locations
Character detail pages with relationships
Backstory element visualization
Searchable indexes of all entities
Each page uses Tailwind for consistent styling


Graphical Node View

Interactive node graph showing entity relationships
Different node types for characters, locations, items
Edges represent relationships with labels
Zoom and pan capabilities
Click nodes to see details
Filter by relationship type or entity category



Step 14: Editing Capabilities

Each entity has an edit mode
Changes are sent to backend via GraphQL mutations
Optimistic updates show changes immediately
Rollback on server errors
Version tracking for major changes

Phase 6: Generation Progress Monitoring
Step 15: Real-time Updates

WebSocket connection established for generation progress
GraphQL subscriptions report generation status
Progress bar shows which phase is currently running
Estimated time remaining based on typical generation duration
Error states are clearly communicated

Step 16: Post-Generation Options

Download complete campaign as PDF
Export to common VTT formats
Share read-only link with players
Continue editing and refining
Generate additional content for specific areas

Error Handling Throughout

Network failures trigger retries with exponential backoff
Partial generation failures allow recovery from last successful step
User is informed of any issues with clear, actionable messages
All errors are logged for debugging
Graceful degradation when optional features fail

Performance Optimizations

Database queries use indexes on commonly filtered columns
Full-text search enabled for character and location searching
Lazy loading of detailed descriptions until needed
Image assets (if any) are optimized and cached
Frontend bundles are code-split by route

This entire process typically takes 2-3 minutes for initial generation, with the UI being immediately responsive once data is loaded. The system is designed to handle campaigns of varying complexity, from simple one-shots to multi-year epics with hundreds of NPCs and locations.RetryJGOk. Please write this Process Flow again, but add a small code snippet to each that demonstrates that step of the process flow, or tool usage.EditPlotted code snippet strategy for process flow rewrite.2sThe user wants me to rewrite the process flow but include small code snippets for each step to demonstrate the implementation. I should keep the snippets concise and focused on showing the key aspects of each step.D&D Campaign Generator: Detailed Process Flow with Code Examples
Phase 1: User Data Collection
Step 1: Initial Intake Forms
The user arrives at the web application and is presented with a multi-step form wizard that collects campaign information.
typescript// Form component example
const CampaignBasicsForm = () => {
  const { campaign, setCampaignField } = useIntakeStore()
  
  return (
    <div className="space-y-6 p-6 bg-gray-900 rounded-lg">
      <input
        className="w-full p-3 bg-gray-800 border border-purple-500/20 rounded"
        value={campaign.name}
        onChange={(e) => setCampaignField('name', e.target.value)}
        placeholder="Campaign Name"
      />
      <select
        className="w-full p-3 bg-gray-800 border border-purple-500/20 rounded"
        value={campaign.magicLevel}
        onChange={(e) => setCampaignField('magicLevel', e.target.value)}
      >
        <option value="low">Low Magic</option>
        <option value="medium">Medium Magic</option>
        <option value="high">High Magic</option>
      </select>
    </div>
  )
}
Step 2: Form Data Management
All form data is managed in Zustand store with persistence.
typescript// Zustand store for intake process
const useIntakeStore = create<IntakeStore>((set, get) => ({
  campaign: {
    name: '',
    magicLevel: 'medium',
    pillarBalance: { combat: 40, social: 30, exploration: 30 }
  },
  playerCharacters: [],
  
  addPlayerCharacter: (pc) => set(state => ({
    playerCharacters: [...state.playerCharacters, pc]
  })),
  
  // Validate before submission
  isComplete: () => {
    const state = get()
    return state.campaign.name && state.playerCharacters.length > 0
  }
}))
Phase 2: Backend Processing
Step 3: Form Submission to Backend
Frontend compiles and sends data to Go backend.
typescript// Frontend submission
const submitCampaign = async () => {
  const intakeData = useIntakeStore.getState()
  
  const response = await fetch('/api/generate-campaign', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(intakeData)
  })
  
  const { jobId } = await response.json()
  // Start monitoring generation progress
  monitorGeneration(jobId)
}
go// Backend handler
func handleGenerateCampaign(w http.ResponseWriter, r *http.Request) {
    var intake IntakeData
    if err := json.NewDecoder(r.Body).Decode(&intake); err != nil {
        http.Error(w, err.Error(), http.StatusBadRequest)
        return
    }
    
    jobID := uuid.New().String()
    go generateCampaign(jobID, intake) // Async generation
    
    json.NewEncoder(w).Encode(map[string]string{"jobId": jobID})
}
Step 4: Prompt Generation
The Go backend transforms user input into structured prompts.
gofunc buildWorldFoundationPrompt(intake IntakeData) string {
    prompt := fmt.Sprintf(`
Generate a D&D campaign world foundation. Return JSON wrapped in <world_foundation> tags.

Campaign Settings:
- Name: %s
- Magic Level: %s
- Themes: %s

Player Characters:
%s

<world_foundation>
{
  "cosmology": {...},
  "major_conflicts": [...],
  "historical_context": {...}
}
</world_foundation>
`, 
    intake.Campaign.Name,
    intake.Campaign.MagicLevel,
    strings.Join(intake.Campaign.Themes, ", "),
    formatPCBackstories(intake.PlayerCharacters))
    
    return prompt
}
Step 5: Anthropic API Calls
Following the 3-call architecture pattern:
go// Call 1: Structure Generation
func generateStructure(client *anthropic.Client, intake IntakeData) (*ParsedStructure, error) {
    prompt := buildStructurePrompt(intake)
    
    response, err := client.Messages.Create(context.Background(), &anthropic.MessageRequest{
        Model:       "claude-3-opus-20240229",
        MaxTokens:   8000,
        Temperature: 0.8,
        System:      "You are a D&D campaign generator. Output ONLY XML tags containing valid JSON.",
        Messages: []anthropic.Message{
            {Role: "user", Content: prompt},
        },
    })
    
    if err != nil {
        return nil, err
    }
    
    return parseStructure(response.Content)
}
Phase 3: Data Processing & Storage
Step 6: Response Parsing
Backend extracts and validates JSON from responses.
gofunc parseStructure(xmlResponse string) (*ParsedStructure, error) {
    // Extract JSON from XML tags
    re := regexp.MustCompile(`<campaign_structure>([\s\S]*?)</campaign_structure>`)
    matches := re.FindStringSubmatch(xmlResponse)
    
    if len(matches) < 2 {
        return nil, errors.New("no campaign_structure tags found")
    }
    
    var structure ParsedStructure
    if err := json.Unmarshal([]byte(matches[1]), &structure); err != nil {
        return nil, fmt.Errorf("failed to parse JSON: %w", err)
    }
    
    return &structure, nil
}
Step 7: Data Transformation
Parsed data is mapped to database structs.
gofunc transformToDBModels(structure *ParsedStructure, campaignID int64) []db.CreateCharacterParams {
    var characters []db.CreateCharacterParams
    
    for _, char := range structure.Characters {
        characters = append(characters, db.CreateCharacterParams{
            CampaignID:        campaignID,
            Name:             char.Name,
            CharacterType:    "npc",
            CurrentLocationID: sql.NullInt64{Int64: findLocationID(char.LocationID), Valid: true},
            CoreIdentity:     sql.NullString{String: char.CoreIdentity, Valid: true},
        })
    }
    
    return characters
}
Step 8: Database Population
Using sqlc-generated queries with transactions:
gofunc populateDatabase(ctx context.Context, sqlDB *sql.DB, data *GeneratedData) error {
    tx, err := sqlDB.Begin()
    if err != nil {
        return err
    }
    defer tx.Rollback()
    
    qtx := db.New(tx)
    
    // Create campaign
    campaign, err := qtx.CreateCampaignWithData(ctx, db.CreateCampaignWithDataParams{
        Name:        data.Name,
        Description: sql.NullString{String: data.Description, Valid: true},
        MagicLevel:  sql.NullString{String: data.MagicLevel, Valid: true},
    })
    if err != nil {
        return err
    }
    
    // Batch insert locations with hierarchy
    for _, loc := range data.Locations {
        _, err := qtx.CreateLocation(ctx, db.CreateLocationParams{
            CampaignID:       campaign.ID,
            Name:            loc.Name,
            LocationTypeID:  loc.TypeID,
            ParentLocationID: sql.NullInt64{Int64: loc.ParentID, Valid: loc.ParentID > 0},
        })
        if err != nil {
            return err
        }
    }
    
    return tx.Commit()
}
Phase 4: GraphQL API Setup
Step 9: GraphQL Schema Generation
GraphQL schema with types matching database:
graphqltype Campaign {
  id: ID!
  name: String!
  magicLevel: MagicLevel!
  worldBuildingFocus: WorldBuildingFocus!
  characters: [Character!]!
  locations: [Location!]!
  backstoryElements: [BackstoryElement!]!
}

type Query {
  campaign(id: ID!): Campaign
  charactersByLocation(locationId: ID!): [Character!]!
}

type Subscription {
  generationProgress(jobId: ID!): GenerationProgress!
}
Step 10: Data Availability
GraphQL resolvers query the database:
gofunc (r *queryResolver) Campaign(ctx context.Context, id string) (*model.Campaign, error) {
    campaignID, err := strconv.ParseInt(id, 10, 64)
    if err != nil {
        return nil, err
    }
    
    // Use sqlc-generated query
    campaign, err := r.db.GetCampaignOverview(ctx, campaignID)
    if err != nil {
        return nil, err
    }
    
    return &model.Campaign{
        ID:         strconv.FormatInt(campaign.ID, 10),
        Name:       campaign.Name,
        MagicLevel: model.MagicLevel(campaign.MagicLevel.String),
    }, nil
}
Phase 5: Frontend Display
Step 11: Data Fetching
Using urql GraphQL client:
typescriptconst CAMPAIGN_QUERY = graphql(`
  query GetCampaign($id: ID!) {
    campaign(id: $id) {
      id
      name
      magicLevel
      characters {
        id
        name
        currentLocation { id name }
        relationships {
          otherCharacter { id name }
          type
        }
      }
    }
  }
`)

const CampaignView = ({ campaignId }: { campaignId: string }) => {
  const [result] = useQuery({
    query: CAMPAIGN_QUERY,
    variables: { id: campaignId }
  })
  
  if (result.fetching) return <CampaignSkeleton />
  if (result.error) return <ErrorDisplay error={result.error} />
  
  return <CampaignDisplay campaign={result.data.campaign} />
}
Step 12: State Management
Fetched data populates Zustand stores:
typescriptconst useCampaignStore = create<CampaignStore>((set, get) => ({
  campaign: null,
  characters: [],
  locations: [],
  
  loadCampaign: (campaignData) => set({
    campaign: campaignData,
    characters: campaignData.characters,
    locations: campaignData.locations
  }),
  
  // Computed getter for character relationships
  getCharacterRelationships: (characterId) => {
    const { characters } = get()
    const character = characters.find(c => c.id === characterId)
    return character?.relationships || []
  }
}))
Step 13: UI Rendering
Written Pages Mode:
typescriptconst LocationHierarchy = () => {
  const locations = useCampaignStore(s => s.locations)
  
  const renderLocation = (location: Location, depth = 0) => (
    <div 
      key={location.id} 
      className={`pl-${depth * 4} py-2 hover:bg-gray-800 cursor-pointer`}
    >
      <Link to={`/locations/${location.id}`}>
        <span className="text-purple-300">{location.name}</span>
        <span className="text-gray-500 ml-2">({location.type})</span>
      </Link>
      {location.children?.map(child => renderLocation(child, depth + 1))}
    </div>
  )
  
  return (
    <div className="bg-gray-900 rounded-lg p-4">
      {locations.filter(l => !l.parentId).map(l => renderLocation(l))}
    </div>
  )
}
Graphical Node View:
typescriptconst RelationshipGraph = () => {
  const { characters, locations } = useCampaignStore()
  
  useEffect(() => {
    const nodes = [
      ...characters.map(c => ({ id: c.id, label: c.name, type: 'character' })),
      ...locations.map(l => ({ id: l.id, label: l.name, type: 'location' }))
    ]
    
    const edges = characters.flatMap(c => 
      c.relationships.map(r => ({
        source: c.id,
        target: r.otherCharacterId,
        label: r.type
      }))
    )
    
    // Initialize D3 or similar graph library
    renderGraph(nodes, edges)
  }, [characters, locations])
  
  return <div id="graph-container" className="w-full h-screen" />
}
Step 14: Editing Capabilities
Each entity has edit functionality:
typescriptconst UPDATE_CHARACTER = graphql(`
  mutation UpdateCharacter($id: ID!, $input: CharacterUpdateInput!) {
    updateCharacter(id: $id, input: $input) {
      id
      name
      currentLocation { id name }
    }
  }
`)

const CharacterEditForm = ({ character }: { character: Character }) => {
  const [, updateCharacter] = useMutation(UPDATE_CHARACTER)
  
  const handleSave = async (updates: CharacterUpdateInput) => {
    const result = await updateCharacter(
      { id: character.id, input: updates },
      {
        optimisticResponse: {
          updateCharacter: { ...character, ...updates }
        }
      }
    )
    
    if (!result.error) {
      toast.success('Character updated!')
    }
  }
  
  return <form onSubmit={handleSave}>...</form>
}
Phase 6: Generation Progress Monitoring
Step 15: Real-time Updates
WebSocket subscription for generation progress:
typescriptconst GENERATION_PROGRESS = graphql(`
  subscription GenerationProgress($jobId: ID!) {
    generationProgress(jobId: $jobId) {
      stage
      progress
      currentAction
      completed
    }
  }
`)

const GenerationMonitor = ({ jobId }: { jobId: string }) => {
  const [result] = useSubscription({
    query: GENERATION_PROGRESS,
    variables: { jobId }
  })
  
  const { stage, progress, currentAction } = result.data?.generationProgress || {}
  
  return (
    <div className="bg-gray-900 p-6 rounded-lg">
      <h3 className="text-xl font-bold text-purple-300 mb-4">
        Generating Your Campaign...
      </h3>
      <div className="w-full bg-gray-800 rounded-full h-4 mb-2">
        <div 
          className="bg-purple-600 h-full rounded-full transition-all"
          style={{ width: `${progress}%` }}
        />
      </div>
      <p className="text-gray-400">{stage}: {currentAction}</p>
    </div>
  )
}
Step 16: Post-Generation Options
Options available after generation completes:
typescriptconst PostGenerationActions = ({ campaignId }: { campaignId: string }) => {
  const exportCampaign = async (format: 'pdf' | 'foundry' | 'roll20') => {
    const response = await fetch(`/api/campaigns/${campaignId}/export`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ format })
    })
    
    const blob = await response.blob()
    downloadBlob(blob, `campaign.${format}`)
  }
  
  return (
    <div className="flex gap-4">
      <button 
        onClick={() => exportCampaign('pdf')}
        className="px-6 py-3 bg-purple-600 hover:bg-purple-700 rounded"
      >
        Export as PDF
      </button>
      <button 
        onClick={() => generateMore('npcs')}
        className="px-6 py-3 bg-gray-700 hover:bg-gray-600 rounded"
      >
        Generate More NPCs
      </button>
    </div>
  )
}
This process creates a complete D&D campaign in 2-3 minutes, with rich interconnected content that respects both player character backstories and DM preferences while maintaining a living, breathing world.