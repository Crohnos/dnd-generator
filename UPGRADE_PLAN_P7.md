# UPGRADE PLAN PHASE 7: UI COMPONENT ARCHITECTURE

## Overview
This phase focuses on building a comprehensive component library to display and interact with 100+ entity types, with special emphasis on visualizing PC backstory connections, relationship networks, and the lived-in world elements.

## Current State
- Simple cards for 4 entity types
- Basic display components
- No component reusability system
- Limited interaction patterns

## Target State
- Reusable component library for 30+ entity types
- Complex relationship visualizations
- Hierarchical navigation components
- Search and filter interfaces
- Bulk editing interfaces
- Export/print layouts
- Responsive and accessible

## Implementation Steps

### Step 1: Component System Foundation
**Duration**: 4 days

1. **Base Component Architecture**
   ```typescript
   // src/components/system/BaseEntityCard.tsx
   import React, { ReactNode } from 'react';
   import { motion, AnimatePresence } from 'framer-motion';
   import { MoreVertical, Eye, EyeOff, Edit, Trash, Link } from 'lucide-react';
   import { cn } from '@/lib/utils';
   
   export interface BaseEntityCardProps<T> {
     entity: T;
     entityType: string;
     title: string;
     subtitle?: string;
     avatar?: ReactNode;
     badges?: Badge[];
     sections?: CardSection[];
     actions?: CardAction[];
     expandable?: boolean;
     selectable?: boolean;
     selected?: boolean;
     onSelect?: (entity: T) => void;
     onClick?: (entity: T) => void;
     className?: string;
   }
   
   export interface Badge {
     label: string;
     variant: 'primary' | 'secondary' | 'success' | 'warning' | 'danger' | 'info';
     icon?: ReactNode;
   }
   
   export interface CardSection {
     id: string;
     title?: string;
     content: ReactNode;
     collapsible?: boolean;
     defaultCollapsed?: boolean;
     hidden?: boolean;
     secret?: boolean;
   }
   
   export interface CardAction {
     id: string;
     label: string;
     icon: ReactNode;
     onClick: () => void;
     variant?: 'default' | 'danger';
     disabled?: boolean;
   }
   
   export function BaseEntityCard<T extends { id: string | number }>({
     entity,
     entityType,
     title,
     subtitle,
     avatar,
     badges = [],
     sections = [],
     actions = [],
     expandable = false,
     selectable = false,
     selected = false,
     onSelect,
     onClick,
     className,
   }: BaseEntityCardProps<T>) {
     const [expanded, setExpanded] = React.useState(false);
     const [secretsVisible, setSecretsVisible] = React.useState(false);
     const [actionsOpen, setActionsOpen] = React.useState(false);
     
     const visibleSections = sections.filter(s => !s.hidden);
     const secretSections = visibleSections.filter(s => s.secret);
     const normalSections = visibleSections.filter(s => !s.secret);
     
     return (
       <motion.div
         layout
         initial={{ opacity: 0, y: 20 }}
         animate={{ opacity: 1, y: 0 }}
         exit={{ opacity: 0, y: -20 }}
         className={cn(
           'bg-gray-800 rounded-lg border border-gray-700 hover:border-purple-500/50 transition-all',
           selectable && 'cursor-pointer',
           selected && 'ring-2 ring-purple-500',
           className
         )}
         onClick={() => {
           if (onClick) onClick(entity);
           if (selectable && onSelect) onSelect(entity);
         }}
       >
         {/* Header */}
         <div className="p-4">
           <div className="flex items-start justify-between">
             <div className="flex items-start space-x-3 flex-1">
               {avatar && (
                 <div className="flex-shrink-0">
                   {avatar}
                 </div>
               )}
               
               <div className="flex-1 min-w-0">
                 <h3 className="text-lg font-semibold text-white truncate">
                   {title}
                 </h3>
                 {subtitle && (
                   <p className="text-sm text-gray-400 truncate">
                     {subtitle}
                   </p>
                 )}
                 
                 {badges.length > 0 && (
                   <div className="flex flex-wrap gap-2 mt-2">
                     {badges.map((badge, i) => (
                       <BadgeComponent key={i} {...badge} />
                     ))}
                   </div>
                 )}
               </div>
             </div>
             
             {/* Actions */}
             {actions.length > 0 && (
               <div className="relative ml-2">
                 <button
                   onClick={(e) => {
                     e.stopPropagation();
                     setActionsOpen(!actionsOpen);
                   }}
                   className="p-1 rounded hover:bg-gray-700"
                 >
                   <MoreVertical className="w-5 h-5 text-gray-400" />
                 </button>
                 
                 <AnimatePresence>
                   {actionsOpen && (
                     <ActionMenu
                       actions={actions}
                       onClose={() => setActionsOpen(false)}
                     />
                   )}
                 </AnimatePresence>
               </div>
             )}
           </div>
         </div>
         
         {/* Content Sections */}
         <div className="px-4 pb-4 space-y-3">
           {normalSections.map((section) => (
             <CardSectionComponent
               key={section.id}
               section={section}
               expanded={expanded}
             />
           ))}
           
           {/* Secret Sections */}
           {secretSections.length > 0 && (
             <div className="pt-3 border-t border-gray-700">
               <button
                 onClick={(e) => {
                   e.stopPropagation();
                   setSecretsVisible(!secretsVisible);
                 }}
                 className="flex items-center space-x-2 text-sm text-yellow-500 hover:text-yellow-400"
               >
                 {secretsVisible ? (
                   <EyeOff className="w-4 h-4" />
                 ) : (
                   <Eye className="w-4 h-4" />
                 )}
                 <span>
                   {secretsVisible ? 'Hide' : 'Show'} DM Secrets
                 </span>
               </button>
               
               <AnimatePresence>
                 {secretsVisible && (
                   <motion.div
                     initial={{ height: 0, opacity: 0 }}
                     animate={{ height: 'auto', opacity: 1 }}
                     exit={{ height: 0, opacity: 0 }}
                     className="mt-3 p-3 bg-yellow-900/20 border border-yellow-700/50 rounded"
                   >
                     {secretSections.map((section) => (
                       <CardSectionComponent
                         key={section.id}
                         section={section}
                         expanded={true}
                       />
                     ))}
                   </motion.div>
                 )}
               </AnimatePresence>
             </div>
           )}
         </div>
         
         {/* Expandable Footer */}
         {expandable && (
           <button
             onClick={(e) => {
               e.stopPropagation();
               setExpanded(!expanded);
             }}
             className="w-full px-4 py-2 text-sm text-gray-400 hover:text-white hover:bg-gray-700/50 border-t border-gray-700"
           >
             {expanded ? 'Show Less' : 'Show More'}
           </button>
         )}
       </motion.div>
     );
   }
   ```

