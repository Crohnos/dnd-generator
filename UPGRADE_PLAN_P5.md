# UPGRADE PLAN PHASE 5: FRONTEND FORM COMPLEXITY

## Overview
This phase focuses on expanding the frontend form system from a simple 3-step wizard to a comprehensive intake process that captures detailed player character backstories and world-building preferences to generate a lived-in world.

## Current State
- 3-step wizard (name, characters, themes)
- Basic form validation
- Simple state management
- Limited data types

## Target State
- Multi-phase intake process focused on PC backstories
- Conditional form rendering based on character details
- Dynamic form generation from schema
- Backstory element extraction
- Import/export functionality
- Advanced validation and dependencies
- Form state persistence

## Implementation Steps

### Step 1: Advanced Form Architecture
**Duration**: 4 days

1. **Form Schema Definition**
   ```typescript
   // src/lib/forms/schema.ts
   export interface FormSchema {
     id: string;
     title: string;
     description: string;
     sections: FormSection[];
     dependencies: FormDependency[];
     validation: ValidationSchema;
   }
   
   export interface FormSection {
     id: string;
     title: string;
     type: 'single' | 'collection' | 'nested';
     fields: FormField[];
     conditions?: DisplayCondition[];
     minItems?: number;
     maxItems?: number;
   }
   
   export interface FormField {
     id: string;
     label: string;
     type: FieldType;
     required: boolean;
     defaultValue?: any;
     options?: FieldOption[];
     validation?: FieldValidation[];
     conditions?: DisplayCondition[];
     metadata?: Record<string, any>;
   }
   
   export type FieldType = 
     | 'text' 
     | 'textarea' 
     | 'number' 
     | 'select' 
     | 'multiselect'
     | 'checkbox' 
     | 'radio' 
     | 'date' 
     | 'relationship'
     | 'tags' 
     | 'json' 
     | 'rich-text'
     | 'file-upload';
   
   export interface DisplayCondition {
     field: string;
     operator: 'equals' | 'not_equals' | 'contains' | 'greater_than' | 'less_than';
     value: any;
   }
   
   export const campaignFormSchema: FormSchema = {
     id: 'campaign_creation',
     title: 'Create Your D&D Campaign',
     description: 'Complete campaign generation wizard',
     sections: [
       {
         id: 'basics',
         title: 'Campaign Basics',
         type: 'single',
         fields: [
           {
             id: 'name',
             label: 'Campaign Name',
             type: 'text',
             required: true,
             validation: [
               { type: 'min_length', value: 3 },
               { type: 'max_length', value: 100 }
             ]
           },
           {
             id: 'genre',
             label: 'Campaign Genre',
             type: 'select',
             required: true,
             options: [
               { value: 'high_fantasy', label: 'High Fantasy' },
               { value: 'dark_fantasy', label: 'Dark Fantasy' },
               { value: 'sword_sorcery', label: 'Sword & Sorcery' },
               { value: 'mythic', label: 'Mythic' },
               { value: 'horror', label: 'Horror' },
             ]
           },
           {
             id: 'world_building_focus',
             label: 'World-Building Focus',
             type: 'radio',
             required: true,
             defaultValue: 'pc_centric',
             options: [
               { value: 'pc_centric', label: 'PC-Centric', description: 'Build the world around PC backstories' },
               { value: 'balanced', label: 'Balanced', description: 'Mix PC elements with broader world' },
               { value: 'expansive', label: 'Expansive', description: 'Rich world with PC connections' }
             ]
           }
         ]
       },
       {
         id: 'world_parameters',
         title: 'World Parameters',
         type: 'single',
         fields: [
           {
             id: 'magic_level',
             label: 'Magic Level',
             type: 'radio',
             required: true,
             options: [
               { value: 'low', label: 'Low Magic', description: 'Magic is rare and mysterious' },
               { value: 'medium', label: 'Medium Magic', description: 'Magic is uncommon but known' },
               { value: 'high', label: 'High Magic', description: 'Magic is common and integrated' },
               { value: 'very_high', label: 'Very High Magic', description: 'Magic permeates everything' }
             ]
           },
           {
             id: 'technology_level',
             label: 'Technology Level',
             type: 'select',
             required: true,
             options: [
               { value: 'stone_age', label: 'Stone Age' },
               { value: 'bronze_age', label: 'Bronze Age' },
               { value: 'iron_age', label: 'Iron Age' },
               { value: 'medieval', label: 'Medieval' },
               { value: 'renaissance', label: 'Renaissance' },
               { value: 'industrial', label: 'Early Industrial' }
             ]
           }
         ]
       },
       {
         id: 'player_characters',
         title: 'Player Characters',
         type: 'collection',
         minItems: 1,
         maxItems: 8,
         fields: [
           {
             id: 'name',
             label: 'Character Name',
             type: 'text',
             required: true
           },
           {
             id: 'backstory',
             label: 'Character Backstory',
             type: 'rich-text',
             required: true,
             metadata: {
               minLength: 100,
               placeholder: 'Describe your character\'s history, important people, places, and events...'
             }
           },
           {
             id: 'important_npcs',
             label: 'Important NPCs from Backstory',
             type: 'tags',
             metadata: {
               placeholder: 'Family members, mentors, rivals, friends...'
             }
           },
           {
             id: 'important_locations',
             label: 'Important Locations',
             type: 'tags',
             metadata: {
               placeholder: 'Hometown, training grounds, significant places...'
             }
           },
           {
             id: 'goals',
             label: 'Character Goals',
             type: 'textarea',
             required: true
           },
           {
             id: 'fears',
             label: 'Character Fears/Weaknesses',
             type: 'textarea'
           }
         ]
       },
       // ... additional sections
     ],
     dependencies: [],
     validation: {}
   };
   ```

