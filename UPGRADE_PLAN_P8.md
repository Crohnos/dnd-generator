# UPGRADE PLAN PHASE 8: PERFORMANCE OPTIMIZATION

## Overview
This phase focuses on optimizing the application to handle 100+ entity types with thousands of records while maintaining smooth performance across all operations. Special attention is given to efficiently loading and displaying PC backstory connections and related world elements.

## Current State
- Basic loading for small datasets
- No lazy loading
- Limited caching
- No performance monitoring

## Target State
- Lazy loading strategies
- Virtual scrolling for large lists
- Efficient caching system
- CDN integration
- Performance monitoring

## Implementation Steps

### Step 1: Virtual Scrolling Implementation
**Duration**: 3 days

1. **Virtual List Component**
   ```typescript
   // src/components/performance/VirtualList.tsx
   import { useVirtualizer } from '@tanstack/react-virtual';
   import { useRef, useMemo } from 'react';
   
   interface VirtualListProps<T> {
     items: T[];
     height: number;
     itemHeight: number | ((index: number) => number);
     renderItem: (item: T, index: number) => React.ReactNode;
     overscan?: number;
     className?: string;
   }
   
   export function VirtualList<T>({
     items,
     height,
     itemHeight,
     renderItem,
     overscan = 5,
     className,
   }: VirtualListProps<T>) {
     const parentRef = useRef<HTMLDivElement>(null);
     
     const virtualizer = useVirtualizer({
       count: items.length,
       getScrollElement: () => parentRef.current,
       estimateSize: typeof itemHeight === 'function' 
         ? itemHeight 
         : () => itemHeight,
       overscan,
     });
     
     const virtualItems = virtualizer.getVirtualItems();
     
     return (
       <div
         ref={parentRef}
         className={cn('overflow-auto', className)}
         style={{ height }}
       >
         <div
           style={{
             height: `${virtualizer.getTotalSize()}px`,
             width: '100%',
             position: 'relative',
           }}
         >
           {virtualItems.map((virtualItem) => (
             <div
               key={virtualItem.key}
               style={{
                 position: 'absolute',
                 top: 0,
                 left: 0,
                 width: '100%',
                 height: `${virtualItem.size}px`,
                 transform: `translateY(${virtualItem.start}px)`,
               }}
             >
               {renderItem(items[virtualItem.index], virtualItem.index)}
             </div>
           ))}
         </div>
       </div>
     );
   }
   ```

2. **Virtual Grid Component**
   ```typescript
   // src/components/performance/VirtualGrid.tsx
   import { FixedSizeGrid as Grid } from 'react-window';
   import AutoSizer from 'react-virtualized-auto-sizer';
   
   interface VirtualGridProps<T> {
     items: T[];
     columnCount: number;
     rowHeight: number;
     columnWidth: number;
     renderCell: (item: T, style: React.CSSProperties) => React.ReactNode;
   }
   
   export function VirtualGrid<T>({
     items,
     columnCount,
     rowHeight,
     columnWidth,
     renderCell,
   }: VirtualGridProps<T>) {
     const rowCount = Math.ceil(items.length / columnCount);
     
     const Cell = ({ columnIndex, rowIndex, style }) => {
       const index = rowIndex * columnCount + columnIndex;
       if (index >= items.length) return null;
       
       return renderCell(items[index], style);
     };
     
     return (
       <AutoSizer>
         {({ height, width }) => (
           <Grid
             columnCount={columnCount}
             columnWidth={columnWidth}
             height={height}
             rowCount={rowCount}
             rowHeight={rowHeight}
             width={width}
           >
             {Cell}
           </Grid>
         )}
       </AutoSizer>
     );
   }
   ```

### Step 2: Lazy Loading System
**Duration**: 3 days

