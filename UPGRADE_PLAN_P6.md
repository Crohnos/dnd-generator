# UPGRADE PLAN PHASE 6: STATE MANAGEMENT SCALING

## Overview
This phase focuses on evolving the state management from a single Zustand store to a sophisticated multi-store architecture supporting 100+ entity types with normalized data, optimistic updates, and real-time synchronization. The state management is designed to efficiently handle the complex relationships between PC backstories and world elements.

## Current State
- Single Zustand store for campaign wizard
- Basic form state management
- No normalization
- Limited caching

## Target State
- Domain-specific stores for each entity type
- Normalized state with entity relationships
- Optimistic updates with rollback
- Real-time synchronization across stores
- Undo/redo functionality
- Selective subscriptions for performance

## Implementation Steps

### Step 1: Normalized State Architecture
**Duration**: 4 days

1. **Entity Store Factory**
   ```typescript
   // src/stores/factory/entityStore.ts
   import { create } from 'zustand';
   import { immer } from 'zustand/middleware/immer';
   import { devtools, subscribeWithSelector } from 'zustand/middleware';
   import { normalize, denormalize, schema } from 'normalizr';
   
   export interface EntityState<T> {
     byId: Record<string, T>;
     allIds: string[];
     loading: Record<string, boolean>;
     errors: Record<string, Error | null>;
   }
   
   export interface EntityActions<T> {
     // CRUD operations
     setOne: (entity: T) => void;
     setMany: (entities: T[]) => void;
     updateOne: (id: string, updates: Partial<T>) => void;
     updateMany: (updates: Array<{ id: string; changes: Partial<T> }>) => void;
     removeOne: (id: string) => void;
     removeMany: (ids: string[]) => void;
     
     // Bulk operations
     upsertOne: (entity: T) => void;
     upsertMany: (entities: T[]) => void;
     
     // Query operations
     selectById: (id: string) => T | undefined;
     selectByIds: (ids: string[]) => T[];
     selectAll: () => T[];
     selectWhere: (predicate: (entity: T) => boolean) => T[];
     
     // Loading states
     setLoading: (id: string, loading: boolean) => void;
     setError: (id: string, error: Error | null) => void;
     
     // Utilities
     clear: () => void;
     reset: () => void;
   }
   
   export function createEntityStore<T extends { id: string | number }>(
     name: string,
     schema: schema.Entity
   ) {
     type State = EntityState<T>;
     type Actions = EntityActions<T>;
     
     const initialState: State = {
       byId: {},
       allIds: [],
       loading: {},
       errors: {},
     };
     
     return create<State & Actions>()(
       devtools(
         subscribeWithSelector(
           immer((set, get) => ({
             ...initialState,
             
             setOne: (entity) => set((state) => {
               const id = String(entity.id);
               state.byId[id] = entity;
               if (!state.allIds.includes(id)) {
                 state.allIds.push(id);
               }
             }),
             
             setMany: (entities) => set((state) => {
               entities.forEach(entity => {
                 const id = String(entity.id);
                 state.byId[id] = entity;
                 if (!state.allIds.includes(id)) {
                   state.allIds.push(id);
                 }
               });
             }),
             
             updateOne: (id, updates) => set((state) => {
               if (state.byId[id]) {
                 Object.assign(state.byId[id], updates);
               }
             }),
             
             updateMany: (updates) => set((state) => {
               updates.forEach(({ id, changes }) => {
                 if (state.byId[id]) {
                   Object.assign(state.byId[id], changes);
                 }
               });
             }),
             
             removeOne: (id) => set((state) => {
               delete state.byId[id];
               state.allIds = state.allIds.filter(i => i !== id);
             }),
             
             removeMany: (ids) => set((state) => {
               ids.forEach(id => {
                 delete state.byId[id];
               });
               state.allIds = state.allIds.filter(id => !ids.includes(id));
             }),
             
             upsertOne: (entity) => {
               const { setOne } = get();
               setOne(entity);
             },
             
             upsertMany: (entities) => {
               const { setMany } = get();
               setMany(entities);
             },
             
             selectById: (id) => get().byId[id],
             
             selectByIds: (ids) => {
               const { byId } = get();
               return ids.map(id => byId[id]).filter(Boolean);
             },
             
             selectAll: () => {
               const { byId, allIds } = get();
               return allIds.map(id => byId[id]);
             },
             
             selectWhere: (predicate) => {
               const { selectAll } = get();
               return selectAll().filter(predicate);
             },
             
             setLoading: (id, loading) => set((state) => {
               state.loading[id] = loading;
             }),
             
             setError: (id, error) => set((state) => {
               state.errors[id] = error;
             }),
             
             clear: () => set(() => initialState),
             
             reset: () => set(() => initialState),
           }))
         ),
         { name: `${name}-store` }
       )
     );
   }
   ```