2. **Dynamic Form Renderer**
   ```typescript
   // src/components/forms/DynamicForm.tsx
   import React, { useMemo } from 'react';
   import { useFormContext } from 'react-hook-form';
   import { FormSection, FormField } from '@/lib/forms/schema';
   import { evaluateConditions } from '@/lib/forms/conditions';
   
   interface DynamicFormSectionProps {
     section: FormSection;
     values: Record<string, any>;
   }
   
   export const DynamicFormSection: React.FC<DynamicFormSectionProps> = ({
     section,
     values
   }) => {
     const { register, control, formState: { errors } } = useFormContext();
     
     const visibleFields = useMemo(() => {
       return section.fields.filter(field => 
         !field.conditions || evaluateConditions(field.conditions, values)
       );
     }, [section.fields, values]);
     
     if (section.type === 'collection') {
       return (
         <CollectionSection
           section={section}
           visibleFields={visibleFields}
           control={control}
           errors={errors}
         />
       );
     }
     
     return (
       <div className="space-y-6">
         <div>
           <h3 className="text-xl font-semibold text-purple-300">
             {section.title}
           </h3>
           {section.description && (
             <p className="text-gray-400 mt-1">{section.description}</p>
           )}
         </div>
         
         <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
           {visibleFields.map(field => (
             <DynamicField
               key={field.id}
               field={field}
               register={register}
               control={control}
               error={errors[field.id]}
             />
           ))}
         </div>
       </div>
     );
   };
   
   const DynamicField: React.FC<{
     field: FormField;
     register: any;
     control: any;
     error: any;
   }> = ({ field, register, control, error }) => {
     switch (field.type) {
       case 'text':
         return (
           <TextField
             {...register(field.id, field.validation)}
             label={field.label}
             required={field.required}
             error={error}
           />
         );
       
       case 'select':
         return (
           <SelectField
             field={field}
             control={control}
             error={error}
           />
         );
       
       case 'multiselect':
         return (
           <MultiSelectField
             field={field}
             control={control}
             error={error}
           />
         );
       
       case 'relationship':
         return (
           <RelationshipField
             field={field}
             control={control}
             error={error}
           />
         );
       
       case 'tags':
         return (
           <TagsField
             field={field}
             control={control}
             error={error}
           />
         );
       
       // ... more field types
       
       default:
         return null;
     }
   };
   ```

### Step 2: Collection Management
**Duration**: 4 days

