import { useState, KeyboardEvent } from 'react';
import { X, Plus } from 'lucide-react';

interface TagInputProps {
  tags: string[];
  onChange: (tags: string[]) => void;
  placeholder?: string;
  suggestions?: string[];
  maxTags?: number;
  className?: string;
}

export function TagInput({ 
  tags, 
  onChange, 
  placeholder = "Add tags...", 
  suggestions = [], 
  maxTags,
  className = ""
}: TagInputProps) {
  const [inputValue, setInputValue] = useState('');
  const [showSuggestions, setShowSuggestions] = useState(false);

  const filteredSuggestions = suggestions.filter(s => 
    s.toLowerCase().includes(inputValue.toLowerCase()) &&
    !tags.includes(s)
  );

  const addTag = (tag: string) => {
    const trimmedTag = tag.trim();
    if (trimmedTag && !tags.includes(trimmedTag) && (!maxTags || tags.length < maxTags)) {
      onChange([...tags, trimmedTag]);
      setInputValue('');
      setShowSuggestions(false);
    }
  };

  const removeTag = (tagToRemove: string) => {
    onChange(tags.filter(tag => tag !== tagToRemove));
  };

  const handleKeyDown = (e: KeyboardEvent<HTMLInputElement>) => {
    if (e.key === 'Enter' || e.key === ',') {
      e.preventDefault();
      addTag(inputValue);
    } else if (e.key === 'Backspace' && inputValue === '' && tags.length > 0) {
      removeTag(tags[tags.length - 1]);
    }
  };

  const handleInputChange = (value: string) => {
    setInputValue(value);
    setShowSuggestions(value.length > 0 && suggestions.length > 0);
  };

  return (
    <div className={`relative ${className}`}>
      {/* Tags Display */}
      <div className="min-h-[42px] p-2 bg-gray-900 border border-gray-700 rounded-lg focus-within:border-dnd-purple focus-within:ring-1 focus-within:ring-dnd-purple">
        <div className="flex flex-wrap gap-2">
          {tags.map((tag, index) => (
            <span
              key={index}
              className="inline-flex items-center gap-1 px-2 py-1 bg-dnd-purple bg-opacity-20 text-dnd-purple text-sm rounded border border-dnd-purple border-opacity-30"
            >
              {tag}
              <button
                onClick={() => removeTag(tag)}
                className="text-dnd-purple hover:text-white transition-colors"
              >
                <X className="w-3 h-3" />
              </button>
            </span>
          ))}
          
          {/* Input Field */}
          {(!maxTags || tags.length < maxTags) && (
            <input
              type="text"
              value={inputValue}
              onChange={(e) => handleInputChange(e.target.value)}
              onKeyDown={handleKeyDown}
              onFocus={() => setShowSuggestions(inputValue.length > 0 && suggestions.length > 0)}
              onBlur={() => setTimeout(() => setShowSuggestions(false), 150)}
              placeholder={tags.length === 0 ? placeholder : ""}
              className="flex-1 min-w-[120px] bg-transparent border-none outline-none text-white placeholder-gray-400"
            />
          )}
        </div>
      </div>

      {/* Add Button (for touch devices) */}
      {inputValue.trim() && (
        <button
          onClick={() => addTag(inputValue)}
          className="absolute right-2 top-1/2 -translate-y-1/2 p-1 text-dnd-purple hover:text-white transition-colors"
        >
          <Plus className="w-4 h-4" />
        </button>
      )}

      {/* Suggestions Dropdown */}
      {showSuggestions && filteredSuggestions.length > 0 && (
        <div className="absolute z-10 w-full mt-1 bg-gray-800 border border-gray-700 rounded-lg shadow-lg max-h-48 overflow-y-auto">
          {filteredSuggestions.slice(0, 8).map((suggestion, index) => (
            <button
              key={index}
              onClick={() => addTag(suggestion)}
              className="w-full px-3 py-2 text-left text-gray-300 hover:bg-gray-700 hover:text-white transition-colors"
            >
              {suggestion}
            </button>
          ))}
        </div>
      )}
      
      {/* Helper Text */}
      <div className="text-xs text-gray-400 mt-1 flex justify-between">
        <span>
          {tags.length === 0 
            ? "Press Enter or comma to add tags" 
            : maxTags 
              ? `${tags.length}/${maxTags} tags` 
              : `${tags.length} tags`
          }
        </span>
        {maxTags && tags.length >= maxTags && (
          <span className="text-yellow-400">Maximum reached</span>
        )}
      </div>
    </div>
  );
}