2. **Schema Definitions**
   ```typescript
   // src/stores/schemas.ts
   import { schema } from 'normalizr';
   
   // Entity schemas
   export const raceSchema = new schema.Entity('races');
   export const cultureSchema = new schema.Entity('cultures');
   export const languageSchema = new schema.Entity('languages');
   export const deitySchema = new schema.Entity('deities');
   export const organizationSchema = new schema.Entity('organizations');
   export const locationSchema = new schema.Entity('locations');
   
   // Character schema with relationships
   export const characterSchema = new schema.Entity('characters', {
     race: raceSchema,
     culture: cultureSchema,
     current_location: locationSchema,
     organization_memberships: [organizationSchema],
   });
   
   // Location schema with nested relationships
   locationSchema.define({
     parent_location: locationSchema,
     child_locations: [locationSchema],
     npcs: [characterSchema],
   });
   
   // Campaign schema with all relationships
   export const campaignSchema = new schema.Entity('campaigns', {
     characters: [characterSchema],
     locations: [locationSchema],
     races: [raceSchema],
     cultures: [cultureSchema],
     deities: [deitySchema],
     organizations: [organizationSchema],
   });
   
   // Relationship schemas
   export const characterRelationshipSchema = new schema.Entity('character_relationships', {
     character1: characterSchema,
     character2: characterSchema,
   });
   
   export const locationRelationshipSchema = new schema.Entity('location_relationships', {
     location1: locationSchema,
     location2: locationSchema,
   });
   ```

3. **Store Registry**
   ```typescript
   // src/stores/registry.ts
   import { createEntityStore } from './factory/entityStore';
   import * as schemas from './schemas';
   
   // Create all entity stores
   export const stores = {
     campaigns: createEntityStore('campaigns', schemas.campaignSchema),
     characters: createEntityStore('characters', schemas.characterSchema),
     locations: createEntityStore('locations', schemas.locationSchema),
     races: createEntityStore('races', schemas.raceSchema),
     cultures: createEntityStore('cultures', schemas.cultureSchema),
     languages: createEntityStore('languages', schemas.languageSchema),
     deities: createEntityStore('deities', schemas.deitySchema),
     organizations: createEntityStore('organizations', schemas.organizationSchema),
     items: createEntityStore('items', schemas.itemSchema),
     backstoryElements: createEntityStore('backstoryElements', schemas.backstoryElementSchema),
     pcConnections: createEntityStore('pcConnections', schemas.pcConnectionSchema),
   };
   
   // Global store registry for dynamic access
   export class StoreRegistry {
     private static instance: StoreRegistry;
     private stores: Map<string, any> = new Map();
     
     private constructor() {
       Object.entries(stores).forEach(([key, store]) => {
         this.stores.set(key, store);
       });
     }
     
     static getInstance(): StoreRegistry {
       if (!StoreRegistry.instance) {
         StoreRegistry.instance = new StoreRegistry();
       }
       return StoreRegistry.instance;
     }
     
     getStore<T>(entityType: string): T {
       const store = this.stores.get(entityType);
       if (!store) {
         throw new Error(`Store for entity type "${entityType}" not found`);
       }
       return store;
     }
     
     getAllStores(): Map<string, any> {
       return new Map(this.stores);
     }
   }
   ```

