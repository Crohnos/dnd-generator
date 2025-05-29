# UPGRADE PLAN PHASE 3: GRAPHQL SCHEMA EVOLUTION

## Overview
This phase focuses on evolving the GraphQL schema from supporting 4 entities to 100+ entities with complex relationships, efficient querying, and real-time subscriptions, all designed to support world-building from PC backstories.

## Current State
- Basic CRUD for 4 entities
- Simple one-to-many relationships
- No pagination or filtering
- Basic subscriptions for status updates

## Target State
- GraphQL types for 100+ entities
- Complex nested resolvers
- Advanced filtering, sorting, and pagination
- Batch loading to prevent N+1 queries
- Field-level permissions
- Optimized subscriptions

## Implementation Steps

### Step 1: Hasura Metadata Architecture
**Duration**: 3 days

1. **Metadata Organization Structure**
   ```yaml
   # hasura/metadata/databases/default/tables/tables.yaml
   - table:
       schema: public
       name: campaigns
   - table:
       schema: public
       name: characters
   - table:
       schema: public
       name: locations
   # ... 100+ table entries
   ```

2. **Table Configuration Template**
   ```yaml
   # hasura/metadata/databases/default/tables/public_characters.yaml
   table:
     schema: public
     name: characters
   
   object_relationships:
     - name: campaign
       using:
         foreign_key_constraint_on: campaign_id
     - name: race
       using:
         foreign_key_constraint_on: race_id
     - name: culture
       using:
         foreign_key_constraint_on: culture_id
     - name: current_location
       using:
         foreign_key_constraint_on: current_location_id
   
   array_relationships:
     - name: relationships_as_character1
       using:
         foreign_key_constraint_on:
           column: character_id_1
           table:
             schema: public
             name: character_relationships
     - name: organization_memberships
       using:
         foreign_key_constraint_on:
           column: character_id
           table:
             schema: public
             name: character_organization_memberships
   
   computed_fields:
     - name: full_relationships
       definition:
         function:
           schema: public
           name: character_all_relationships
   
   select_permissions:
     - role: public
       permission:
         columns:
           - id
           - name
           - character_type
           - core_identity
           - primary_motivation
         filter:
           deleted_at:
             _is_null: true
         allow_aggregations: true
   ```

3. **Custom SQL Functions**
   ```sql
   -- hasura/metadata/databases/default/functions/character_relationships.sql
   CREATE OR REPLACE FUNCTION character_all_relationships(character_row characters)
   RETURNS SETOF character_relationships AS $$
     SELECT cr.*
     FROM character_relationships cr
     WHERE (cr.character_id_1 = character_row.id OR cr.character_id_2 = character_row.id)
       AND cr.deleted_at IS NULL
   $$ LANGUAGE sql STABLE;
   
   CREATE OR REPLACE FUNCTION location_full_path(location_row locations)
   RETURNS TEXT AS $$
     WITH RECURSIVE location_path AS (
       SELECT id, name, parent_location_id, name::TEXT as path
       FROM locations
       WHERE id = location_row.id
       
       UNION ALL
       
       SELECT l.id, l.name, l.parent_location_id, 
              l.name || ' > ' || lp.path
       FROM locations l
       JOIN location_path lp ON l.id = lp.parent_location_id
     )
     SELECT path FROM location_path
     WHERE parent_location_id IS NULL
   $$ LANGUAGE sql STABLE;
   ```

### Step 2: Advanced Query Capabilities
**Duration**: 4 days

1. **Filtering System**
   ```graphql
   # Custom GraphQL schema extensions
   type Query {
     searchCharacters(
       campaign_id: Int!
       search_text: String
       character_types: [CharacterType!]
       race_ids: [Int!]
       culture_ids: [Int!]
       location_ids: [Int!]
       limit: Int
       offset: Int
       order_by: [CharacterOrderBy!]
     ): CharacterSearchResult!
     
     locationHierarchy(
       campaign_id: Int!
       root_location_id: Int
       max_depth: Int
     ): [LocationNode!]!
     
     pcBackstoryElements(
       campaign_id: Int!
       pc_id: Int!
       element_type: String
     ): [BackstoryElement!]!
     
     relationshipGraph(
       campaign_id: Int!
       center_character_id: Int
       degrees: Int
       relationship_types: [String!]
     ): RelationshipGraph!
   }
   
   type CharacterSearchResult {
     total_count: Int!
     items: [Character!]!
     facets: CharacterFacets!
   }
   
   type CharacterFacets {
     races: [FacetCount!]!
     cultures: [FacetCount!]!
     locations: [FacetCount!]!
     character_types: [FacetCount!]!
   }
   
   type FacetCount {
     value: String!
     count: Int!
   }
   ```

