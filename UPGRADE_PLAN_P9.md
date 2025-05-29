# UPGRADE PLAN PHASE 9: SEARCH & DISCOVERY IMPLEMENTATION

## Overview
This phase focuses on implementing comprehensive search and discovery features for navigating 100+ entity types with full-text search, faceted filtering, and intelligent recommendations. The search system prioritizes finding connections to PC backstories and related world elements.

## Current State
- No search functionality
- Basic filtering only
- No cross-entity search
- Limited discovery features

## Target State
- Full-text search across all entities
- Faceted search interfaces
- Relationship-based discovery
- Saved searches and alerts
- Search result ranking
- Cross-entity unified search

## Implementation Steps

### Step 1: Search Infrastructure
**Duration**: 3 days

1. **Search Service Setup**
   ```typescript
   // src/services/search/searchService.ts
   import MiniSearch from 'minisearch';
   
   interface SearchDocument {
     id: string;
     entityType: string;
     name: string;
     description?: string;
     tags?: string[];
     metadata?: Record<string, any>;
   }
   
   export class SearchService {
     private indices: Map<string, MiniSearch>;
     private globalIndex: MiniSearch;
     
     constructor() {
       this.indices = new Map();
       this.globalIndex = new MiniSearch({
         fields: ['name', 'description', 'tags'],
         storeFields: ['id', 'entityType', 'name', 'metadata'],
         searchOptions: {
           boost: { name: 2 },
           fuzzy: 0.2,
           prefix: true,
         },
       });
     }
     
     createIndex(entityType: string, fields: string[]): void {
       this.indices.set(entityType, new MiniSearch({
         fields,
         storeFields: ['id', 'name', 'metadata'],
         searchOptions: {
           boost: { name: 2 },
           fuzzy: 0.2,
         },
       }));
     }
     
     indexDocuments(entityType: string, documents: any[]): void {
       const index = this.indices.get(entityType);
       if (!index) throw new Error(`No index for ${entityType}`);
       
       const searchDocs = documents.map(doc => ({
         id: `${entityType}:${doc.id}`,
         entityType,
         ...doc,
       }));
       
       index.addAll(searchDocs);
       this.globalIndex.addAll(searchDocs);
     }
     
     search(query: string, options?: SearchOptions): SearchResults {
       const { entityTypes, limit = 20, offset = 0 } = options || {};
       
       let results: any[];
       
       if (entityTypes && entityTypes.length === 1) {
         const index = this.indices.get(entityTypes[0]);
         results = index ? index.search(query) : [];
       } else {
         results = this.globalIndex.search(query);
         if (entityTypes) {
           results = results.filter(r => entityTypes.includes(r.entityType));
         }
       }
       
       const paginated = results.slice(offset, offset + limit);
       
       return {
         items: paginated,
         total: results.length,
         facets: this.calculateFacets(results),
       };
     }
     
     private calculateFacets(results: any[]): SearchFacets {
       const facets: SearchFacets = {
         entityTypes: new Map(),
         tags: new Map(),
       };
       
       results.forEach(result => {
         // Entity type facets
         const count = facets.entityTypes.get(result.entityType) || 0;
         facets.entityTypes.set(result.entityType, count + 1);
         
         // Tag facets
         if (result.tags) {
           result.tags.forEach(tag => {
             const tagCount = facets.tags.get(tag) || 0;
             facets.tags.set(tag, tagCount + 1);
           });
         }
       });
       
       return facets;
     }
   }
   ```

2. **Search Indexing Hook**
   ```typescript
   // src/hooks/useSearchIndex.ts
   import { useEffect } from 'react';
   import { useStore } from '@/stores';
   
   export function useSearchIndex(entityType: string) {
     const store = useStore(entityType);
     const searchService = useSearchService();
     
     useEffect(() => {
       // Initial indexing
       const entities = store.selectAll();
       if (entities.length > 0) {
         searchService.indexDocuments(entityType, entities);
       }
       
       // Subscribe to changes
       const unsubscribe = store.subscribe(
         (state) => state,
         (curr, prev) => {
           // Detect changes and update index
           const added = curr.allIds.filter(id => !prev.allIds.includes(id));
           const removed = prev.allIds.filter(id => !curr.allIds.includes(id));
           const updated = curr.allIds.filter(id => 
             prev.allIds.includes(id) && 
             curr.byId[id] !== prev.byId[id]
           );
           
           if (removed.length > 0) {
             searchService.removeDocuments(entityType, removed);
           }
           
           if (added.length > 0 || updated.length > 0) {
             const docs = [...added, ...updated].map(id => curr.byId[id]);
             searchService.updateDocuments(entityType, docs);
           }
         }
       );
       
       return unsubscribe;
     }, [entityType]);
   }
   ```

### Step 2: Faceted Search UI
**Duration**: 3 days

