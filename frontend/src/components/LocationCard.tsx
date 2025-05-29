import { MapPin, Eye, EyeOff } from 'lucide-react';
import { useState } from 'react';

interface LocationCardProps {
  location: {
    id: number;
    name: string;
    type?: string | null;
    description?: string | null;
    properties?: any;
  };
}

export function LocationCard({ location }: LocationCardProps) {
  const [showSecrets, setShowSecrets] = useState(false);

  const getLocationIcon = (type?: string | null) => {
    if (!type) return 'text-dnd-gold';
    
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
    <div className="card hover:border-dnd-gold transition-colors">
      {/* Header */}
      <div className="flex items-start space-x-4 mb-4">
        <div className="flex-shrink-0">
          <div className={`w-12 h-12 ${getLocationIcon(location.type)} bg-opacity-20 rounded-full flex items-center justify-center`}
               style={{ backgroundColor: 'currentColor', opacity: 0.1 }}>
            <MapPin className={`w-6 h-6 ${getLocationIcon(location.type)}`} />
          </div>
        </div>
        <div className="flex-grow">
          <h3 className="text-lg font-bold text-white mb-1">{location.name}</h3>
          {location.type && (
            <span className="inline-block px-2 py-1 bg-dnd-gold bg-opacity-10 text-dnd-gold text-xs rounded-full">
              {location.type}
            </span>
          )}
        </div>
      </div>

      {/* Description */}
      {location.description && (
        <p className="text-gray-300 mb-4 leading-relaxed">{location.description}</p>
      )}

      {/* Atmosphere */}
      {location.properties?.atmosphere && (
        <div className="mb-4">
          <h4 className="text-sm font-semibold text-gray-400 mb-2">Atmosphere</h4>
          <p className="text-gray-300 text-sm italic leading-relaxed">{location.properties.atmosphere}</p>
        </div>
      )}

      {/* Notable Features */}
      {location.properties?.notable_features && location.properties.notable_features.length > 0 && (
        <div className="mb-4">
          <h4 className="text-sm font-semibold text-gray-400 mb-2">Notable Features</h4>
          <ul className="text-gray-300 text-sm space-y-1">
            {location.properties.notable_features.map((feature: string, index: number) => (
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
          <h4 className="text-sm font-semibold text-gray-400 mb-2">Connections</h4>
          <div className="flex flex-wrap gap-1">
            {location.properties.connections.map((connection: string, index: number) => (
              <span
                key={index}
                className="px-2 py-1 bg-gray-600 bg-opacity-30 text-gray-300 text-xs rounded"
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
            className="flex items-center justify-between w-full text-left text-sm font-semibold text-yellow-400 hover:text-yellow-300 transition-colors"
          >
            <span>Hidden Secrets</span>
            {showSecrets ? (
              <EyeOff className="w-4 h-4" />
            ) : (
              <Eye className="w-4 h-4" />
            )}
          </button>
          
          {showSecrets && (
            <div className="mt-3 p-3 bg-yellow-900 bg-opacity-20 border border-yellow-600 border-opacity-30 rounded">
              <ul className="text-yellow-100 text-sm space-y-2">
                {location.properties.secrets.map((secret: string, index: number) => (
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