1. **Collection Field Components**
   ```typescript
   // src/components/forms/CollectionField.tsx
   import { useFieldArray } from 'react-hook-form';
   import { DndProvider, useDrag, useDrop } from 'react-dnd';
   import { HTML5Backend } from 'react-dnd-html5-backend';
   
   interface CollectionFieldProps {
     name: string;
     fields: FormField[];
     minItems?: number;
     maxItems?: number;
     itemTemplate?: any;
   }
   
   export const CollectionField: React.FC<CollectionFieldProps> = ({
     name,
     fields,
     minItems = 0,
     maxItems,
     itemTemplate
   }) => {
     const { fields: items, append, remove, move } = useFieldArray({ name });
     
     const addItem = () => {
       if (!maxItems || items.length < maxItems) {
         append(itemTemplate || createDefaultItem(fields));
       }
     };
     
     const canRemove = items.length > minItems;
     const canAdd = !maxItems || items.length < maxItems;
     
     return (
       <DndProvider backend={HTML5Backend}>
         <div className="space-y-4">
           <div className="flex justify-between items-center">
             <h4 className="text-lg font-medium text-purple-300">
               {name} ({items.length})
             </h4>
             {canAdd && (
               <button
                 type="button"
                 onClick={addItem}
                 className="btn-secondary text-sm"
               >
                 <Plus className="w-4 h-4 mr-1" />
                 Add Item
               </button>
             )}
           </div>
           
           <div className="space-y-3">
             {items.map((item, index) => (
               <DraggableItem
                 key={item.id}
                 index={index}
                 item={item}
                 fields={fields}
                 onRemove={canRemove ? () => remove(index) : undefined}
                 onMove={(dragIndex, dropIndex) => move(dragIndex, dropIndex)}
               />
             ))}
           </div>
           
           {items.length === 0 && (
             <div className="text-center py-8 text-gray-500">
               No items yet. Click "Add Item" to start.
             </div>
           )}
         </div>
       </DndProvider>
     );
   };
   
   const DraggableItem: React.FC<{
     index: number;
     item: any;
     fields: FormField[];
     onRemove?: () => void;
     onMove: (from: number, to: number) => void;
   }> = ({ index, item, fields, onRemove, onMove }) => {
     const [{ isDragging }, drag] = useDrag({
       type: 'collection-item',
       item: { index },
       collect: (monitor) => ({
         isDragging: monitor.isDragging(),
       }),
     });
     
     const [, drop] = useDrop({
       accept: 'collection-item',
       hover: (draggedItem: { index: number }) => {
         if (draggedItem.index !== index) {
           onMove(draggedItem.index, index);
           draggedItem.index = index;
         }
       },
     });
     
     return (
       <div
         ref={(node) => drag(drop(node))}
         className={`bg-gray-800 rounded-lg p-4 ${
           isDragging ? 'opacity-50' : ''
         }`}
       >
         <div className="flex justify-between items-start mb-3">
           <div className="flex items-center">
             <GripVertical className="w-5 h-5 text-gray-500 mr-2 cursor-move" />
             <span className="text-sm text-gray-400">Item {index + 1}</span>
           </div>
           {onRemove && (
             <button
               type="button"
               onClick={onRemove}
               className="text-red-500 hover:text-red-400"
             >
               <X className="w-5 h-5" />
             </button>
           )}
         </div>
         
         <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
           {fields.map(field => (
             <DynamicField
               key={field.id}
               field={{
                 ...field,
                 id: `${name}.${index}.${field.id}`
               }}
             />
           ))}
         </div>
       </div>
     );
   };
   ```