### Step 2: Relationship Management
**Duration**: 3 days

1. **Relationship Store**
   ```typescript
   // src/stores/relationshipStore.ts
   import { create } from 'zustand';
   import { immer } from 'zustand/middleware/immer';
   
   interface RelationshipState {
     // Entity to entity relationships
     characterRelationships: Map<string, Set<string>>;
     locationHierarchy: Map<string, { parent?: string; children: Set<string> }>;
     organizationMembers: Map<string, Set<string>>;
     itemOwnership: Map<string, { type: 'character' | 'location'; id: string }>;
     
     // Cross-entity relationships
     characterLocations: Map<string, string>;
     questRelatedEntities: Map<string, { npcs: Set<string>; locations: Set<string> }>;
   }
   
   interface RelationshipActions {
     // Character relationships
     addCharacterRelationship: (char1: string, char2: string) => void;
     removeCharacterRelationship: (char1: string, char2: string) => void;
     getCharacterRelationships: (charId: string) => string[];
     
     // Location hierarchy
     setLocationParent: (locationId: string, parentId: string | null) => void;
     addLocationChild: (parentId: string, childId: string) => void;
     getLocationHierarchy: (locationId: string) => { parent?: string; children: string[] };
     getLocationAncestors: (locationId: string) => string[];
     getLocationDescendants: (locationId: string) => string[];
     
     // Organization memberships
     addOrganizationMember: (orgId: string, memberId: string) => void;
     removeOrganizationMember: (orgId: string, memberId: string) => void;
     getOrganizationMembers: (orgId: string) => string[];
     getCharacterOrganizations: (charId: string) => string[];
     
     // Complex queries
     getRelatedEntities: (entityId: string, entityType: string, depth?: number) => Map<string, Set<string>>;
     findShortestPath: (entity1: string, entity2: string, entityType: string) => string[] | null;
   }
   
   export const useRelationshipStore = create<RelationshipState & RelationshipActions>()(
     immer((set, get) => ({
       characterRelationships: new Map(),
       locationHierarchy: new Map(),
       organizationMembers: new Map(),
       itemOwnership: new Map(),
       characterLocations: new Map(),
       questRelatedEntities: new Map(),
       
       addCharacterRelationship: (char1, char2) => set((state) => {
         // Ensure bidirectional relationship
         if (!state.characterRelationships.has(char1)) {
           state.characterRelationships.set(char1, new Set());
         }
         if (!state.characterRelationships.has(char2)) {
           state.characterRelationships.set(char2, new Set());
         }
         state.characterRelationships.get(char1)!.add(char2);
         state.characterRelationships.get(char2)!.add(char1);
       }),
       
       removeCharacterRelationship: (char1, char2) => set((state) => {
         state.characterRelationships.get(char1)?.delete(char2);
         state.characterRelationships.get(char2)?.delete(char1);
       }),
       
       getCharacterRelationships: (charId) => {
         const relationships = get().characterRelationships.get(charId);
         return relationships ? Array.from(relationships) : [];
       },
       
       setLocationParent: (locationId, parentId) => set((state) => {
         const current = state.locationHierarchy.get(locationId);
         
         // Remove from old parent's children
         if (current?.parent) {
           state.locationHierarchy.get(current.parent)?.children.delete(locationId);
         }
         
         // Set new parent
         if (!state.locationHierarchy.has(locationId)) {
           state.locationHierarchy.set(locationId, { children: new Set() });
         }
         
         if (parentId) {
           state.locationHierarchy.get(locationId)!.parent = parentId;
           
           // Add to new parent's children
           if (!state.locationHierarchy.has(parentId)) {
             state.locationHierarchy.set(parentId, { children: new Set() });
           }
           state.locationHierarchy.get(parentId)!.children.add(locationId);
         } else {
           delete state.locationHierarchy.get(locationId)!.parent;
         }
       }),
       
       getLocationAncestors: (locationId) => {
         const ancestors: string[] = [];
         let current = locationId;
         const hierarchy = get().locationHierarchy;
         
         while (hierarchy.get(current)?.parent) {
           const parent = hierarchy.get(current)!.parent!;
           ancestors.push(parent);
           current = parent;
           
           // Prevent infinite loops
           if (ancestors.length > 100) break;
         }
         
         return ancestors;
       },
       
       getLocationDescendants: (locationId) => {
         const descendants: string[] = [];
         const toVisit = [locationId];
         const hierarchy = get().locationHierarchy;
         
         while (toVisit.length > 0) {
           const current = toVisit.pop()!;
           const children = hierarchy.get(current)?.children;
           
           if (children) {
             const childArray = Array.from(children);
             descendants.push(...childArray);
             toVisit.push(...childArray);
           }
         }
         
         return descendants;
       },
       
       getRelatedEntities: (entityId, entityType, depth = 2) => {
         const visited = new Set<string>();
         const related = new Map<string, Set<string>>();
         const queue: Array<{ id: string; type: string; level: number }> = [
           { id: entityId, type: entityType, level: 0 }
         ];
         
         while (queue.length > 0) {
           const { id, type, level } = queue.shift()!;
           
           if (visited.has(`${type}:${id}`) || level > depth) continue;
           visited.add(`${type}:${id}`);
           
           // Get related entities based on type
           if (type === 'character') {
             const relationships = get().getCharacterRelationships(id);
             relationships.forEach(relId => {
               if (!related.has('characters')) related.set('characters', new Set());
               related.get('characters')!.add(relId);
               queue.push({ id: relId, type: 'character', level: level + 1 });
             });
             
             // Add location
             const location = get().characterLocations.get(id);
             if (location) {
               if (!related.has('locations')) related.set('locations', new Set());
               related.get('locations')!.add(location);
               queue.push({ id: location, type: 'location', level: level + 1 });
             }
           }
           
           // Similar logic for other entity types...
         }
         
         return related;
       },
       
       findShortestPath: (entity1, entity2, entityType) => {
         // BFS implementation for finding shortest path
         const queue: Array<{ id: string; path: string[] }> = [
           { id: entity1, path: [entity1] }
         ];
         const visited = new Set<string>();
         
         while (queue.length > 0) {
           const { id, path } = queue.shift()!;
           
           if (id === entity2) {
             return path;
           }
           
           if (visited.has(id)) continue;
           visited.add(id);
           
           // Get neighbors based on entity type
           let neighbors: string[] = [];
           if (entityType === 'character') {
             neighbors = get().getCharacterRelationships(id);
           }
           // Add other entity types...
           
           neighbors.forEach(neighbor => {
             if (!visited.has(neighbor)) {
               queue.push({ id: neighbor, path: [...path, neighbor] });
             }
           });
         }
         
         return null;
       },
     }))
   );
   ```