2. **Remote Schema for Complex Queries**
   ```typescript
   // backend/src/graphql/schema.rs
   use async_graphql::{Context, Object, Result};
   
   pub struct QueryRoot;
   
   #[Object]
   impl QueryRoot {
       async fn search_characters(
           &self,
           ctx: &Context<'_>,
           campaign_id: i32,
           search_text: Option<String>,
           character_types: Option<Vec<CharacterType>>,
           limit: Option<i32>,
           offset: Option<i32>,
       ) -> Result<CharacterSearchResult> {
           let pool = ctx.data::<PgPool>()?;
           
           let mut query = CharacterQueryBuilder::new()
               .base_query()
               .filter_campaign(campaign_id);
           
           if let Some(text) = search_text {
               query = query.search_text(&text);
           }
           
           if let Some(types) = character_types {
               query = query.filter_types(types);
           }
           
           let total_count = query.clone().count(pool).await?;
           let items = query
               .limit(limit.unwrap_or(20))
               .offset(offset.unwrap_or(0))
               .execute(pool)
               .await?;
           
           let facets = calculate_facets(pool, campaign_id).await?;
           
           Ok(CharacterSearchResult {
               total_count,
               items,
               facets,
           })
       }
       
       async fn location_hierarchy(
           &self,
           ctx: &Context<'_>,
           campaign_id: i32,
           root_location_id: Option<i32>,
           max_depth: Option<i32>,
       ) -> Result<Vec<LocationNode>> {
           let pool = ctx.data::<PgPool>()?;
           
           let hierarchy = LocationService::build_hierarchy(
               pool,
               campaign_id,
               root_location_id,
               max_depth.unwrap_or(10),
           ).await?;
           
           Ok(hierarchy)
       }
   }
   ```

### Step 3: Subscription Optimization
**Duration**: 3 days

1. **Granular Subscriptions**
   ```graphql
   type Subscription {
     # Campaign-level subscriptions
     campaignUpdates(campaign_id: Int!): CampaignUpdate!
     
     # Entity-specific subscriptions
     characterUpdates(
       campaign_id: Int!
       character_ids: [Int!]
       update_types: [UpdateType!]
     ): CharacterUpdate!
     
     locationUpdates(
       campaign_id: Int!
       location_ids: [Int!]
       include_children: Boolean
     ): LocationUpdate!
     
     # Generation progress with detailed info
     generationProgress(
       campaign_id: Int!
     ): GenerationProgress!
   }
   
   type GenerationProgress {
     campaign_id: Int!
     overall_progress: Float!
     current_phase: GenerationPhase!
     phase_progress: Float!
     entities_generated: EntityCounts!
     estimated_time_remaining: Int
     errors: [GenerationError!]
   }
   
   type EntityCounts {
     characters: Int!
     locations: Int!
     organizations: Int!
     backstory_elements: Int!
     items: Int!
   }
   ```

2. **Subscription Filtering**
   ```yaml
   # hasura/metadata/databases/default/tables/public_campaigns.yaml
   event_triggers:
     - name: campaign_updates
       definition:
         enable_manual: false
         insert:
           columns: "*"
         update:
           columns:
             - status
             - metadata
         delete:
           columns: "*"
       retry_conf:
         num_retries: 3
         interval_sec: 10
         timeout_sec: 60
       webhook: http://backend:3001/webhooks/campaign-updates
   ```

### Step 4: Performance Optimization
**Duration**: 3 days