1. **Lazy Loading Hook**
   ```typescript
   // src/hooks/useLazyLoad.ts
   import { useState, useEffect, useCallback, useRef } from 'react';
   
   interface LazyLoadOptions {
     threshold?: number;
     rootMargin?: string;
     pageSize?: number;
   }
   
   export function useLazyLoad<T>(
     loadMore: (page: number) => Promise<T[]>,
     options: LazyLoadOptions = {}
   ) {
     const {
       threshold = 0.1,
       rootMargin = '100px',
       pageSize = 20,
     } = options;
     
     const [items, setItems] = useState<T[]>([]);
     const [page, setPage] = useState(1);
     const [loading, setLoading] = useState(false);
     const [hasMore, setHasMore] = useState(true);
     const [error, setError] = useState<Error | null>(null);
     
     const observerRef = useRef<IntersectionObserver | null>(null);
     const loadMoreRef = useCallback((node: HTMLElement | null) => {
       if (loading) return;
       
       if (observerRef.current) {
         observerRef.current.disconnect();
       }
       
       observerRef.current = new IntersectionObserver(
         (entries) => {
           if (entries[0].isIntersecting && hasMore && !loading) {
             loadNextPage();
           }
         },
         { threshold, rootMargin }
       );
       
       if (node) {
         observerRef.current.observe(node);
       }
     }, [loading, hasMore]);
     
     const loadNextPage = async () => {
       if (loading || !hasMore) return;
       
       setLoading(true);
       setError(null);
       
       try {
         const newItems = await loadMore(page);
         
         if (newItems.length === 0 || newItems.length < pageSize) {
           setHasMore(false);
         }
         
         setItems(prev => [...prev, ...newItems]);
         setPage(prev => prev + 1);
       } catch (err) {
         setError(err as Error);
       } finally {
         setLoading(false);
       }
     };
     
     return {
       items,
       loading,
       error,
       hasMore,
       loadMoreRef,
       refresh: () => {
         setItems([]);
         setPage(1);
         setHasMore(true);
         loadNextPage();
       },
     };
   }
   ```

2. **Image Lazy Loading**
   ```typescript
   // src/components/performance/LazyImage.tsx
   import { useState, useEffect, useRef } from 'react';
   
   interface LazyImageProps {
     src: string;
     alt: string;
     placeholder?: string;
     className?: string;
     onLoad?: () => void;
     onError?: () => void;
   }
   
   export function LazyImage({
     src,
     alt,
     placeholder = '/placeholder.svg',
     className,
     onLoad,
     onError,
   }: LazyImageProps) {
     const [imageSrc, setImageSrc] = useState(placeholder);
     const [imageRef, setImageRef] = useState<HTMLImageElement | null>(null);
     const observerRef = useRef<IntersectionObserver | null>(null);
     
     useEffect(() => {
       if (!imageRef) return;
       
       observerRef.current = new IntersectionObserver(
         (entries) => {
           entries.forEach((entry) => {
             if (entry.isIntersecting) {
               const img = new Image();
               img.src = src;
               
               img.onload = () => {
                 setImageSrc(src);
                 onLoad?.();
               };
               
               img.onerror = () => {
                 onError?.();
               };
               
               observerRef.current?.disconnect();
             }
           });
         },
         { threshold: 0.1 }
       );
       
       observerRef.current.observe(imageRef);
       
       return () => {
         observerRef.current?.disconnect();
       };
     }, [imageRef, src]);
     
     return (
       <img
         ref={setImageRef}
         src={imageSrc}
         alt={alt}
         className={className}
         loading="lazy"
       />
     );
   }
   ```

### Step 3: Caching Strategy
**Duration**: 2 days

1. **Query Cache Manager**
   ```typescript
   // src/lib/cache/queryCache.ts
   import { LRUCache } from 'lru-cache';
   
   class QueryCacheManager {
     private cache: LRUCache<string, any>;
     private subscriptions: Map<string, Set<() => void>>;
     
     constructor(options: { max: number; ttl: number }) {
       this.cache = new LRUCache({
         max: options.max,
         ttl: options.ttl,
       });
       this.subscriptions = new Map();
     }
     
     get<T>(key: string): T | undefined {
       return this.cache.get(key);
     }
     
     set<T>(key: string, value: T, ttl?: number): void {
       this.cache.set(key, value, { ttl });
       this.notify(key);
     }
     
     invalidate(pattern?: string | RegExp): void {
       if (!pattern) {
         this.cache.clear();
       } else {
         const keys = Array.from(this.cache.keys());
         keys.forEach(key => {
           if (typeof pattern === 'string' ? key.includes(pattern) : pattern.test(key)) {
             this.cache.delete(key);
             this.notify(key);
           }
         });
       }
     }
     
     subscribe(key: string, callback: () => void): () => void {
       if (!this.subscriptions.has(key)) {
         this.subscriptions.set(key, new Set());
       }
       
       this.subscriptions.get(key)!.add(callback);
       
       return () => {
         this.subscriptions.get(key)?.delete(callback);
       };
     }
     
     private notify(key: string): void {
       this.subscriptions.get(key)?.forEach(callback => callback());
     }
   }
   
   export const queryCache = new QueryCacheManager({
     max: 500,
     ttl: 5 * 60 * 1000, // 5 minutes
   });
   ```