2. **Bulk Import/Export**
   ```typescript
   // src/components/forms/BulkImport.tsx
   import Papa from 'papaparse';
   import { validateBulkData } from '@/lib/forms/validation';
   
   interface BulkImportProps {
     fields: FormField[];
     onImport: (data: any[]) => void;
     template?: string;
   }
   
   export const BulkImport: React.FC<BulkImportProps> = ({
     fields,
     onImport,
     template
   }) => {
     const [isImporting, setIsImporting] = useState(false);
     const [errors, setErrors] = useState<string[]>([]);
     
     const handleFileUpload = async (file: File) => {
       setIsImporting(true);
       setErrors([]);
       
       try {
         const text = await file.text();
         const extension = file.name.split('.').pop()?.toLowerCase();
         
         let data: any[];
         
         if (extension === 'csv') {
           const result = Papa.parse(text, {
             header: true,
             dynamicTyping: true,
             skipEmptyLines: true
           });
           data = result.data;
         } else if (extension === 'json') {
           data = JSON.parse(text);
         } else {
           throw new Error('Unsupported file format');
         }
         
         // Validate data
         const validationResult = validateBulkData(data, fields);
         if (!validationResult.valid) {
           setErrors(validationResult.errors);
           return;
         }
         
         // Transform data to match form structure
         const transformed = transformBulkData(data, fields);
         onImport(transformed);
         
       } catch (error) {
         setErrors([error.message]);
       } finally {
         setIsImporting(false);
       }
     };
     
     const downloadTemplate = () => {
       const headers = fields.map(f => f.label);
       const sampleRow = fields.map(f => {
         switch (f.type) {
           case 'text': return 'Sample Text';
           case 'number': return 1;
           case 'select': return f.options?.[0]?.value || '';
           case 'multiselect': return f.options?.slice(0, 2).map(o => o.value).join(',') || '';
           default: return '';
         }
       });
       
       const csv = Papa.unparse({
         fields: headers,
         data: [sampleRow]
       });
       
       const blob = new Blob([csv], { type: 'text/csv' });
       const url = URL.createObjectURL(blob);
       const a = document.createElement('a');
       a.href = url;
       a.download = template || 'import-template.csv';
       a.click();
     };
     
     return (
       <div className="bg-gray-800 rounded-lg p-6">
         <h3 className="text-lg font-semibold text-purple-300 mb-4">
           Bulk Import
         </h3>
         
         <div className="space-y-4">
           <div className="flex gap-4">
             <label className="btn-primary cursor-pointer">
               <Upload className="w-5 h-5 mr-2" />
               Choose File
               <input
                 type="file"
                 accept=".csv,.json"
                 onChange={(e) => e.target.files?.[0] && handleFileUpload(e.target.files[0])}
                 className="hidden"
               />
             </label>
             
             <button
               type="button"
               onClick={downloadTemplate}
               className="btn-secondary"
             >
               <Download className="w-5 h-5 mr-2" />
               Download Template
             </button>
           </div>
           
           {errors.length > 0 && (
             <div className="bg-red-900/20 border border-red-500/50 rounded p-4">
               <h4 className="text-red-400 font-medium mb-2">Import Errors:</h4>
               <ul className="list-disc list-inside text-sm text-red-300">
                 {errors.map((error, i) => (
                   <li key={i}>{error}</li>
                 ))}
               </ul>
             </div>
           )}
           
           {isImporting && (
             <div className="flex items-center justify-center py-8">
               <Loader className="w-6 h-6 animate-spin text-purple-400" />
               <span className="ml-2 text-gray-400">Processing file...</span>
             </div>
           )}
         </div>
       </div>
     );
   };
   ```

### Step 3: Advanced Validation
**Duration**: 3 days

1. **Cross-Field Validation**
   ```typescript
   // src/lib/forms/validation.ts
   import { z } from 'zod';
   
   export interface ValidationRule {
     type: 'required' | 'min' | 'max' | 'pattern' | 'custom' | 'cross_field';
     value?: any;
     message?: string;
     fields?: string[];
     validator?: (value: any, allValues: any) => boolean | string;
   }
   
   export class FormValidator {
     private schema: z.ZodSchema;
     private crossFieldValidators: Map<string, ValidationRule[]>;
     
     constructor(formSchema: FormSchema) {
       this.schema = this.buildZodSchema(formSchema);
       this.crossFieldValidators = this.buildCrossFieldValidators(formSchema);
     }
     
     async validate(data: any): Promise<ValidationResult> {
       try {
         // Zod validation
         const validated = await this.schema.parseAsync(data);
         
         // Cross-field validation
         const crossFieldErrors = this.validateCrossFields(validated);
         
         if (crossFieldErrors.length > 0) {
           return {
             valid: false,
             errors: crossFieldErrors,
             data: validated
           };
         }
         
         return {
           valid: true,
           errors: [],
           data: validated
         };
       } catch (error) {
         if (error instanceof z.ZodError) {
           return {
             valid: false,
             errors: error.errors.map(e => ({
               field: e.path.join('.'),
               message: e.message
             })),
             data: null
           };
         }
         throw error;
       }
     }
     
     private validateCrossFields(data: any): ValidationError[] {
       const errors: ValidationError[] = [];
       
       for (const [field, rules] of this.crossFieldValidators) {
         for (const rule of rules) {
           if (rule.type === 'cross_field' && rule.validator) {
             const result = rule.validator(
               this.getFieldValue(data, field),
               data
             );
             
             if (typeof result === 'string') {
               errors.push({
                 field,
                 message: result
               });
             } else if (!result) {
               errors.push({
                 field,
                 message: rule.message || 'Validation failed'
               });
             }
           }
         }
       }
       
       return errors;
     }
     
     private getFieldValue(data: any, path: string): any {
       return path.split('.').reduce((obj, key) => obj?.[key], data);
     }
   }
   
   // Example validators
   export const campaignValidators = {
     validatePillarBalance: (value: any, allValues: any) => {
       const { combat, social, exploration } = allValues.pillar_balance || {};
       const total = (combat || 0) + (social || 0) + (exploration || 0);
       
       if (total !== 100) {
         return `Pillar balance must total 100% (currently ${total}%)`;
       }
       return true;
     },
     
     validatePlayerCharacterUniqueness: (value: any, allValues: any) => {
       const characters = allValues.player_characters || [];
       const names = characters.map(c => c.name.toLowerCase());
       const uniqueNames = new Set(names);
       
       if (names.length !== uniqueNames.size) {
         return 'Player character names must be unique';
       }
       return true;
     },
     
     validateThemeCompatibility: (value: any, allValues: any) => {
       const incompatible = {
         'comedy': ['horror', 'grimdark'],
         'high_magic': ['no_magic'],
         'political_intrigue': ['pure_dungeon_crawl']
       };
       
       const themes = allValues.themes || [];
       
       for (const [theme, conflicts] of Object.entries(incompatible)) {
         if (themes.includes(theme)) {
           const found = conflicts.filter(c => themes.includes(c));
           if (found.length > 0) {
             return `Theme "${theme}" conflicts with "${found.join(', ')}"`;
           }
         }
       }
       
       return true;
     }
   };
   ```

