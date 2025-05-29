# UPGRADE PLAN PHASE 2: BACKEND MODEL GENERATION & DATA LAYER

## Overview
This phase focuses on expanding the Rust backend to support 100+ database tables with proper model generation, relationship handling, and efficient data operations, all centered around creating a lived-in world from player character backstories.

## Current State
- Simple models for 4 entities
- Basic CRUD operations
- Manual model definitions
- Simple JSON serialization

## Target State
- Automated model generation for 100+ entities
- Complex relationship handling
- Batch operations for performance
- Trait-based common operations
- Hierarchical data structure support

## Implementation Steps

### Step 1: Model Generation Infrastructure
**Duration**: 3 days

1. **SQLx Model Generation Setup**
   ```toml
   # Cargo.toml additions
   [build-dependencies]
   sqlx = { version = "0.8", features = ["postgres", "offline"] }
   
   [dependencies]
   async-trait = "0.1"
   itertools = "0.12"
   rayon = "1.8"  # For parallel processing
   ```

2. **Create Model Traits**
   ```rust
   // src/models/traits.rs
   use async_trait::async_trait;
   use sqlx::{PgPool, Result};
   use chrono::{DateTime, Utc};
   
   #[async_trait]
   pub trait SoftDeletable {
       async fn soft_delete(&mut self, pool: &PgPool) -> Result<()>;
       async fn restore(&mut self, pool: &PgPool) -> Result<()>;
       fn is_deleted(&self) -> bool;
   }
   
   #[async_trait]
   pub trait Timestamped {
       fn created_at(&self) -> DateTime<Utc>;
       fn updated_at(&self) -> DateTime<Utc>;
       async fn touch(&mut self, pool: &PgPool) -> Result<()>;
   }
   
   #[async_trait]
   pub trait Hierarchical<T> {
       async fn parent(&self, pool: &PgPool) -> Result<Option<T>>;
       async fn children(&self, pool: &PgPool) -> Result<Vec<T>>;
       async fn ancestors(&self, pool: &PgPool) -> Result<Vec<T>>;
       async fn descendants(&self, pool: &PgPool) -> Result<Vec<T>>;
   }
   ```

3. **Model Generation Script**
   ```rust
   // build.rs
   use std::env;
   use std::path::Path;
   
   fn main() {
       // Generate models from database schema
       if env::var("SKIP_MODEL_GEN").is_err() {
           println!("cargo:rerun-if-changed=migrations/");
           generate_models();
       }
   }
   
   fn generate_models() {
       // Read schema and generate Rust structs
       // Implementation would query information_schema
   }
   ```

### Step 2: Core Entity Models
**Duration**: 4 days

1. **Character System Models**
   ```rust
   // src/models/character.rs
   use serde::{Serialize, Deserialize};
   use sqlx::FromRow;
   use chrono::{DateTime, Utc};
   
   #[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
   pub struct Character {
       pub id: i32,
       pub campaign_id: i32,
       pub character_type: CharacterType,
       pub name: String,
       pub race_id: Option<i32>,
       pub culture_id: Option<i32>,
       
       // 5-bullet method
       pub core_identity: Option<String>,
       pub primary_motivation: Option<String>,
       pub distinctive_quirk: Option<String>,
       pub current_situation: Option<String>,
       pub hidden_information: Option<String>,
       
       // Extended fields
       pub personality_traits: serde_json::Value,
       pub ideals: serde_json::Value,
       pub bonds: serde_json::Value,
       pub flaws: serde_json::Value,
       
       // PC connection fields
       pub backstory_summary: Option<String>,
       pub connected_to_pc_id: Option<i32>,
       pub connection_type: Option<String>,
       
       // Metadata
       pub created_at: DateTime<Utc>,
       pub updated_at: DateTime<Utc>,
       pub deleted_at: Option<DateTime<Utc>>,
   }
   
   #[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
   #[sqlx(type_name = "character_type", rename_all = "lowercase")]
   pub enum CharacterType {
       PC,
       NPC,
   }
   
   #[derive(Debug, Clone, Serialize, Deserialize)]
   pub struct CharacterWithRelations {
       #[serde(flatten)]
       pub character: Character,
       pub race: Option<Race>,
       pub culture: Option<Culture>,
       pub relationships: Vec<CharacterRelationship>,
       pub organization_memberships: Vec<OrganizationMembership>,
       pub current_location: Option<Location>,
       pub pc_backstory_elements: Vec<BackstoryElement>,
   }
   ```

