import { MapPin, Eye, EyeOff } from 'lucide-react';
import { useState } from 'react';

interface LocationCardProps {
  location: {
    id: number;
    name: string;
    location_type: string;
    parent_location_id?: number | null;
    geography_region_id?: number | null;
    description?: string | null;
    population?: number | null;
    government_type?: string | null;
    notable_features?: string[] | null;
    climate?: string | null;
    coordinates?: any;
    size_category?: string | null;
    accessibility?: string[] | null;
    security_level?: string | null;
    wealth_level?: string | null;
    pc_significance?: string | null;
  };
}

export function LocationCard({ location }: LocationCardProps) {
  const [showSecrets, setShowSecrets] = useState(false);

  const getLocationIcon = (type: string) => {
    // Color-code by location type
    const colors = {
      'city': 'text-blue-400',
      'district': 'text-blue-300',
      'building': 'text-gray-400',
      'dungeon': 'text-red-400',
      'wilderness': 'text-green-400',
      'castle': 'text-purple-400',
      'temple': 'text-yellow-400',
      'tavern': 'text-orange-400',
      'shop': 'text-emerald-400',
      'tower': 'text-indigo-400',
    } as const;
    
    return colors[type.toLowerCase() as keyof typeof colors] || 'text-dnd-gold';
  };

  return (
    <div className="card hover:border-dnd-gold transition-colors">
      {/* Header */}
      <div className="flex items-start space-x-4 mb-4">
        <div className="flex-shrink-0">
          <div className={`w-12 h-12 ${getLocationIcon(location.location_type)} bg-opacity-20 rounded-full flex items-center justify-center`}
               style={{ backgroundColor: 'currentColor', opacity: 0.1 }}>
            <MapPin className={`w-6 h-6 ${getLocationIcon(location.location_type)}`} />
          </div>
        </div>
        <div className="flex-grow">
          <h3 className="text-lg font-bold text-white mb-1">{location.name}</h3>
          <div className="flex items-center space-x-2">
            <span className="inline-block px-2 py-1 bg-dnd-gold bg-opacity-10 text-dnd-gold text-xs rounded-full">
              {location.location_type}
            </span>
            {location.size_category && (
              <span className="inline-block px-2 py-1 bg-gray-700 text-gray-300 text-xs rounded-full">
                {location.size_category}
              </span>
            )}
          </div>
        </div>
      </div>

      {/* Description */}
      {location.description && (
        <p className="text-gray-300 mb-4 leading-relaxed">{location.description}</p>
      )}

      {/* Location Details */}
      <div className="grid grid-cols-2 gap-4 mb-4 text-sm">
        {location.population && (
          <div>
            <span className="text-gray-400">Population:</span>
            <span className="text-white ml-2">{location.population.toLocaleString()}</span>
          </div>
        )}
        {location.government_type && (
          <div>
            <span className="text-gray-400">Government:</span>
            <span className="text-white ml-2">{location.government_type}</span>
          </div>
        )}
        {location.climate && (
          <div>
            <span className="text-gray-400">Climate:</span>
            <span className="text-white ml-2">{location.climate}</span>
          </div>
        )}
        {location.security_level && (
          <div>
            <span className="text-gray-400">Security:</span>
            <span className="text-white ml-2">{location.security_level}</span>
          </div>
        )}
        {location.wealth_level && (
          <div>
            <span className="text-gray-400">Wealth:</span>
            <span className="text-white ml-2">{location.wealth_level}</span>
          </div>
        )}
      </div>

      {/* Notable Features */}
      {location.notable_features && location.notable_features.length > 0 && (
        <div className="mb-4">
          <h4 className="text-sm font-semibold text-gray-400 mb-2">Notable Features</h4>
          <ul className="text-gray-300 text-sm space-y-1">
            {location.notable_features.map((feature: string, index: number) => (
              <li key={index} className="flex items-start">
                <span className="text-dnd-gold mr-2">•</span>
                {feature}
              </li>
            ))}
          </ul>
        </div>
      )}

      {/* Accessibility */}
      {location.accessibility && location.accessibility.length > 0 && (
        <div className="mb-4">
          <h4 className="text-sm font-semibold text-gray-400 mb-2">Accessibility</h4>
          <div className="flex flex-wrap gap-1">
            {location.accessibility.map((access: string, index: number) => (
              <span
                key={index}
                className="px-2 py-1 bg-green-600 bg-opacity-30 text-green-300 text-xs rounded"
              >
                {access}
              </span>
            ))}
          </div>
        </div>
      )}

      {/* PC Significance */}
      {location.pc_significance && (
        <div className="mb-4 p-3 bg-dnd-purple bg-opacity-10 border border-dnd-purple border-opacity-30 rounded">
          <h4 className="text-sm font-semibold text-dnd-purple mb-2">Campaign Significance</h4>
          <p className="text-gray-300 text-sm">{location.pc_significance}</p>
        </div>
      )}

      {/* Coordinates */}
      {location.coordinates && (
        <div className="border-t border-gray-700 pt-3 mt-auto">
          <div className="text-xs text-gray-400">
            <span>Coordinates: </span>
            <span className="font-mono text-gray-300">
              {typeof location.coordinates === 'object' 
                ? JSON.stringify(location.coordinates)
                : location.coordinates
              }
            </span>
          </div>
        </div>
      )}
    </div>
  );
}