### Step 4: Form State Persistence
**Duration**: 2 days

1. **Auto-Save System**
   ```typescript
   // src/hooks/useFormPersistence.ts
   import { useEffect, useRef, useCallback } from 'react';
   import { debounce } from 'lodash';
   import localforage from 'localforage';
   
   interface FormPersistenceOptions {
     key: string;
     debounceMs?: number;
     excludeFields?: string[];
   }
   
   export function useFormPersistence(
     formData: any,
     options: FormPersistenceOptions
   ) {
     const { key, debounceMs = 1000, excludeFields = [] } = options;
     const lastSaved = useRef<Date | null>(null);
     const [isRestoring, setIsRestoring] = useState(true);
     const [lastSaveTime, setLastSaveTime] = useState<Date | null>(null);
     
     // Create storage instance
     const storage = localforage.createInstance({
       name: 'campaign-forms',
       storeName: 'drafts'
     });
     
     // Save function
     const save = useCallback(async (data: any) => {
       try {
         const filtered = filterExcludedFields(data, excludeFields);
         const saveData = {
           data: filtered,
           timestamp: new Date(),
           version: '1.0'
         };
         
         await storage.setItem(key, saveData);
         lastSaved.current = new Date();
         setLastSaveTime(new Date());
         
       } catch (error) {
         console.error('Failed to save form data:', error);
       }
     }, [key, excludeFields]);
     
     // Debounced save
     const debouncedSave = useMemo(
       () => debounce(save, debounceMs),
       [save, debounceMs]
     );
     
     // Auto-save on changes
     useEffect(() => {
       if (!isRestoring && formData) {
         debouncedSave(formData);
       }
     }, [formData, debouncedSave, isRestoring]);
     
     // Restore on mount
     const restore = useCallback(async () => {
       try {
         const saved = await storage.getItem<any>(key);
         if (saved && saved.data) {
           setLastSaveTime(saved.timestamp);
           return saved.data;
         }
       } catch (error) {
         console.error('Failed to restore form data:', error);
       } finally {
         setIsRestoring(false);
       }
       return null;
     }, [key]);
     
     // Clear saved data
     const clear = useCallback(async () => {
       try {
         await storage.removeItem(key);
         setLastSaveTime(null);
       } catch (error) {
         console.error('Failed to clear form data:', error);
       }
     }, [key]);
     
     // Export data
     const exportData = useCallback(async () => {
       const data = {
         ...formData,
         _metadata: {
           exported_at: new Date(),
           form_key: key
         }
       };
       
       const blob = new Blob(
         [JSON.stringify(data, null, 2)],
         { type: 'application/json' }
       );
       
       const url = URL.createObjectURL(blob);
       const a = document.createElement('a');
       a.href = url;
       a.download = `campaign-draft-${Date.now()}.json`;
       a.click();
       URL.revokeObjectURL(url);
     }, [formData, key]);
     
     return {
       restore,
       clear,
       exportData,
       lastSaveTime,
       isRestoring,
       isSaved: !!lastSaveTime
     };
   }
   
   // Usage in form component
   export const CampaignWizard: React.FC = () => {
     const form = useForm<CampaignFormData>();
     const formData = form.watch();
     
     const {
       restore,
       clear,
       exportData,
       lastSaveTime,
       isRestoring
     } = useFormPersistence(formData, {
       key: 'campaign-wizard-draft',
       excludeFields: ['temp_calculations']
     });
     
     // Restore on mount
     useEffect(() => {
       restore().then(data => {
         if (data) {
           form.reset(data);
           toast.info('Draft restored from previous session');
         }
       });
     }, []);
     
     return (
       <div>
         {lastSaveTime && (
           <div className="text-sm text-gray-500">
             Auto-saved {formatRelativeTime(lastSaveTime)}
           </div>
         )}
         {/* Form content */}
       </div>
     );
   };
   ```