2. **Prefetch Hook**
   ```typescript
   // src/hooks/usePrefetch.ts
   export function usePrefetch() {
     const prefetchQuery = useCallback(async (
       key: string,
       fetcher: () => Promise<any>,
       options?: { force?: boolean }
     ) => {
       if (!options?.force && queryCache.get(key)) {
         return;
       }
       
       try {
         const data = await fetcher();
         queryCache.set(key, data);
       } catch (error) {
         console.error(`Prefetch failed for ${key}:`, error);
       }
     }, []);
     
     const prefetchRelated = useCallback(async (
       entityType: string,
       entityId: string,
       depth: number = 1
     ) => {
       // Prefetch related entities based on relationships
       const relationships = getEntityRelationships(entityType);
       
       for (const rel of relationships) {
         const key = `${rel.type}:${entityId}:${rel.field}`;
         await prefetchQuery(key, () => 
           fetchRelatedEntities(entityType, entityId, rel.field)
         );
       }
     }, [prefetchQuery]);
     
     return { prefetchQuery, prefetchRelated };
   }
   ```

### Step 4: Bundle Optimization
**Duration**: 2 days

1. **Dynamic Imports**
   ```typescript
   // src/lib/dynamicImports.ts
   import dynamic from 'next/dynamic';
   import { ComponentType } from 'react';
   
   // Heavy components
   export const RelationshipGraph = dynamic(
     () => import('@/components/visualizations/RelationshipGraph'),
     {
       loading: () => <GraphSkeleton />,
       ssr: false,
     }
   );
   
   export const BackstoryNetwork = dynamic(
     () => import('@/components/visualizations/BackstoryNetwork'),
     {
       loading: () => <GraphSkeleton />,
       ssr: false,
     }
   );
   
   export const AdvancedEditor = dynamic(
     () => import('@/components/editors/AdvancedEditor'),
     {
       loading: () => <EditorSkeleton />,
     }
   );
   
   // Route-based code splitting
   export const routes = {
     '/campaigns/[id]/graph': dynamic(
       () => import('@/app/campaigns/[id]/graph/page')
     ),
     '/campaigns/[id]/export': dynamic(
       () => import('@/app/campaigns/[id]/export/page')
     ),
   };
   ```

2. **Asset Optimization**
   ```typescript
   // next.config.js
   module.exports = {
     images: {
       domains: ['cdn.example.com'],
       deviceSizes: [640, 750, 828, 1080, 1200],
       imageSizes: [16, 32, 48, 64, 96],
       formats: ['image/webp'],
     },
     compress: true,
     poweredByHeader: false,
     generateEtags: true,
     
     webpack: (config, { isServer }) => {
       // Bundle analyzer
       if (process.env.ANALYZE) {
         const { BundleAnalyzerPlugin } = require('webpack-bundle-analyzer');
         config.plugins.push(
           new BundleAnalyzerPlugin({
             analyzerMode: 'static',
             reportFilename: isServer
               ? '../analyze/server.html'
               : './analyze/client.html',
           })
         );
       }
       
       return config;
     },
   };
   ```

### Step 5: Performance Monitoring
**Duration**: 1 day

1. **Performance Observer**
   ```typescript
   // src/lib/performance/monitor.ts
   class PerformanceMonitor {
     private metrics: Map<string, number[]> = new Map();
     
     measureComponent(name: string, fn: () => void): void {
       const start = performance.now();
       fn();
       const duration = performance.now() - start;
       
       if (!this.metrics.has(name)) {
         this.metrics.set(name, []);
       }
       
       this.metrics.get(name)!.push(duration);
       
       if (duration > 16) { // Longer than one frame
         console.warn(`Slow render: ${name} took ${duration.toFixed(2)}ms`);
       }
     }
     
     getMetrics(name: string) {
       const times = this.metrics.get(name) || [];
       if (times.length === 0) return null;
       
       return {
         avg: times.reduce((a, b) => a + b) / times.length,
         min: Math.min(...times),
         max: Math.max(...times),
         p95: this.percentile(times, 0.95),
       };
     }
     
     private percentile(arr: number[], p: number): number {
       const sorted = [...arr].sort((a, b) => a - b);
       const index = Math.ceil(sorted.length * p) - 1;
       return sorted[index];
     }
   }
   
   export const perfMonitor = new PerformanceMonitor();
   ```

## Testing Strategy

1. **Performance Benchmarks**
   - Initial load time < 3s
   - Time to interactive < 5s
   - Frame rate > 55fps during scrolling
   - Memory usage < 200MB for 10k items

2. **Load Testing**
   - 10,000+ entities
   - 100+ concurrent users
   - Complex filtering scenarios

## Success Metrics

- 90% of operations complete in < 100ms
- Virtual scrolling maintains 60fps
- Bundle size < 500KB (gzipped)
- Cache hit rate > 80%
- Zero memory leaks

## Next Phase
Proceed to [UPGRADE_PLAN_P9.md](./UPGRADE_PLAN_P9.md) for Search & Discovery Implementation.