2. **Entity-Specific Card Components**
   ```typescript
   // src/components/entities/CharacterCard.tsx
   import { BaseEntityCard } from '../system/BaseEntityCard';
   import { CharacterAvatar } from '../avatars/CharacterAvatar';
   import { Character } from '@/types';
   
   interface CharacterCardProps {
     character: Character;
     showRelationships?: boolean;
     showStats?: boolean;
     showSecrets?: boolean;
     variant?: 'compact' | 'full' | 'detailed';
     onEdit?: (character: Character) => void;
     onDelete?: (character: Character) => void;
     onViewRelationships?: (character: Character) => void;
   }
   
   export function CharacterCard({
     character,
     showRelationships = true,
     showStats = false,
     showSecrets = true,
     variant = 'full',
     onEdit,
     onDelete,
     onViewRelationships,
   }: CharacterCardProps) {
     const badges = [
       {
         label: character.character_type,
         variant: character.character_type === 'pc' ? 'primary' : 'secondary',
       },
       character.race && {
         label: character.race.name,
         variant: 'info',
       },
       character.culture && {
         label: character.culture.name,
         variant: 'info',
       },
     ].filter(Boolean);
     
     const sections = [
       {
         id: 'core',
         content: (
           <div className="space-y-2 text-sm">
             {character.core_identity && (
               <p className="text-gray-300">{character.core_identity}</p>
             )}
             {character.primary_motivation && (
               <div>
                 <span className="text-gray-500">Motivation:</span>{' '}
                 <span className="text-gray-300">{character.primary_motivation}</span>
               </div>
             )}
           </div>
         ),
       },
       character.personality_traits && {
         id: 'personality',
         title: 'Personality Traits',
         content: (
           <div className="flex flex-wrap gap-2">
             {character.personality_traits.map((trait, i) => (
               <span
                 key={i}
                 className="px-2 py-1 bg-purple-900/30 text-purple-300 rounded text-xs"
               >
                 {trait}
               </span>
             ))}
           </div>
         ),
         collapsible: true,
       },
       showStats && character.stats && {
         id: 'stats',
         title: 'Statistics',
         content: <CharacterStats stats={character.stats} />,
         collapsible: true,
         defaultCollapsed: true,
       },
       showRelationships && character.relationships?.length > 0 && {
         id: 'relationships',
         title: 'Relationships',
         content: (
           <RelationshipList
             relationships={character.relationships}
             onViewDetails={onViewRelationships}
           />
         ),
         collapsible: true,
       },
       character.connected_to_pc_id && {
         id: 'pc_connection',
         title: 'PC Connection',
         content: (
           <div className="bg-purple-900/20 p-3 rounded">
             <p className="text-sm text-purple-300">
               {character.connection_type} of {character.connected_pc_name}
             </p>
             {character.backstory_summary && (
               <p className="text-sm text-gray-300 mt-2">
                 {character.backstory_summary}
               </p>
             )}
           </div>
         ),
       },
       showSecrets && character.hidden_information && {
         id: 'secrets',
         title: 'Hidden Information',
         content: (
           <p className="text-sm text-yellow-300">
             {character.hidden_information}
           </p>
         ),
         secret: true,
       },
     ].filter(Boolean);
     
     const actions = [
       onEdit && {
         id: 'edit',
         label: 'Edit',
         icon: <Edit className="w-4 h-4" />,
         onClick: () => onEdit(character),
       },
       onViewRelationships && {
         id: 'relationships',
         label: 'View Relationships',
         icon: <Link className="w-4 h-4" />,
         onClick: () => onViewRelationships(character),
       },
       onDelete && {
         id: 'delete',
         label: 'Delete',
         icon: <Trash className="w-4 h-4" />,
         onClick: () => onDelete(character),
         variant: 'danger',
       },
     ].filter(Boolean);
     
     return (
       <BaseEntityCard
         entity={character}
         entityType="character"
         title={character.name}
         subtitle={`${character.level ? `Level ${character.level}` : ''} ${character.class || ''} ${character.connected_to_pc_id ? '• Connected to PC' : ''}`}
         avatar={<CharacterAvatar character={character} size="md" />}
         badges={badges}
         sections={sections}
         actions={actions}
         expandable={variant === 'full'}
       />
     );
   }
   ```