1. **Search Interface Component**
   ```typescript
   // src/components/search/SearchInterface.tsx
   import { useState, useCallback } from 'react';
   import { Search, Filter, X } from 'lucide-react';
   
   interface SearchInterfaceProps {
     entityTypes?: string[];
     onResults: (results: SearchResults) => void;
     savedSearches?: SavedSearch[];
   }
   
   export function SearchInterface({
     entityTypes,
     onResults,
     savedSearches,
   }: SearchInterfaceProps) {
     const [query, setQuery] = useState('');
     const [filters, setFilters] = useState<SearchFilters>({});
     const [selectedFacets, setSelectedFacets] = useState<SelectedFacets>({});
     const searchService = useSearchService();
     
     const performSearch = useCallback(() => {
       const results = searchService.search(query, {
         entityTypes: filters.entityTypes || entityTypes,
         filters: selectedFacets,
       });
       
       onResults(results);
     }, [query, filters, selectedFacets]);
     
     return (
       <div className="space-y-4">
         {/* Search Bar */}
         <div className="relative">
           <input
             type="text"
             value={query}
             onChange={(e) => setQuery(e.target.value)}
             onKeyDown={(e) => e.key === 'Enter' && performSearch()}
             placeholder="Search across all content..."
             className="w-full pl-10 pr-4 py-2 bg-gray-800 border border-gray-700 rounded-lg"
           />
           <Search className="absolute left-3 top-2.5 w-5 h-5 text-gray-400" />
         </div>
         
         {/* Quick Filters */}
         <div className="flex flex-wrap gap-2">
           {entityTypes?.map(type => (
             <button
               key={type}
               onClick={() => toggleEntityType(type)}
               className={cn(
                 'px-3 py-1 rounded-full text-sm',
                 filters.entityTypes?.includes(type)
                   ? 'bg-purple-600 text-white'
                   : 'bg-gray-700 text-gray-300'
               )}
             >
               {type}
             </button>
           ))}
         </div>
         
         {/* Saved Searches */}
         {savedSearches && savedSearches.length > 0 && (
           <div className="border-t border-gray-700 pt-4">
             <h3 className="text-sm font-medium text-gray-400 mb-2">
               Saved Searches
             </h3>
             <div className="space-y-1">
               {savedSearches.map(search => (
                 <button
                   key={search.id}
                   onClick={() => applySavedSearch(search)}
                   className="block w-full text-left px-3 py-2 text-sm hover:bg-gray-800 rounded"
                 >
                   {search.name}
                 </button>
               ))}
             </div>
           </div>
         )}
       </div>
     );
   }
   ```

2. **Search Results Component**
   ```typescript
   // src/components/search/SearchResults.tsx
   export function SearchResults({ results }: { results: SearchResults }) {
     const [groupBy, setGroupBy] = useState<'relevance' | 'type'>('relevance');
     
     const groupedResults = useMemo(() => {
       if (groupBy === 'type') {
         return groupResultsByType(results.items);
       }
       return { all: results.items };
     }, [results.items, groupBy]);
     
     return (
       <div className="space-y-4">
         {/* Results Header */}
         <div className="flex items-center justify-between">
           <p className="text-sm text-gray-400">
             {results.total} results found
           </p>
           <select
             value={groupBy}
             onChange={(e) => setGroupBy(e.target.value as any)}
             className="bg-gray-800 border border-gray-700 rounded px-2 py-1 text-sm"
           >
             <option value="relevance">By Relevance</option>
             <option value="type">By Type</option>
           </select>
         </div>
         
         {/* Facets */}
         <div className="flex flex-wrap gap-4">
           <FacetGroup
             title="Type"
             facets={results.facets.entityTypes}
             selected={selectedFacets.entityTypes}
             onChange={(values) => updateFacets('entityTypes', values)}
           />
           <FacetGroup
             title="Tags"
             facets={results.facets.tags}
             selected={selectedFacets.tags}
             onChange={(values) => updateFacets('tags', values)}
           />
         </div>
         
         {/* Results */}
         <div className="space-y-2">
           {Object.entries(groupedResults).map(([group, items]) => (
             <div key={group}>
               {groupBy === 'type' && (
                 <h3 className="text-lg font-medium text-purple-300 mb-2">
                   {group}
                 </h3>
               )}
               {items.map((item) => (
                 <SearchResultItem key={item.id} item={item} />
               ))}
             </div>
           ))}
         </div>
       </div>
     );
   }
   ```

### Step 3: Relationship Discovery
**Duration**: 2 days

