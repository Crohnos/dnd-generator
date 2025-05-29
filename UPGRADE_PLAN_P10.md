# UPGRADE PLAN PHASE 10: EXPORT & INTEGRATION

## Overview
This phase focuses on implementing comprehensive export functionality and integrations with popular VTT (Virtual Tabletop) platforms, enabling users to use their generated worlds across different tools. Exports emphasize the PC-centric nature of the world and maintain backstory connections.

## Current State
- No export functionality
- No external integrations
- Limited sharing capabilities
- No API access

## Target State
- Multiple export formats (PDF, JSON, CSV)
- VTT platform integrations (Foundry, Roll20, D&D Beyond)
- API for third-party tools
- Selective export capabilities
- Template-based exports
- Version control integration

## Implementation Steps

### Step 1: Export System Architecture
**Duration**: 3 days

1. **Export Service Foundation**
   ```typescript
   // src/services/export/exportService.ts
   export interface ExportOptions {
     format: 'pdf' | 'json' | 'csv' | 'foundry' | 'roll20' | 'dndbeyond';
     entities: string[];
     includeRelationships: boolean;
     includeSecrets: boolean;
     template?: string;
   }
   
   export abstract class BaseExporter {
     abstract format: string;
     abstract mimeType: string;
     abstract extension: string;
     
     abstract export(
       campaign: Campaign,
       entities: Map<string, any[]>,
       options: ExportOptions
     ): Promise<Blob>;
     
     protected filterEntities(
       entities: Map<string, any[]>,
       options: ExportOptions
     ): Map<string, any[]> {
       const filtered = new Map();
       
       options.entities.forEach(type => {
         const items = entities.get(type) || [];
         const processedItems = options.includeSecrets
           ? items
           : items.map(item => this.removeSecrets(item));
         
         filtered.set(type, processedItems);
       });
       
       return filtered;
     }
     
     protected removeSecrets(entity: any): any {
       const secretFields = [
         'secret_info',
         'hidden_information',
         'dm_notes',
         'private_notes',
         'backstory_gm_notes'
       ];
       
       const cleaned = { ...entity };
       secretFields.forEach(field => delete cleaned[field]);
       
       return cleaned;
     }
   }
   ```

2. **PDF Exporter**
   ```typescript
   // src/services/export/pdfExporter.ts
   import { jsPDF } from 'jspdf';
   import 'jspdf-autotable';
   
   export class PDFExporter extends BaseExporter {
     format = 'pdf';
     mimeType = 'application/pdf';
     extension = 'pdf';
     
     async export(
       campaign: Campaign,
       entities: Map<string, any[]>,
       options: ExportOptions
     ): Promise<Blob> {
       const doc = new jsPDF();
       const filtered = this.filterEntities(entities, options);
       
       // Title page
       this.addTitlePage(doc, campaign);
       
       // Table of contents
       doc.addPage();
       this.addTableOfContents(doc, filtered);
       
       // Entity sections
       let pageNumber = 3;
       for (const [entityType, items] of filtered) {
         doc.addPage();
         pageNumber = this.addEntitySection(
           doc,
           entityType,
           items,
           pageNumber
         );
       }
       
       // Add page numbers
       this.addPageNumbers(doc);
       
       return doc.output('blob');
     }
     
     private addEntitySection(
       doc: jsPDF,
       entityType: string,
       items: any[],
       startPage: number
     ): number {
       // Section header
       doc.setFontSize(18);
       doc.text(this.formatTitle(entityType), 20, 30);
       
       let yPosition = 50;
       
       items.forEach(item => {
         if (yPosition > 250) {
           doc.addPage();
           yPosition = 30;
         }
         
         // Entity card
         yPosition = this.renderEntity(doc, item, yPosition, entityType);
       });
       
       return doc.internal.getNumberOfPages();
     }
     
     private renderEntity(
       doc: jsPDF,
       entity: any,
       startY: number,
       entityType: string
     ): number {
       const template = this.getTemplate(entityType);
       return template.render(doc, entity, startY);
     }
   }
   ```

### Step 2: VTT Platform Integrations
**Duration**: 4 days