### Step 3: Optimistic Updates
**Duration**: 3 days

1. **Optimistic Update Manager**
   ```typescript
   // src/stores/optimistic/optimisticManager.ts
   import { v4 as uuidv4 } from 'uuid';
   
   interface OptimisticUpdate {
     id: string;
     timestamp: Date;
     entityType: string;
     entityId: string;
     operation: 'create' | 'update' | 'delete';
     previousState: any;
     optimisticState: any;
     status: 'pending' | 'confirmed' | 'failed';
   }
   
   export class OptimisticUpdateManager {
     private updates: Map<string, OptimisticUpdate> = new Map();
     private rollbackHandlers: Map<string, (update: OptimisticUpdate) => void> = new Map();
     
     registerUpdate(
       entityType: string,
       entityId: string,
       operation: OptimisticUpdate['operation'],
       previousState: any,
       optimisticState: any,
       rollbackHandler: (update: OptimisticUpdate) => void
     ): string {
       const updateId = uuidv4();
       const update: OptimisticUpdate = {
         id: updateId,
         timestamp: new Date(),
         entityType,
         entityId,
         operation,
         previousState,
         optimisticState,
         status: 'pending',
       };
       
       this.updates.set(updateId, update);
       this.rollbackHandlers.set(updateId, rollbackHandler);
       
       // Auto-cleanup after 5 minutes
       setTimeout(() => {
         if (this.updates.get(updateId)?.status === 'pending') {
           this.rollback(updateId);
         }
       }, 5 * 60 * 1000);
       
       return updateId;
     }
     
     confirm(updateId: string) {
       const update = this.updates.get(updateId);
       if (update) {
         update.status = 'confirmed';
         this.cleanup(updateId);
       }
     }
     
     rollback(updateId: string) {
       const update = this.updates.get(updateId);
       const rollbackHandler = this.rollbackHandlers.get(updateId);
       
       if (update && rollbackHandler) {
         update.status = 'failed';
         rollbackHandler(update);
         this.cleanup(updateId);
       }
     }
     
     private cleanup(updateId: string) {
       setTimeout(() => {
         this.updates.delete(updateId);
         this.rollbackHandlers.delete(updateId);
       }, 1000);
     }
     
     getPendingUpdates(): OptimisticUpdate[] {
       return Array.from(this.updates.values())
         .filter(u => u.status === 'pending')
         .sort((a, b) => a.timestamp.getTime() - b.timestamp.getTime());
     }
   }
   
   // Global instance
   export const optimisticManager = new OptimisticUpdateManager();
   ```

