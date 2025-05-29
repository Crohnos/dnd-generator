import { MapPin, Eye, EyeOff } from 'lucide-react';
import { useState } from 'react';

interface LocationCardProps {
  location: {
    id: number;
    name: string;
    type: string;
    description: string;
    properties?: {
      atmosphere?: string;
      notable_features?: string[];
      secrets?: string[];
      connections?: string[];
    };
  };
}

export function LocationCard({ location }: LocationCardProps) {
  const [showSecrets, setShowSecrets] = useState(false);

  const getLocationIcon = (type: string) => {
    // Color-code by location type
    const colors = {
      'Castle': 'text-purple-400',
      'Town': 'text-blue-400',
      'Forest': 'text-green-400',
      'Dungeon': 'text-red-400',
      'Temple': 'text-yellow-400',
      'Tavern': 'text-orange-400',
    } as const;
    
    return colors[type as keyof typeof colors] || 'text-dnd-gold';
  };

  return (
    <div className="card">
      {/* Header */}
      <div className="flex items-start space-x-3 mb-4">
        <div className="flex-shrink-0">
          <MapPin className={`h-10 w-10 ${getLocationIcon(location.type)} bg-opacity-20 rounded-full p-2`} 
                  style={{ backgroundColor: 'currentColor', opacity: 0.2 }} />
        </div>
        <div className="flex-1">
          <h3 className="text-lg font-bold text-white">{location.name}</h3>
          <p className="text-dnd-gold text-sm">{location.type}</p>
        </div>
      </div>

      {/* Description */}
      <p className="text-gray-300 mb-4 leading-relaxed">{location.description}</p>

      {/* Atmosphere */}
      {location.properties?.atmosphere && (
        <div className="mb-4">
          <h4 className="text-sm font-semibold text-white mb-2">Atmosphere</h4>
          <p className="text-gray-300 text-sm italic">{location.properties.atmosphere}</p>
        </div>
      )}

      {/* Notable Features */}
      {location.properties?.notable_features && location.properties.notable_features.length > 0 && (
        <div className="mb-4">
          <h4 className="text-sm font-semibold text-white mb-2">Notable Features</h4>
          <ul className="text-gray-300 text-sm space-y-1">
            {location.properties.notable_features.map((feature, index) => (
              <li key={index} className="flex items-start">
                <span className="text-dnd-gold mr-2">•</span>
                {feature}
              </li>
            ))}
          </ul>
        </div>
      )}

      {/* Connections */}
      {location.properties?.connections && location.properties.connections.length > 0 && (
        <div className="mb-4">
          <h4 className="text-sm font-semibold text-white mb-2">Connections</h4>
          <div className="flex flex-wrap gap-2">
            {location.properties.connections.map((connection, index) => (
              <span
                key={index}
                className="px-2 py-1 bg-gray-600 bg-opacity-50 text-gray-300 text-xs rounded-full"
              >
                {connection}
              </span>
            ))}
          </div>
        </div>
      )}

      {/* Hidden Secrets */}
      {location.properties?.secrets && location.properties.secrets.length > 0 && (
        <div className="border-t border-gray-700 pt-4">
          <button
            onClick={() => setShowSecrets(!showSecrets)}
            className="flex items-center space-x-2 text-yellow-400 hover:text-yellow-300 text-sm font-medium mb-2"
          >
            {showSecrets ? (
              <EyeOff className="h-4 w-4" />
            ) : (
              <Eye className="h-4 w-4" />
            )}
            <span>Hidden Secrets</span>
          </button>
          
          {showSecrets && (
            <div className="bg-yellow-500 bg-opacity-10 border border-yellow-500 border-opacity-30 rounded-lg p-3">
              <ul className="text-yellow-200 text-sm space-y-2">
                {location.properties.secrets.map((secret, index) => (
                  <li key={index} className="flex items-start">
                    <span className="text-yellow-400 mr-2">•</span>
                    {secret}
                  </li>
                ))}
              </ul>
            </div>
          )}
        </div>
      )}
    </div>
  );
}