1. **Foundry VTT Exporter**
   ```typescript
   // src/services/export/foundryExporter.ts
   export class FoundryExporter extends BaseExporter {
     format = 'foundry';
     mimeType = 'application/json';
     extension = 'json';
     
     async export(
       campaign: Campaign,
       entities: Map<string, any[]>,
       options: ExportOptions
     ): Promise<Blob> {
       const world = {
         name: campaign.name,
         title: campaign.name,
         description: campaign.description,
         system: 'dnd5e',
         version: '2.0.0',
         compatibility: {
           minimum: '10',
           verified: '11',
         },
         folders: this.createFolders(entities),
         actors: this.convertCharacters(entities.get('characters') || []),
         scenes: this.convertLocations(entities.get('locations') || []),
         journal: this.convertJournalEntries(entities),
         items: this.convertItems(entities.get('items') || []),
       };
       
       return new Blob(
         [JSON.stringify(world, null, 2)],
         { type: this.mimeType }
       );
     }
     
     private convertCharacters(characters: Character[]): FoundryActor[] {
       return characters.map(char => ({
         _id: this.generateFoundryId(char.id),
         name: char.name,
         type: char.character_type === 'pc' ? 'character' : 'npc',
         system: {
           abilities: this.convertAbilities(char.stats),
           attributes: {
             hp: {
               value: char.stats?.hit_points_current,
               max: char.stats?.hit_points_max,
             },
             ac: {
               value: char.stats?.armor_class || 10,
             },
           },
           details: {
             biography: {
               value: this.formatBiography(char),
               public: this.formatPublicBiography(char),
             },
             backstory_connections: this.formatBackstoryConnections(char),
             race: char.race?.name,
             class: char.class,
             level: char.level,
           },
         },
         items: [],
         effects: [],
         folder: this.getFolderId('characters', char.character_type),
       }));
     }
   }
   ```

2. **Roll20 Exporter**
   ```typescript
   // src/services/export/roll20Exporter.ts
   export class Roll20Exporter extends BaseExporter {
     format = 'roll20';
     mimeType = 'application/json';
     extension = 'json';
     
     async export(
       campaign: Campaign,
       entities: Map<string, any[]>,
       options: ExportOptions
     ): Promise<Blob> {
       const campaignData = {
         schema_version: 1,
         campaign: {
           name: campaign.name,
           description: campaign.description,
         },
         characters: this.convertToRoll20Format(
           entities.get('characters') || []
         ),
         handouts: this.createHandouts(entities),
         maps: this.convertLocationsToMaps(
           entities.get('locations') || []
         ),
       };
       
       return new Blob(
         [JSON.stringify(campaignData, null, 2)],
         { type: this.mimeType }
       );
     }
   }
   ```

### Step 3: Template System
**Duration**: 2 days

1. **Export Templates**
   ```typescript
   // src/services/export/templates/templateEngine.ts
   import Handlebars from 'handlebars';
   
   export class TemplateEngine {
     private templates: Map<string, HandlebarsTemplateDelegate>;
     
     constructor() {
       this.templates = new Map();
       this.registerHelpers();
       this.loadTemplates();
     }
     
     private registerHelpers(): void {
       Handlebars.registerHelper('formatList', (items: any[]) => {
         if (!items || items.length === 0) return 'None';
         return items.join(', ');
       });
       
       Handlebars.registerHelper('formatCR', (cr: number) => {
         if (cr < 1) return `1/${Math.round(1/cr)}`;
         return cr.toString();
       });
       
       Handlebars.registerHelper('statModifier', (score: number) => {
         const modifier = Math.floor((score - 10) / 2);
         return modifier >= 0 ? `+${modifier}` : modifier.toString();
       });
     }
     
     renderTemplate(
       templateName: string,
       data: any,
       format: 'html' | 'markdown' = 'html'
     ): string {
       const template = this.templates.get(`${templateName}.${format}`);
       if (!template) {
         throw new Error(`Template ${templateName}.${format} not found`);
       }
       
       return template(data);
     }
   }
   ```

2. **Character Template Example**
   ```handlebars
   <!-- templates/character.html.hbs -->
   <div class="character-sheet">
     <h1>{{name}}</h1>
     <div class="header-info">
       <span class="race">{{race.name}}</span>
       <span class="class">Level {{level}} {{class}}</span>
     </div>
     
     <div class="stats">
       {{#each stats}}
       <div class="stat">
         <h4>{{@key}}</h4>
         <div class="score">{{this}}</div>
         <div class="modifier">{{statModifier this}}</div>
       </div>
       {{/each}}
     </div>
     
     <div class="details">
       <h3>Background</h3>
       <p>{{core_identity}}</p>
       
       <h3>Motivation</h3>
       <p>{{primary_motivation}}</p>
       
       {{#if personality_traits}}
       <h3>Personality Traits</h3>
       <ul>
         {{#each personality_traits}}
         <li>{{this}}</li>
         {{/each}}
       </ul>
       {{/if}}
     </div>
     
     {{#if includeSecrets}}
     <div class="dm-section">
       <h3>DM Information</h3>
       <p>{{hidden_information}}</p>
     </div>
     {{/if}}
   </div>
   ```