2. **Location Hierarchy Models**
   ```rust
   // src/models/location.rs
   #[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
   pub struct Location {
       pub id: i32,
       pub campaign_id: i32,
       pub parent_location_id: Option<i32>,
       pub location_type_id: i32,
       pub name: String,
       pub description: Option<String>,
       pub population: Option<i32>,
       pub danger_level: Option<i32>,
       pub map_coordinates: Option<serde_json::Value>,
       
       // Computed fields
       #[sqlx(skip)]
       pub depth: Option<i32>,
       #[sqlx(skip)]
       pub path: Option<Vec<String>>,
   }
   
   impl Location {
       pub async fn with_hierarchy(&self, pool: &PgPool) -> Result<LocationWithHierarchy> {
           let children = self.get_children(pool).await?;
           let parent = self.get_parent(pool).await?;
           let ancestors = self.get_ancestors(pool).await?;
           
           Ok(LocationWithHierarchy {
               location: self.clone(),
               parent,
               children,
               ancestors,
           })
       }
   }
   ```

3. **Organization Models**
   ```rust
   // src/models/organization.rs
   #[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
   pub struct Organization {
       pub id: i32,
       pub campaign_id: i32,
       pub name: String,
       pub organization_type: Option<String>,
       pub goals: serde_json::Value,
       pub headquarters_location_id: Option<i32>,
       pub deleted_at: Option<DateTime<Utc>>,
   }
   
   #[derive(Debug, Clone, Serialize, Deserialize)]
   pub struct OrganizationWithMembers {
       #[serde(flatten)]
       pub organization: Organization,
       pub ranks: Vec<OrganizationRank>,
       pub members: Vec<CharacterMembership>,
       pub headquarters: Option<Location>,
   }
   ```

### Step 3: Relationship Management
**Duration**: 3 days

1. **Many-to-Many Handling**
   ```rust
   // src/models/relationships.rs
   use std::collections::HashMap;
   
   #[derive(Debug, Clone)]
   pub struct RelationshipGraph<T> {
       nodes: HashMap<i32, T>,
       edges: HashMap<i32, Vec<Edge>>,
   }
   
   #[derive(Debug, Clone)]
   pub struct Edge {
       pub from_id: i32,
       pub to_id: i32,
       pub relationship_type: String,
       pub metadata: serde_json::Value,
   }
   
   impl<T> RelationshipGraph<T> {
       pub fn new() -> Self {
           Self {
               nodes: HashMap::new(),
               edges: HashMap::new(),
           }
       }
       
       pub fn add_node(&mut self, id: i32, node: T) {
           self.nodes.insert(id, node);
       }
       
       pub fn add_edge(&mut self, edge: Edge) {
           self.edges
               .entry(edge.from_id)
               .or_insert_with(Vec::new)
               .push(edge);
       }
       
       pub fn get_connected(&self, id: i32) -> Vec<&T> {
           self.edges
               .get(&id)
               .map(|edges| {
                   edges.iter()
                       .filter_map(|e| self.nodes.get(&e.to_id))
                       .collect()
               })
               .unwrap_or_default()
       }
   }
   ```

2. **Batch Operations**
   ```rust
   // src/services/batch_operations.rs
   use futures::future::try_join_all;
   use rayon::prelude::*;
   
   pub struct BatchOperations {
       pool: PgPool,
       batch_size: usize,
   }
   
   impl BatchOperations {
       pub async fn insert_characters(&self, characters: Vec<Character>) -> Result<Vec<i32>> {
           let chunks: Vec<_> = characters
               .chunks(self.batch_size)
               .map(|chunk| chunk.to_vec())
               .collect();
           
           let futures = chunks.into_iter().map(|chunk| {
               let pool = self.pool.clone();
               async move {
                   let mut tx = pool.begin().await?;
                   let mut ids = Vec::new();
                   
                   for character in chunk {
                       let id = sqlx::query_scalar!(
                           r#"
                           INSERT INTO characters (campaign_id, name, character_type)
                           VALUES ($1, $2, $3)
                           RETURNING id
                           "#,
                           character.campaign_id,
                           character.name,
                           character.character_type as _
                       )
                       .fetch_one(&mut *tx)
                       .await?;
                       
                       ids.push(id);
                   }
                   
                   tx.commit().await?;
                   Ok::<Vec<i32>, sqlx::Error>(ids)
               }
           });
           
           let results = try_join_all(futures).await?;
           Ok(results.into_iter().flatten().collect())
       }
   }
   ```

### Step 4: Query Builders
**Duration**: 3 days