2. **Optimistic Store Wrapper**
   ```typescript
   // src/stores/optimistic/createOptimisticStore.ts
   import { StateCreator } from 'zustand';
   import { optimisticManager } from './optimisticManager';
   
   export function withOptimisticUpdates<T extends object>(
     storeCreator: StateCreator<T>
   ): StateCreator<T & OptimisticActions> {
     return (set, get, api) => ({
       ...storeCreator(set, get, api),
       
       optimisticUpdate: async (
         entityId: string,
         updates: Partial<any>,
         serverUpdate: () => Promise<any>
       ) => {
         const store = get() as any;
         const entity = store.selectById(entityId);
         
         if (!entity) {
           throw new Error(`Entity ${entityId} not found`);
         }
         
         // Save current state
         const previousState = { ...entity };
         
         // Apply optimistic update
         store.updateOne(entityId, updates);
         
         // Register with manager
         const updateId = optimisticManager.registerUpdate(
           api.getState().name || 'unknown',
           entityId,
           'update',
           previousState,
           { ...entity, ...updates },
           (update) => {
             // Rollback handler
             store.updateOne(entityId, update.previousState);
           }
         );
         
         try {
           // Perform server update
           const result = await serverUpdate();
           
           // Confirm optimistic update
           optimisticManager.confirm(updateId);
           
           // Update with server response
           if (result) {
             store.updateOne(entityId, result);
           }
           
           return result;
         } catch (error) {
           // Rollback on error
           optimisticManager.rollback(updateId);
           throw error;
         }
       },
       
       optimisticCreate: async (
         entity: any,
         serverCreate: () => Promise<any>
       ) => {
         const store = get() as any;
         const tempId = `temp_${Date.now()}`;
         const entityWithTempId = { ...entity, id: tempId };
         
         // Add to store optimistically
         store.setOne(entityWithTempId);
         
         // Register with manager
         const updateId = optimisticManager.registerUpdate(
           api.getState().name || 'unknown',
           tempId,
           'create',
           null,
           entityWithTempId,
           () => {
             // Rollback handler
             store.removeOne(tempId);
           }
         );
         
         try {
           // Perform server create
           const result = await serverCreate();
           
           // Replace temp entity with real one
           store.removeOne(tempId);
           store.setOne(result);
           
           // Confirm optimistic update
           optimisticManager.confirm(updateId);
           
           return result;
         } catch (error) {
           // Rollback automatically handled by manager
           optimisticManager.rollback(updateId);
           throw error;
         }
       },
       
       optimisticDelete: async (
         entityId: string,
         serverDelete: () => Promise<void>
       ) => {
         const store = get() as any;
         const entity = store.selectById(entityId);
         
         if (!entity) {
           throw new Error(`Entity ${entityId} not found`);
         }
         
         // Save current state
         const previousState = { ...entity };
         
         // Remove optimistically
         store.removeOne(entityId);
         
         // Register with manager
         const updateId = optimisticManager.registerUpdate(
           api.getState().name || 'unknown',
           entityId,
           'delete',
           previousState,
           null,
           () => {
             // Rollback handler
             store.setOne(previousState);
           }
         );
         
         try {
           // Perform server delete
           await serverDelete();
           
           // Confirm optimistic update
           optimisticManager.confirm(updateId);
         } catch (error) {
           // Rollback on error
           optimisticManager.rollback(updateId);
           throw error;
         }
       },
     });
   }
   ```