1. **DataLoader Implementation**
   ```typescript
   // frontend/src/lib/dataloaders.ts
   import DataLoader from 'dataloader';
   import { urqlClient } from './urql';
   
   export const createCharacterLoader = () => {
     return new DataLoader<number, Character>(async (ids) => {
       const { data } = await urqlClient.query(
         gql`
           query BatchLoadCharacters($ids: [Int!]!) {
             characters(where: { id: { _in: $ids } }) {
               id
               name
               race { id name }
               culture { id name }
               current_location { id name }
             }
           }
         `,
         { ids }
       ).toPromise();
       
       const characterMap = new Map(
         data.characters.map(char => [char.id, char])
       );
       
       return ids.map(id => characterMap.get(id));
     });
   };
   
   export const createLocationLoader = () => {
     return new DataLoader<number, Location>(async (ids) => {
       // Similar implementation for locations
     });
   };
   ```

2. **Query Complexity Analysis**
   ```yaml
   # hasura/metadata/query_collections.yaml
   - name: restricted_queries
     definition:
       queries:
         - name: DeepNestedQuery
           query: |
             query DeepNestedQuery($campaign_id: Int!) {
               campaigns_by_pk(id: $campaign_id) {
                 characters(limit: 100) {
                   relationships {
                     other_character {
                       relationships {
                         other_character {
                           name
                         }
                       }
                     }
                   }
                 }
               }
             }
   ```

### Step 5: GraphQL Codegen Enhancement
**Duration**: 2 days

1. **Enhanced Codegen Configuration**
   ```yaml
   # frontend/codegen.yml
   overwrite: true
   schema: http://localhost:8080/v1/graphql
   documents:
     - src/**/*.graphql
     - src/**/*.tsx
   generates:
     src/generated/graphql.tsx:
       plugins:
         - typescript
         - typescript-operations
         - typescript-urql
       config:
         withHooks: true
         withComponent: false
         withHOC: false
         avoidOptionals: false
         scalars:
           timestamptz: string
           jsonb: any
           uuid: string
         preResolveTypes: true
         namingConvention:
           typeNames: pascal-case#pascalCase
           enumValues: upper-case#upperCase
     src/generated/introspection.json:
       plugins:
         - introspection
       config:
         minify: true
   ```

2. **Custom Scalar Handling**
   ```typescript
   // frontend/src/lib/scalars.ts
   import { Kind, GraphQLScalarType } from 'graphql';
   
   export const DateTimeScalar = new GraphQLScalarType({
     name: 'DateTime',
     description: 'DateTime custom scalar type',
     serialize(value) {
       return value instanceof Date ? value.toISOString() : value;
     },
     parseValue(value) {
       return new Date(value);
     },
     parseLiteral(ast) {
       if (ast.kind === Kind.STRING) {
         return new Date(ast.value);
       }
       return null;
     },
   });
   
   export const JSONBScalar = new GraphQLScalarType({
     name: 'JSONB',
     description: 'JSONB custom scalar type',
     serialize(value) {
       return value;
     },
     parseValue(value) {
       return value;
     },
     parseLiteral(ast) {
       if (ast.kind === Kind.OBJECT) {
         return parseJSONLiteral(ast);
       }
       return null;
     },
   });
   ```

## Testing Strategy

1. **Query Performance Testing**
   - Complex nested query benchmarks
   - N+1 query detection
   - Subscription latency testing

2. **Schema Validation**
   - Breaking change detection
   - Type safety verification
   - Permission testing

3. **Load Testing**
   - Concurrent query handling
   - Subscription scalability
   - Cache effectiveness

## Migration Path

1. **Incremental Schema Updates**
   - Add new types without breaking existing
   - Deprecate old fields gradually
   - Version schema changes

2. **Client Migration**
   - Update queries incrementally
   - Test with both old and new schemas
   - Monitor for performance regressions

## Success Metrics

- Query response time <200ms for 95th percentile
- Zero N+1 queries in production
- Subscription latency <50ms
- Schema introspection <1s
- 100% type coverage in frontend

## Next Phase
Proceed to [UPGRADE_PLAN_P4.md](./UPGRADE_PLAN_P4.md) for Anthropic API Enhancement.