1. **Dynamic Query Construction**
   ```rust
   // src/services/query_builder.rs
   use sqlx::QueryBuilder;
   
   pub struct CharacterQueryBuilder<'a> {
       builder: QueryBuilder<'a, Postgres>,
       base_applied: bool,
   }
   
   impl<'a> CharacterQueryBuilder<'a> {
       pub fn new() -> Self {
           let mut builder = QueryBuilder::new("");
           Self {
               builder,
               base_applied: false,
           }
       }
       
       pub fn base_query(mut self) -> Self {
           self.builder.push(r#"
               SELECT c.*, r.name as race_name, cult.name as culture_name
               FROM characters c
               LEFT JOIN races r ON c.race_id = r.id
               LEFT JOIN cultures cult ON c.culture_id = cult.id
               WHERE c.deleted_at IS NULL
           "#);
           self.base_applied = true;
           self
       }
       
       pub fn filter_campaign(mut self, campaign_id: i32) -> Self {
           let conjunction = if self.base_applied { " AND " } else { " WHERE " };
           self.builder.push(conjunction);
           self.builder.push("c.campaign_id = ");
           self.builder.push_bind(campaign_id);
           self
       }
       
       pub fn filter_type(mut self, character_type: CharacterType) -> Self {
           self.builder.push(" AND c.character_type = ");
           self.builder.push_bind(character_type);
           self
       }
       
       pub fn with_relationships(mut self) -> Self {
           // Add relationship joins
           self
       }
       
       pub fn build(self) -> sqlx::query::Query<'a, Postgres, PgArguments> {
           self.builder.build()
       }
   }
   ```

### Step 5: Performance Optimization
**Duration**: 2 days

1. **Connection Pooling**
   ```rust
   // src/db/pool.rs
   use sqlx::postgres::{PgPool, PgPoolOptions};
   use std::time::Duration;
   
   pub async fn create_pool(database_url: &str) -> Result<PgPool> {
       let pool = PgPoolOptions::new()
           .max_connections(32)
           .min_connections(5)
           .connect_timeout(Duration::from_secs(8))
           .idle_timeout(Duration::from_secs(600))
           .max_lifetime(Duration::from_secs(1800))
           .connect(database_url)
           .await?;
       
       Ok(pool)
   }
   ```

2. **Query Caching**
   ```rust
   // src/services/cache.rs
   use std::sync::Arc;
   use tokio::sync::RwLock;
   use std::collections::HashMap;
   use std::time::{Duration, Instant};
   
   pub struct QueryCache<K, V> {
       cache: Arc<RwLock<HashMap<K, CachedValue<V>>>>,
       ttl: Duration,
   }
   
   struct CachedValue<V> {
       value: V,
       expires_at: Instant,
   }
   
   impl<K: Eq + std::hash::Hash + Clone, V: Clone> QueryCache<K, V> {
       pub async fn get_or_fetch<F, Fut>(&self, key: K, fetch: F) -> Result<V>
       where
           F: FnOnce() -> Fut,
           Fut: std::future::Future<Output = Result<V>>,
       {
           // Check cache first
           {
               let cache = self.cache.read().await;
               if let Some(cached) = cache.get(&key) {
                   if cached.expires_at > Instant::now() {
                       return Ok(cached.value.clone());
                   }
               }
           }
           
           // Fetch if not cached
           let value = fetch().await?;
           
           // Update cache
           {
               let mut cache = self.cache.write().await;
               cache.insert(key, CachedValue {
                   value: value.clone(),
                   expires_at: Instant::now() + self.ttl,
               });
           }
           
           Ok(value)
       }
   }
   ```

## Testing Strategy

1. **Model Testing**
   - Unit tests for all model methods
   - Integration tests with test database
   - Performance benchmarks

2. **Relationship Testing**
   - Graph traversal tests
   - Circular reference handling
   - Orphan detection

3. **Batch Operation Testing**
   - Large dataset insertion
   - Transaction rollback scenarios
   - Concurrent operation handling

## Migration Path

1. **Gradual Model Migration**
   - Keep old models functional
   - Add new models alongside
   - Migrate endpoints incrementally

2. **Database Compatibility**
   - Test with both old and new schemas
   - Ensure no breaking changes
   - Performance comparison

## Success Metrics

- Model generation completes in <5 seconds
- Batch insertions handle 10k+ records efficiently
- Query performance improves by 50%+
- Memory usage remains stable
- All relationships properly typed

## Next Phase
Proceed to [UPGRADE_PLAN_P3.md](./UPGRADE_PLAN_P3.md) for GraphQL Schema Evolution.