### Step 4: Real-time Synchronization
**Duration**: 3 days

1. **Sync Manager**
   ```typescript
   // src/stores/sync/syncManager.ts
   import { useSubscription } from 'urql';
   import { useEffect } from 'react';
   import { StoreRegistry } from '../registry';
   
   interface SyncUpdate {
     entityType: string;
     operation: 'create' | 'update' | 'delete';
     entityId: string;
     data?: any;
     timestamp: string;
   }
   
   const SYNC_SUBSCRIPTION = `
     subscription CampaignSync($campaignId: Int!) {
       campaign_sync(campaign_id: $campaignId) {
         entityType
         operation
         entityId
         data
         timestamp
       }
     }
   `;
   
   export function useCampaignSync(campaignId: number) {
     const registry = StoreRegistry.getInstance();
     const [result] = useSubscription({
       query: SYNC_SUBSCRIPTION,
       variables: { campaignId },
     });
     
     useEffect(() => {
       if (result.data?.campaign_sync) {
         const update: SyncUpdate = result.data.campaign_sync;
         const store = registry.getStore(update.entityType);
         
         if (!store) {
           console.warn(`No store found for entity type: ${update.entityType}`);
           return;
         }
         
         switch (update.operation) {
           case 'create':
             store.upsertOne(update.data);
             break;
           case 'update':
             store.updateOne(update.entityId, update.data);
             break;
           case 'delete':
             store.removeOne(update.entityId);
             break;
         }
       }
     }, [result.data]);
     
     return {
       connected: !result.error && !result.fetching,
       error: result.error,
     };
   }
   
   // Batch sync for initial load
   export async function syncCampaignData(
     campaignId: number,
     client: any
   ): Promise<void> {
     const registry = StoreRegistry.getInstance();
     
     // Fetch all campaign data
     const { data } = await client.query(GET_FULL_CAMPAIGN, {
       id: campaignId,
     }).toPromise();
     
     if (!data?.campaigns_by_pk) {
       throw new Error('Campaign not found');
     }
     
     const campaign = data.campaigns_by_pk;
     
     // Normalize and distribute to stores
     const normalized = normalize(campaign, campaignSchema);
     
     // Update each entity store
     Object.entries(normalized.entities).forEach(([entityType, entities]) => {
       const store = registry.getStore(entityType);
       if (store && entities) {
         store.setMany(Object.values(entities));
       }
     });
   }
   ```

### Step 5: Undo/Redo System
**Duration**: 2 days