### Step 5: Progressive Disclosure
**Duration**: 2 days

1. **Conditional Sections**
   ```typescript
   // src/components/forms/ConditionalSection.tsx
   interface ConditionalSectionProps {
     conditions: DisplayCondition[];
     values: Record<string, any>;
     children: React.ReactNode;
     fallback?: React.ReactNode;
   }
   
   export const ConditionalSection: React.FC<ConditionalSectionProps> = ({
     conditions,
     values,
     children,
     fallback
   }) => {
     const isVisible = useMemo(() => {
       return evaluateConditions(conditions, values);
     }, [conditions, values]);
     
     if (!isVisible) {
       return fallback ? <>{fallback}</> : null;
     }
     
     return (
       <motion.div
         initial={{ opacity: 0, height: 0 }}
         animate={{ opacity: 1, height: 'auto' }}
         exit={{ opacity: 0, height: 0 }}
         transition={{ duration: 0.3 }}
       >
         {children}
       </motion.div>
     );
   };
   
   // Complex form with progressive disclosure
   export const WorldBuildingForm: React.FC = () => {
     const { watch } = useFormContext();
     const values = watch();
     
     return (
       <div className="space-y-8">
         <FormSection>
           <SelectField
             name="cosmology_type"
             label="Cosmology Type"
             options={[
               { value: 'standard', label: 'Standard D&D Cosmology' },
               { value: 'custom', label: 'Custom Cosmology' },
               { value: 'simplified', label: 'Simplified (Material Plane Only)' }
             ]}
           />
         </FormSection>
         
         <ConditionalSection
           conditions={[
             { field: 'cosmology_type', operator: 'equals', value: 'custom' }
           ]}
           values={values}
         >
           <FormSection title="Custom Planes">
             <CollectionField
               name="custom_planes"
               minItems={1}
               maxItems={20}
               fields={[
                 {
                   id: 'name',
                   label: 'Plane Name',
                   type: 'text',
                   required: true
                 },
                 {
                   id: 'description',
                   label: 'Description',
                   type: 'textarea',
                   required: true
                 },
                 {
                   id: 'alignment',
                   label: 'Alignment',
                   type: 'select',
                   options: alignmentOptions
                 }
               ]}
             />
           </FormSection>
         </ConditionalSection>
         
         <ConditionalSection
           conditions={[
             { field: 'cosmology_type', operator: 'not_equals', value: 'simplified' }
           ]}
           values={values}
         >
           <FormSection title="Deities & Pantheons">
             <NumberField
               name="deity_count"
               label="Number of Major Deities"
               min={0}
               max={50}
             />
             
             <ConditionalSection
               conditions={[
                 { field: 'deity_count', operator: 'greater_than', value: 0 }
               ]}
               values={values}
             >
               <PantheonBuilder deityCount={values.deity_count} />
             </ConditionalSection>
           </FormSection>
         </ConditionalSection>
       </div>
     );
   };
   ```

## Testing Strategy

1. **Form Validation Testing**
   - Unit tests for all validators
   - Cross-field validation scenarios
   - Error message clarity

2. **User Experience Testing**
   - Form completion time tracking
   - Error recovery testing
   - Accessibility compliance

3. **Performance Testing**
   - Large collection handling
   - Auto-save performance
   - Conditional rendering efficiency

## Success Metrics

- Form completion rate >80%
- Average time to complete <15 minutes
- Auto-save reliability >99.9%
- Zero data loss incidents
- Validation error clarity score >4.5/5

## Next Phase
Proceed to [UPGRADE_PLAN_P6.md](./UPGRADE_PLAN_P6.md) for State Management Scaling.