### Step 2: Relationship Visualizations
**Duration**: 4 days

1. **Interactive Relationship Graph**
   ```typescript
   // src/components/visualizations/RelationshipGraph.tsx
   import React, { useEffect, useRef, useMemo } from 'react';
   import * as d3 from 'd3';
   import { useResizeObserver } from '@/hooks/useResizeObserver';
   
   interface GraphNode {
     id: string;
     name: string;
     type: string;
     group?: string;
     avatar?: string;
     metadata?: Record<string, any>;
   }
   
   interface GraphLink {
     source: string;
     target: string;
     type: string;
     label?: string;
     strength?: number;
     metadata?: Record<string, any>;
   }
   
   interface RelationshipGraphProps {
     nodes: GraphNode[];
     links: GraphLink[];
     centerNodeId?: string;
     maxDepth?: number;
     nodeSize?: number;
     linkDistance?: number;
     chargeStrength?: number;
     onNodeClick?: (node: GraphNode) => void;
     onLinkClick?: (link: GraphLink) => void;
     onNodeHover?: (node: GraphNode | null) => void;
     className?: string;
   }
   
   export function RelationshipGraph({
     nodes,
     links,
     centerNodeId,
     maxDepth = 3,
     nodeSize = 40,
     linkDistance = 100,
     chargeStrength = -300,
     onNodeClick,
     onLinkClick,
     onNodeHover,
     className,
   }: RelationshipGraphProps) {
     const svgRef = useRef<SVGSVGElement>(null);
     const containerRef = useRef<HTMLDivElement>(null);
     const dimensions = useResizeObserver(containerRef);
     
     // Filter nodes by depth from center
     const visibleNodes = useMemo(() => {
       if (!centerNodeId) return nodes;
       
       const depths = new Map<string, number>();
       const queue = [{ id: centerNodeId, depth: 0 }];
       const visited = new Set<string>();
       
       while (queue.length > 0) {
         const { id, depth } = queue.shift()!;
         if (visited.has(id) || depth > maxDepth) continue;
         
         visited.add(id);
         depths.set(id, depth);
         
         // Add connected nodes
         links.forEach(link => {
           if (link.source === id && !visited.has(link.target)) {
             queue.push({ id: link.target, depth: depth + 1 });
           }
           if (link.target === id && !visited.has(link.source)) {
             queue.push({ id: link.source, depth: depth + 1 });
           }
         });
       }
       
       return nodes.filter(n => depths.has(n.id));
     }, [nodes, links, centerNodeId, maxDepth]);
     
     // Filter links to visible nodes
     const visibleLinks = useMemo(() => {
       const nodeIds = new Set(visibleNodes.map(n => n.id));
       return links.filter(l => nodeIds.has(l.source) && nodeIds.has(l.target));
     }, [links, visibleNodes]);
     
     useEffect(() => {
       if (!svgRef.current || !dimensions) return;
       
       const { width, height } = dimensions;
       const svg = d3.select(svgRef.current);
       
       // Clear previous content
       svg.selectAll('*').remove();
       
       // Create zoom behavior
       const zoom = d3.zoom<SVGSVGElement, unknown>()
         .scaleExtent([0.1, 4])
         .on('zoom', (event) => {
           g.attr('transform', event.transform);
         });
       
       svg.call(zoom);
       
       // Create main group
       const g = svg.append('g');
       
       // Create arrow markers
       svg.append('defs').selectAll('marker')
         .data(['resolved', 'unresolved', 'family', 'hostile'])
         .join('marker')
         .attr('id', d => `arrow-${d}`)
         .attr('viewBox', '0 -5 10 10')
         .attr('refX', nodeSize / 2 + 10)
         .attr('refY', 0)
         .attr('markerWidth', 6)
         .attr('markerHeight', 6)
         .attr('orient', 'auto')
         .append('path')
         .attr('d', 'M0,-5L10,0L0,5')
         .attr('fill', d => {
           const colors = {
             resolved: '#10b981',
             unresolved: '#ef4444',
             family: '#3b82f6',
             hostile: '#ef4444',
           };
           return colors[d] || '#6b7280';
         });
       
       // Create simulation
       const simulation = d3.forceSimulation(visibleNodes)
         .force('link', d3.forceLink(visibleLinks)
           .id((d: any) => d.id)
           .distance(linkDistance)
         )
         .force('charge', d3.forceManyBody().strength(chargeStrength))
         .force('center', d3.forceCenter(width / 2, height / 2))
         .force('collision', d3.forceCollide(nodeSize));
       
       // Create links
       const link = g.append('g')
         .selectAll('line')
         .data(visibleLinks)
         .join('line')
         .attr('stroke', d => {
           const colors = {
             family: '#3b82f6',
             ally: '#10b981',
             enemy: '#ef4444',
             neutral: '#6b7280',
           };
           return colors[d.type] || '#6b7280';
         })
         .attr('stroke-width', d => Math.sqrt(d.strength || 1) * 2)
         .attr('marker-end', d => `url(#arrow-${d.type})`);
       
       // Create link labels
       const linkLabel = g.append('g')
         .selectAll('text')
         .data(visibleLinks.filter(d => d.label))
         .join('text')
         .attr('text-anchor', 'middle')
         .attr('fill', '#9ca3af')
         .attr('font-size', '12px')
         .text(d => d.label!);
       
       // Create nodes
       const node = g.append('g')
         .selectAll('g')
         .data(visibleNodes)
         .join('g')
         .call(d3.drag<SVGGElement, GraphNode>()
           .on('start', dragstarted)
           .on('drag', dragged)
           .on('end', dragended)
         );
       
       // Add circles
       node.append('circle')
         .attr('r', nodeSize / 2)
         .attr('fill', d => {
           const colors = {
             pc: '#8b5cf6',
             npc: '#3b82f6',
             location: '#10b981',
             organization: '#f59e0b',
             item: '#ef4444',
           };
           return colors[d.type] || '#6b7280';
         })
         .attr('stroke', '#1f2937')
         .attr('stroke-width', 2);
       
       // Add labels
       node.append('text')
         .text(d => d.name)
         .attr('x', 0)
         .attr('y', nodeSize / 2 + 15)
         .attr('text-anchor', 'middle')
         .attr('fill', '#e5e7eb')
         .attr('font-size', '14px');
       
       // Add avatars/icons
       node.each(function(d) {
         if (d.avatar) {
           d3.select(this).append('image')
             .attr('href', d.avatar)
             .attr('x', -nodeSize / 4)
             .attr('y', -nodeSize / 4)
             .attr('width', nodeSize / 2)
             .attr('height', nodeSize / 2)
             .attr('clip-path', 'circle()');
         }
       });
       
       // Add interactions
       node.on('click', (event, d) => {
         event.stopPropagation();
         onNodeClick?.(d);
       })
       .on('mouseenter', (event, d) => {
         onNodeHover?.(d);
         // Highlight connected nodes
         node.style('opacity', n => {
           if (n.id === d.id) return 1;
           const connected = visibleLinks.some(l => 
             (l.source === d.id && l.target === n.id) ||
             (l.target === d.id && l.source === n.id)
           );
           return connected ? 1 : 0.3;
         });
         link.style('opacity', l => 
           l.source === d.id || l.target === d.id ? 1 : 0.3
         );
       })
       .on('mouseleave', () => {
         onNodeHover?.(null);
         node.style('opacity', 1);
         link.style('opacity', 1);
       });
       
       // Simulation tick
       simulation.on('tick', () => {
         link
           .attr('x1', (d: any) => d.source.x)
           .attr('y1', (d: any) => d.source.y)
           .attr('x2', (d: any) => d.target.x)
           .attr('y2', (d: any) => d.target.y);
         
         linkLabel
           .attr('x', (d: any) => (d.source.x + d.target.x) / 2)
           .attr('y', (d: any) => (d.source.y + d.target.y) / 2);
         
         node.attr('transform', (d: any) => `translate(${d.x},${d.y})`);
       });
       
       // Drag functions
       function dragstarted(event: any, d: any) {
         if (!event.active) simulation.alphaTarget(0.3).restart();
         d.fx = d.x;
         d.fy = d.y;
       }
       
       function dragged(event: any, d: any) {
         d.fx = event.x;
         d.fy = event.y;
       }
       
       function dragended(event: any, d: any) {
         if (!event.active) simulation.alphaTarget(0);
         d.fx = null;
         d.fy = null;
       }
       
       // Cleanup
       return () => {
         simulation.stop();
       };
     }, [visibleNodes, visibleLinks, dimensions]);
     
     return (
       <div ref={containerRef} className={cn('relative', className)}>
         <svg
           ref={svgRef}
           className="w-full h-full"
           style={{ cursor: 'grab' }}
         />
         
         {/* Controls */}
         <div className="absolute top-4 right-4 space-y-2">
           <button
             onClick={() => {
               const svg = d3.select(svgRef.current);
               svg.transition().duration(750).call(
                 d3.zoom<SVGSVGElement, unknown>().transform,
                 d3.zoomIdentity
               );
             }}
             className="p-2 bg-gray-800 rounded hover:bg-gray-700"
             title="Reset zoom"
           >
             <Home className="w-4 h-4" />
           </button>
         </div>
       </div>
     );
   }
   ```

### Step 3: Hierarchical Navigation
**Duration**: 3 days

1. **Tree View Component**
   ```typescript
   // src/components/navigation/TreeView.tsx
   import React, { useState, useMemo } from 'react';
   import { ChevronRight, ChevronDown, Folder, FolderOpen, MapPin } from 'lucide-react';
   import { motion, AnimatePresence } from 'framer-motion';
   
   export interface TreeNode<T = any> {
     id: string;
     label: string;
     data: T;
     children?: TreeNode<T>[];
     icon?: React.ReactNode;
     badge?: string | number;
     actions?: TreeNodeAction[];
   }
   
   export interface TreeNodeAction {
     id: string;
     icon: React.ReactNode;
     label: string;
     onClick: (node: TreeNode) => void;
   }
   
   interface TreeViewProps<T> {
     data: TreeNode<T>[];
     selectedId?: string;
     expandedIds?: Set<string>;
     onSelect?: (node: TreeNode<T>) => void;
     onToggle?: (nodeId: string, expanded: boolean) => void;
     searchTerm?: string;
     showActions?: boolean;
     showBadges?: boolean;
     indentSize?: number;
     className?: string;
   }
   
   export function TreeView<T>({
     data,
     selectedId,
     expandedIds: controlledExpandedIds,
     onSelect,
     onToggle,
     searchTerm = '',
     showActions = true,
     showBadges = true,
     indentSize = 24,
     className,
   }: TreeViewProps<T>) {
     const [localExpandedIds, setLocalExpandedIds] = useState<Set<string>>(
       new Set()
     );
     
     const expandedIds = controlledExpandedIds || localExpandedIds;
     
     const filteredData = useMemo(() => {
       if (!searchTerm) return data;
       
       const filterNodes = (nodes: TreeNode<T>[]): TreeNode<T>[] => {
         return nodes.reduce((acc, node) => {
           const matches = node.label.toLowerCase().includes(searchTerm.toLowerCase());
           const filteredChildren = node.children ? filterNodes(node.children) : [];
           
           if (matches || filteredChildren.length > 0) {
             acc.push({
               ...node,
               children: filteredChildren,
             });
           }
           
           return acc;
         }, [] as TreeNode<T>[]);
       };
       
       return filterNodes(data);
     }, [data, searchTerm]);
     
     const handleToggle = (nodeId: string) => {
       const newExpanded = new Set(expandedIds);
       if (newExpanded.has(nodeId)) {
         newExpanded.delete(nodeId);
       } else {
         newExpanded.add(nodeId);
       }
       
       if (onToggle) {
         onToggle(nodeId, !expandedIds.has(nodeId));
       } else {
         setLocalExpandedIds(newExpanded);
       }
     };
     
     return (
       <div className={cn('select-none', className)}>
         {filteredData.map(node => (
           <TreeNodeComponent
             key={node.id}
             node={node}
             depth={0}
             selectedId={selectedId}
             expandedIds={expandedIds}
             onSelect={onSelect}
             onToggle={handleToggle}
             showActions={showActions}
             showBadges={showBadges}
             indentSize={indentSize}
             searchTerm={searchTerm}
           />
         ))}
       </div>
     );
   }
   
   function TreeNodeComponent<T>({
     node,
     depth,
     selectedId,
     expandedIds,
     onSelect,
     onToggle,
     showActions,
     showBadges,
     indentSize,
     searchTerm,
   }: {
     node: TreeNode<T>;
     depth: number;
   } & Omit<TreeViewProps<T>, 'data'>) {
     const hasChildren = node.children && node.children.length > 0;
     const isExpanded = expandedIds.has(node.id);
     const isSelected = selectedId === node.id;
     const [showNodeActions, setShowNodeActions] = useState(false);
     
     // Highlight search matches
     const highlightedLabel = useMemo(() => {
       if (!searchTerm) return node.label;
       
       const regex = new RegExp(`(${searchTerm})`, 'gi');
       const parts = node.label.split(regex);
       
       return parts.map((part, i) => 
         regex.test(part) ? (
           <mark key={i} className="bg-yellow-500/30 text-yellow-300">
             {part}
           </mark>
         ) : (
           part
         )
       );
     }, [node.label, searchTerm]);
     
     return (
       <div>
         <div
           className={cn(
             'flex items-center px-2 py-1 rounded cursor-pointer hover:bg-gray-800',
             isSelected && 'bg-purple-900/30 hover:bg-purple-900/40'
           )}
           style={{ paddingLeft: `${depth * indentSize + 8}px` }}
           onClick={() => onSelect?.(node)}
           onMouseEnter={() => setShowNodeActions(true)}
           onMouseLeave={() => setShowNodeActions(false)}
         >
           {/* Expand/Collapse */}
           {hasChildren && (
             <button
               onClick={(e) => {
                 e.stopPropagation();
                 onToggle(node.id);
               }}
               className="p-0.5 hover:bg-gray-700 rounded"
             >
               {isExpanded ? (
                 <ChevronDown className="w-4 h-4 text-gray-400" />
               ) : (
                 <ChevronRight className="w-4 h-4 text-gray-400" />
               )}
             </button>
           )}
           
           {/* Icon */}
           <div className="ml-1 mr-2 flex-shrink-0">
             {node.icon || (
               hasChildren ? (
                 isExpanded ? (
                   <FolderOpen className="w-4 h-4 text-purple-400" />
                 ) : (
                   <Folder className="w-4 h-4 text-gray-400" />
                 )
               ) : (
                 <MapPin className="w-4 h-4 text-gray-500" />
               )
             )}
           </div>
           
           {/* Label */}
           <span className="flex-1 text-sm text-gray-300 truncate">
             {highlightedLabel}
           </span>
           
           {/* Badge */}
           {showBadges && node.badge !== undefined && (
             <span className="ml-2 px-1.5 py-0.5 bg-gray-700 text-gray-400 text-xs rounded">
               {node.badge}
             </span>
           )}
           
           {/* Actions */}
           {showActions && node.actions && showNodeActions && (
             <div className="ml-2 flex items-center space-x-1">
               {node.actions.map(action => (
                 <button
                   key={action.id}
                   onClick={(e) => {
                     e.stopPropagation();
                     action.onClick(node);
                   }}
                   className="p-1 hover:bg-gray-700 rounded"
                   title={action.label}
                 >
                   {action.icon}
                 </button>
               ))}
             </div>
           )}
         </div>
         
         {/* Children */}
         <AnimatePresence>
           {hasChildren && isExpanded && (
             <motion.div
               initial={{ height: 0, opacity: 0 }}
               animate={{ height: 'auto', opacity: 1 }}
               exit={{ height: 0, opacity: 0 }}
               transition={{ duration: 0.2 }}
             >
               {node.children!.map(child => (
                 <TreeNodeComponent
                   key={child.id}
                   node={child}
                   depth={depth + 1}
                   selectedId={selectedId}
                   expandedIds={expandedIds}
                   onSelect={onSelect}
                   onToggle={onToggle}
                   showActions={showActions}
                   showBadges={showBadges}
                   indentSize={indentSize}
                   searchTerm={searchTerm}
                 />
               ))}
             </motion.div>
           )}
         </AnimatePresence>
       </div>
     );
   }
   ```

### Step 4: Search and Filter Interfaces
**Duration**: 3 days

1. **Advanced Search Component**
   ```typescript
   // src/components/search/AdvancedSearch.tsx
   import React, { useState, useCallback } from 'react';
   import { Search, Filter, X, ChevronDown } from 'lucide-react';
   import { motion, AnimatePresence } from 'framer-motion';
   import { useDebounce } from '@/hooks/useDebounce';
   
   export interface SearchFilter {
     id: string;
     label: string;
     type: 'select' | 'multiselect' | 'range' | 'boolean' | 'date';
     options?: Array<{ value: string; label: string }>;
     value?: any;
   }
   
   export interface SearchFacet {
     id: string;
     label: string;
     values: Array<{
       value: string;
       label: string;
       count: number;
     }>;
   }
   
   interface AdvancedSearchProps {
     placeholder?: string;
     filters: SearchFilter[];
     facets?: SearchFacet[];
     onSearch: (query: string, filters: Record<string, any>) => void;
     onFilterChange?: (filterId: string, value: any) => void;
     showSuggestions?: boolean;
     suggestions?: string[];
     className?: string;
   }
   
   export function AdvancedSearch({
     placeholder = 'Search...',
     filters,
     facets,
     onSearch,
     onFilterChange,
     showSuggestions = true,
     suggestions = [],
     className,
   }: AdvancedSearchProps) {
     const [query, setQuery] = useState('');
     const [showFilters, setShowFilters] = useState(false);
     const [activeFilters, setActiveFilters] = useState<Record<string, any>>({});
     const [showSuggestionsDropdown, setShowSuggestionsDropdown] = useState(false);
     
     const debouncedQuery = useDebounce(query, 300);
     
     // Trigger search on query change
     React.useEffect(() => {
       onSearch(debouncedQuery, activeFilters);
     }, [debouncedQuery, activeFilters]);
     
     const handleFilterChange = (filterId: string, value: any) => {
       setActiveFilters(prev => ({
         ...prev,
         [filterId]: value,
       }));
       onFilterChange?.(filterId, value);
     };
     
     const clearFilter = (filterId: string) => {
       setActiveFilters(prev => {
         const newFilters = { ...prev };
         delete newFilters[filterId];
         return newFilters;
       });
       onFilterChange?.(filterId, undefined);
     };
     
     const activeFilterCount = Object.keys(activeFilters).length;
     
     return (
       <div className={cn('relative', className)}>
         {/* Search Bar */}
         <div className="relative">
           <div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
             <Search className="h-5 w-5 text-gray-400" />
           </div>
           
           <input
             type="text"
             value={query}
             onChange={(e) => {
               setQuery(e.target.value);
               setShowSuggestionsDropdown(true);
             }}
             onFocus={() => setShowSuggestionsDropdown(true)}
             placeholder={placeholder}
             className="block w-full pl-10 pr-20 py-2 bg-gray-800 border border-gray-700 rounded-lg text-white placeholder-gray-400 focus:outline-none focus:border-purple-500"
           />
           
           <div className="absolute inset-y-0 right-0 flex items-center pr-2 space-x-2">
             {query && (
               <button
                 onClick={() => setQuery('')}
                 className="p-1 hover:bg-gray-700 rounded"
               >
                 <X className="h-4 w-4 text-gray-400" />
               </button>
             )}
             
             <button
               onClick={() => setShowFilters(!showFilters)}
               className={cn(
                 'p-1 hover:bg-gray-700 rounded relative',
                 showFilters && 'bg-gray-700'
               )}
             >
               <Filter className="h-4 w-4 text-gray-400" />
               {activeFilterCount > 0 && (
                 <span className="absolute -top-1 -right-1 w-4 h-4 bg-purple-500 text-white text-xs rounded-full flex items-center justify-center">
                   {activeFilterCount}
                 </span>
               )}
             </button>
           </div>
         </div>
         
         {/* Suggestions Dropdown */}
         <AnimatePresence>
           {showSuggestions && showSuggestionsDropdown && suggestions.length > 0 && (
             <motion.div
               initial={{ opacity: 0, y: -10 }}
               animate={{ opacity: 1, y: 0 }}
               exit={{ opacity: 0, y: -10 }}
               className="absolute z-10 w-full mt-1 bg-gray-800 border border-gray-700 rounded-lg shadow-lg"
             >
               {suggestions.map((suggestion, i) => (
                 <button
                   key={i}
                   onClick={() => {
                     setQuery(suggestion);
                     setShowSuggestionsDropdown(false);
                   }}
                   className="w-full px-4 py-2 text-left text-sm text-gray-300 hover:bg-gray-700"
                 >
                   {suggestion}
                 </button>
               ))}
             </motion.div>
           )}
         </AnimatePresence>
         
         {/* Filters Panel */}
         <AnimatePresence>
           {showFilters && (
             <motion.div
               initial={{ opacity: 0, height: 0 }}
               animate={{ opacity: 1, height: 'auto' }}
               exit={{ opacity: 0, height: 0 }}
               className="mt-4 p-4 bg-gray-800 border border-gray-700 rounded-lg"
             >
               {/* Active Filters */}
               {activeFilterCount > 0 && (
                 <div className="mb-4">
                   <div className="flex items-center justify-between mb-2">
                     <h4 className="text-sm font-medium text-gray-400">
                       Active Filters
                     </h4>
                     <button
                       onClick={() => setActiveFilters({})}
                       className="text-xs text-purple-400 hover:text-purple-300"
                     >
                       Clear all
                     </button>
                   </div>
                   <div className="flex flex-wrap gap-2">
                     {Object.entries(activeFilters).map(([filterId, value]) => {
                       const filter = filters.find(f => f.id === filterId);
                       if (!filter) return null;
                       
                       return (
                         <span
                           key={filterId}
                           className="inline-flex items-center px-2 py-1 bg-purple-900/30 text-purple-300 text-sm rounded"
                         >
                           {filter.label}: {formatFilterValue(value)}
                           <button
                             onClick={() => clearFilter(filterId)}
                             className="ml-1 hover:text-purple-100"
                           >
                             <X className="w-3 h-3" />
                           </button>
                         </span>
                       );
                     })}
                   </div>
                 </div>
               )}
               
               {/* Filter Controls */}
               <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                 {filters.map(filter => (
                   <FilterControl
                     key={filter.id}
                     filter={filter}
                     value={activeFilters[filter.id]}
                     onChange={(value) => handleFilterChange(filter.id, value)}
                   />
                 ))}
               </div>
               
               {/* Facets */}
               {facets && facets.length > 0 && (
                 <div className="mt-4 pt-4 border-t border-gray-700">
                   <h4 className="text-sm font-medium text-gray-400 mb-3">
                     Refine by
                   </h4>
                   <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                     {facets.map(facet => (
                       <FacetGroup
                         key={facet.id}
                         facet={facet}
                         selected={activeFilters[facet.id] || []}
                         onChange={(values) => handleFilterChange(facet.id, values)}
                       />
                     ))}
                   </div>
                 </div>
               )}
             </motion.div>
           )}
         </AnimatePresence>
       </div>
     );
   }
   ```

### Step 5: Bulk Operations Interface
**Duration**: 2 days

1. **Bulk Selection Manager**
   ```typescript
   // src/components/bulk/BulkOperations.tsx
   import React, { useState, useMemo } from 'react';
   import { Check, Square, CheckSquare, MoreHorizontal } from 'lucide-react';
   
   interface BulkOperationsProps<T> {
     items: T[];
     selectedIds: Set<string>;
     onSelectionChange: (ids: Set<string>) => void;
     actions: BulkAction<T>[];
     getItemId: (item: T) => string;
     className?: string;
   }
   
   interface BulkAction<T> {
     id: string;
     label: string;
     icon?: React.ReactNode;
     variant?: 'default' | 'danger';
     requireConfirmation?: boolean;
     confirmMessage?: string;
     isDisabled?: (selectedItems: T[]) => boolean;
     execute: (selectedItems: T[]) => Promise<void>;
   }
   
   export function BulkOperations<T>({
     items,
     selectedIds,
     onSelectionChange,
     actions,
     getItemId,
     className,
   }: BulkOperationsProps<T>) {
     const [executing, setExecuting] = useState<string | null>(null);
     const [showActions, setShowActions] = useState(false);
     
     const selectedItems = useMemo(
       () => items.filter(item => selectedIds.has(getItemId(item))),
       [items, selectedIds, getItemId]
     );
     
     const allSelected = items.length > 0 && selectedIds.size === items.length;
     const someSelected = selectedIds.size > 0 && selectedIds.size < items.length;
     
     const handleSelectAll = () => {
       if (allSelected) {
         onSelectionChange(new Set());
       } else {
         onSelectionChange(new Set(items.map(getItemId)));
       }
     };
     
     const handleAction = async (action: BulkAction<T>) => {
       if (action.requireConfirmation) {
         const message = action.confirmMessage || 
           `Are you sure you want to ${action.label.toLowerCase()} ${selectedIds.size} items?`;
         
         if (!confirm(message)) return;
       }
       
       setExecuting(action.id);
       try {
         await action.execute(selectedItems);
         onSelectionChange(new Set()); // Clear selection after success
       } catch (error) {
         console.error(`Failed to execute ${action.label}:`, error);
       } finally {
         setExecuting(null);
       }
     };
     
     if (selectedIds.size === 0) return null;
     
     return (
       <motion.div
         initial={{ opacity: 0, y: 20 }}
         animate={{ opacity: 1, y: 0 }}
         exit={{ opacity: 0, y: 20 }}
         className={cn(
           'fixed bottom-4 left-1/2 transform -translate-x-1/2',
           'bg-gray-900 border border-gray-700 rounded-lg shadow-xl',
           'flex items-center space-x-4 px-4 py-3',
           className
         )}
       >
         {/* Selection Info */}
         <div className="flex items-center space-x-3">
           <button
             onClick={handleSelectAll}
             className="p-1 hover:bg-gray-800 rounded"
           >
             {allSelected ? (
               <CheckSquare className="w-5 h-5 text-purple-400" />
             ) : someSelected ? (
               <Square className="w-5 h-5 text-purple-400" />
             ) : (
               <Square className="w-5 h-5 text-gray-400" />
             )}
           </button>
           
           <span className="text-sm text-gray-300">
             {selectedIds.size} selected
           </span>
         </div>
         
         <div className="h-6 w-px bg-gray-700" />
         
         {/* Actions */}
         <div className="flex items-center space-x-2">
           {actions.slice(0, 3).map(action => {
             const isDisabled = action.isDisabled?.(selectedItems) || executing !== null;
             
             return (
               <button
                 key={action.id}
                 onClick={() => handleAction(action)}
                 disabled={isDisabled}
                 className={cn(
                   'px-3 py-1 rounded text-sm font-medium',
                   'transition-colors',
                   action.variant === 'danger'
                     ? 'bg-red-900/50 text-red-300 hover:bg-red-900/70'
                     : 'bg-gray-800 text-gray-300 hover:bg-gray-700',
                   isDisabled && 'opacity-50 cursor-not-allowed',
                   executing === action.id && 'animate-pulse'
                 )}
               >
                 {action.icon && (
                   <span className="mr-1">{action.icon}</span>
                 )}
                 {action.label}
               </button>
             );
           })}
           
           {actions.length > 3 && (
             <div className="relative">
               <button
                 onClick={() => setShowActions(!showActions)}
                 className="p-1 hover:bg-gray-800 rounded"
               >
                 <MoreHorizontal className="w-5 h-5 text-gray-400" />
               </button>
               
               <AnimatePresence>
                 {showActions && (
                   <motion.div
                     initial={{ opacity: 0, scale: 0.95 }}
                     animate={{ opacity: 1, scale: 1 }}
                     exit={{ opacity: 0, scale: 0.95 }}
                     className="absolute bottom-full right-0 mb-2 w-48 bg-gray-800 border border-gray-700 rounded-lg shadow-lg"
                   >
                     {actions.slice(3).map(action => {
                       const isDisabled = action.isDisabled?.(selectedItems) || executing !== null;
                       
                       return (
                         <button
                           key={action.id}
                           onClick={() => handleAction(action)}
                           disabled={isDisabled}
                           className={cn(
                             'w-full px-4 py-2 text-left text-sm',
                             'hover:bg-gray-700',
                             isDisabled && 'opacity-50 cursor-not-allowed'
                           )}
                         >
                           {action.icon && (
                             <span className="mr-2">{action.icon}</span>
                           )}
                           {action.label}
                         </button>
                       );
                     })}
                   </motion.div>
                 )}
               </AnimatePresence>
             </div>
           )}
         </div>
       </motion.div>
     );
   }
   ```

## Testing Strategy

1. **Component Testing**
   - Unit tests for all components
   - Visual regression testing
   - Interaction testing
   - Accessibility testing

2. **Performance Testing**
   - Render performance with 1000+ items
   - Scroll performance
   - Animation smoothness
   - Memory usage

3. **Cross-browser Testing**
   - Chrome, Firefox, Safari, Edge
   - Mobile responsiveness
   - Touch interactions

## Success Metrics

- Component render time <16ms
- Smooth 60fps animations
- WCAG AA accessibility compliance
- Zero visual regressions
- 95% code coverage

## Next Phase
Proceed to [UPGRADE_PLAN_P8.md](./UPGRADE_PLAN_P8.md) for Performance Optimization.