1. **History Manager**
   ```typescript
   // src/stores/history/historyManager.ts
   import { create } from 'zustand';
   
   interface HistoryEntry {
     id: string;
     timestamp: Date;
     description: string;
     entityType: string;
     entityId: string;
     previousState: any;
     nextState: any;
     metadata?: Record<string, any>;
   }
   
   interface HistoryState {
     past: HistoryEntry[];
     future: HistoryEntry[];
     maxHistorySize: number;
     isUndoing: boolean;
     isRedoing: boolean;
   }
   
   interface HistoryActions {
     pushEntry: (entry: Omit<HistoryEntry, 'id' | 'timestamp'>) => void;
     undo: () => HistoryEntry | null;
     redo: () => HistoryEntry | null;
     canUndo: () => boolean;
     canRedo: () => boolean;
     clearHistory: () => void;
     getHistory: (entityType?: string, entityId?: string) => HistoryEntry[];
   }
   
   export const useHistoryStore = create<HistoryState & HistoryActions>((set, get) => ({
     past: [],
     future: [],
     maxHistorySize: 100,
     isUndoing: false,
     isRedoing: false,
     
     pushEntry: (entry) => set((state) => {
       const newEntry: HistoryEntry = {
         ...entry,
         id: `${Date.now()}_${Math.random()}`,
         timestamp: new Date(),
       };
       
       // Add to past
       const newPast = [...state.past, newEntry];
       
       // Limit history size
       if (newPast.length > state.maxHistorySize) {
         newPast.shift();
       }
       
       return {
         past: newPast,
         // Clear future when new action is performed
         future: state.isUndoing || state.isRedoing ? state.future : [],
         isUndoing: false,
         isRedoing: false,
       };
     }),
     
     undo: () => {
       const { past, future } = get();
       if (past.length === 0) return null;
       
       const entry = past[past.length - 1];
       
       set((state) => ({
         past: state.past.slice(0, -1),
         future: [entry, ...state.future],
         isUndoing: true,
       }));
       
       return entry;
     },
     
     redo: () => {
       const { future } = get();
       if (future.length === 0) return null;
       
       const entry = future[0];
       
       set((state) => ({
         past: [...state.past, entry],
         future: state.future.slice(1),
         isRedoing: true,
       }));
       
       return entry;
     },
     
     canUndo: () => get().past.length > 0,
     canRedo: () => get().future.length > 0,
     
     clearHistory: () => set({ past: [], future: [] }),
     
     getHistory: (entityType, entityId) => {
       const { past } = get();
       return past.filter(entry => {
         if (entityType && entry.entityType !== entityType) return false;
         if (entityId && entry.entityId !== entityId) return false;
         return true;
       });
     },
   }));
   
   // Hook for integrating with entity stores
   export function useUndoableStore<T>(
     store: any,
     entityType: string
   ) {
     const history = useHistoryStore();
     const { isUndoing, isRedoing } = history;
     
     // Wrap store actions with history tracking
     const trackedActions = {
       updateOne: (id: string, updates: Partial<T>, description?: string) => {
         const entity = store.selectById(id);
         if (!entity || isUndoing || isRedoing) {
           store.updateOne(id, updates);
           return;
         }
         
         const previousState = { ...entity };
         store.updateOne(id, updates);
         
         history.pushEntry({
           description: description || `Update ${entityType}`,
           entityType,
           entityId: id,
           previousState,
           nextState: { ...entity, ...updates },
         });
       },
       
       // Similar wrappers for other actions...
     };
     
     // Undo/redo handlers
     const handleUndo = () => {
       const entry = history.undo();
       if (entry && entry.entityType === entityType) {
         store.updateOne(entry.entityId, entry.previousState);
       }
     };
     
     const handleRedo = () => {
       const entry = history.redo();
       if (entry && entry.entityType === entityType) {
         store.updateOne(entry.entityId, entry.nextState);
       }
     };
     
     return {
       ...store,
       ...trackedActions,
       undo: handleUndo,
       redo: handleRedo,
       canUndo: history.canUndo,
       canRedo: history.canRedo,
     };
   }
   ```

## Testing Strategy

1. **State Management Testing**
   - Unit tests for all store operations
   - Normalization/denormalization tests
   - Relationship integrity tests

2. **Performance Testing**
   - Large dataset handling (10k+ entities)
   - Subscription performance
   - Memory leak detection

3. **Synchronization Testing**
   - Multi-client sync scenarios
   - Conflict resolution
   - Offline/online transitions

## Success Metrics

- Store operation latency <10ms
- Memory usage stable with 10k+ entities
- Zero lost updates in sync scenarios
- Undo/redo reliability 100%
- Optimistic update success rate >95%

## Next Phase
Proceed to [UPGRADE_PLAN_P7.md](./UPGRADE_PLAN_P7.md) for UI Component Architecture.