### Step 4: API Development
**Duration**: 3 days

1. **REST API Endpoints**
   ```rust
   // backend/src/handlers/api.rs
   use axum::{
       extract::{Path, Query, State},
       Json,
       response::IntoResponse,
   };
   
   pub async fn get_campaign_export(
       Path(campaign_id): Path<i32>,
       Query(params): Query<ExportParams>,
       State(state): State<AppState>,
   ) -> Result<impl IntoResponse> {
       let campaign = state.db.get_campaign_by_id(campaign_id).await?;
       
       // Validate access
       validate_api_access(&params.api_key, campaign_id)?;
       
       // Build export data
       let export_data = match params.format.as_str() {
           "json" => export_json(&campaign, &params).await?,
           "csv" => export_csv(&campaign, &params).await?,
           _ => return Err(ApiError::InvalidFormat),
       };
       
       Ok(Json(export_data))
   }
   
   pub async fn export_entities(
       Path((campaign_id, entity_type)): Path<(i32, String)>,
       Query(params): Query<ExportParams>,
       State(state): State<AppState>,
   ) -> Result<impl IntoResponse> {
       validate_api_access(&params.api_key, campaign_id)?;
       
       let entities = match entity_type.as_str() {
           "characters" => {
               let chars = state.db.get_campaign_characters(campaign_id).await?;
               serde_json::to_value(chars)?
           }
           "locations" => {
               let locs = state.db.get_campaign_locations(campaign_id).await?;
               serde_json::to_value(locs)?
           }
           _ => return Err(ApiError::InvalidEntityType),
       };
       
       Ok(Json(entities))
   }
   ```

2. **API Authentication**
   ```rust
   // backend/src/middleware/auth.rs
   #[derive(Debug, Serialize, Deserialize)]
   pub struct ApiKey {
       pub id: i32,
       pub key: String,
       pub campaign_id: i32,
       pub permissions: Vec<String>,
       pub rate_limit: i32,
       pub expires_at: Option<DateTime<Utc>>,
   }
   
   pub async fn validate_api_key(
       key: &str,
       campaign_id: i32,
       required_permission: &str,
   ) -> Result<ApiKey> {
       let api_key = sqlx::query_as!(
           ApiKey,
           r#"
           SELECT * FROM api_keys
           WHERE key = $1 AND campaign_id = $2
           AND (expires_at IS NULL OR expires_at > NOW())
           "#,
           key,
           campaign_id
       )
       .fetch_one(&pool)
       .await?;
       
       if !api_key.permissions.contains(&required_permission.to_string()) {
           return Err(ApiError::InsufficientPermissions);
       }
       
       // Check rate limit
       check_rate_limit(&api_key).await?;
       
       Ok(api_key)
   }
   ```

### Step 5: Version Control Integration
**Duration**: 1 day

1. **Git Export**
   ```typescript
   // src/services/export/gitExporter.ts
   export class GitExporter {
     async exportToGit(
       campaign: Campaign,
       entities: Map<string, any[]>,
       options: GitExportOptions
     ): Promise<void> {
       const exportDir = path.join(options.directory, campaign.name);
       
       // Create directory structure
       await this.createDirectoryStructure(exportDir);
       
       // Export campaign metadata
       await fs.writeFile(
         path.join(exportDir, 'campaign.json'),
         JSON.stringify(campaign, null, 2)
       );
       
       // Export entities by type
       for (const [entityType, items] of entities) {
         const entityDir = path.join(exportDir, entityType);
         await fs.mkdir(entityDir, { recursive: true });
         
         for (const item of items) {
           const filename = `${this.sanitizeFilename(item.name)}.json`;
           await fs.writeFile(
             path.join(entityDir, filename),
             JSON.stringify(item, null, 2)
           );
         }
       }
       
       // Create README
       await this.createReadme(exportDir, campaign, entities);
       
       // Initialize git if requested
       if (options.initGit) {
         await this.initializeGitRepo(exportDir);
       }
     }
   }
   ```

## Testing Strategy

1. **Export Validation**
   - Format compliance testing
   - Data integrity verification
   - Secret removal validation

2. **Integration Testing**
   - VTT platform import testing
   - API endpoint testing
   - Template rendering tests

## Success Metrics

- Export generation time < 5s
- 100% format compliance
- Zero data loss in exports
- API response time < 200ms
- Successful imports in all VTT platforms

## Next Phase
Proceed to [UPGRADE_PLAN_P11.md](./UPGRADE_PLAN_P11.md) for Final Integration & Deployment.