1. **Discovery Engine**
   ```typescript
   // src/services/discovery/discoveryEngine.ts
   export class DiscoveryEngine {
     constructor(
       private relationshipStore: RelationshipStore,
       private searchService: SearchService
     ) {}
     
     async discoverRelated(
       entityId: string,
       entityType: string,
       options: DiscoveryOptions = {}
     ): Promise<DiscoveryResults> {
       // Prioritize PC backstory connections
       const { prioritizePCConnections = true } = options;
       const {
         maxDepth = 2,
         limit = 20,
         types = [],
       } = options;
       
       // Get directly related entities
       const related = await this.relationshipStore.getRelatedEntities(
         entityId,
         entityType,
         maxDepth
       );
       
       // Score by relevance
       const scored = this.scoreRelatedEntities(related, {
         sourceId: entityId,
         sourceType: entityType,
       });
       
       // Filter by type if specified
       const filtered = types.length > 0
         ? scored.filter(e => types.includes(e.type))
         : scored;
       
       // Get top results
       const topResults = filtered.slice(0, limit);
       
       return {
         items: topResults,
         total: filtered.length,
         paths: this.getDiscoveryPaths(entityId, topResults),
       };
     }
     
     private scoreRelatedEntities(
       entities: RelatedEntity[],
       context: ScoringContext
     ): ScoredEntity[] {
       return entities.map(entity => {
         let score = 0;
         
         // Distance score (closer = higher)
         score += (10 - entity.distance) * 10;
         
         // Relationship type score
         const relScore = {
           parent: 50,
           child: 40,
           sibling: 30,
           related: 20,
         };
         score += relScore[entity.relationshipType] || 10;
         
         // Shared attributes score
         score += entity.sharedAttributes.length * 5;
         
         // PC connection bonus
         if (entity.connected_to_pc) {
           score += 100;
         }
         
         // Backstory element bonus
         if (entity.is_backstory_element) {
           score += 75;
         }
         
         return {
           ...entity,
           score,
         };
       }).sort((a, b) => b.score - a.score);
     }
   }
   ```

### Step 4: Saved Searches
**Duration**: 2 days

1. **Saved Search Management**
   ```typescript
   // src/stores/savedSearchStore.ts
   import { create } from 'zustand';
   import { persist } from 'zustand/middleware';
   
   interface SavedSearch {
     id: string;
     name: string;
     query: string;
     filters: SearchFilters;
     entityTypes: string[];
     createdAt: Date;
     lastUsed?: Date;
     notifyOnNew?: boolean;
   }
   
   interface SavedSearchStore {
     searches: SavedSearch[];
     addSearch: (search: Omit<SavedSearch, 'id' | 'createdAt'>) => void;
     removeSearch: (id: string) => void;
     updateSearch: (id: string, updates: Partial<SavedSearch>) => void;
     useSearch: (id: string) => SavedSearch | undefined;
   }
   
   export const useSavedSearchStore = create<SavedSearchStore>()(
     persist(
       (set, get) => ({
         searches: [],
         
         addSearch: (search) => set((state) => ({
           searches: [...state.searches, {
             ...search,
             id: generateId(),
             createdAt: new Date(),
           }],
         })),
         
         removeSearch: (id) => set((state) => ({
           searches: state.searches.filter(s => s.id !== id),
         })),
         
         updateSearch: (id, updates) => set((state) => ({
           searches: state.searches.map(s =>
             s.id === id ? { ...s, ...updates } : s
           ),
         })),
         
         useSearch: (id) => {
           const search = get().searches.find(s => s.id === id);
           if (search) {
             get().updateSearch(id, { lastUsed: new Date() });
           }
           return search;
         },
       }),
       {
         name: 'saved-searches',
       }
     )
   );
   ```

### Step 5: Search Analytics
**Duration**: 1 day

1. **Search Analytics Tracker**
   ```typescript
   // src/services/analytics/searchAnalytics.ts
   export class SearchAnalytics {
     private searches: SearchEvent[] = [];
     
     trackSearch(event: {
       query: string;
       filters: any;
       resultCount: number;
       clickedResults: string[];
     }): void {
       this.searches.push({
         ...event,
         timestamp: new Date(),
         sessionId: this.getSessionId(),
       });
       
       // Send to analytics service
       this.sendAnalytics('search', event);
     }
     
     getPopularSearches(days: number = 7): PopularSearch[] {
       const cutoff = new Date();
       cutoff.setDate(cutoff.getDate() - days);
       
       const recent = this.searches.filter(s => s.timestamp > cutoff);
       const grouped = this.groupByQuery(recent);
       
       return Object.entries(grouped)
         .map(([query, events]) => ({
           query,
           count: events.length,
           avgResults: this.average(events.map(e => e.resultCount)),
           clickRate: this.calculateClickRate(events),
         }))
         .sort((a, b) => b.count - a.count)
         .slice(0, 10);
     }
   }
   ```

## Testing Strategy

1. **Search Accuracy**
   - Relevance scoring validation
   - Fuzzy matching testing
   - Cross-entity search verification

2. **Performance Testing**
   - Index build time < 1s for 10k documents
   - Search response time < 100ms
   - Facet calculation < 50ms

## Success Metrics

- Search result relevance > 90%
- Average search time < 100ms
- Click-through rate > 30%
- Zero failed searches
- Index size < 10% of data size

## Next Phase
Proceed to [UPGRADE_PLAN_P10.md](./UPGRADE_PLAN_P10.md) for Export & Integration.