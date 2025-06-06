import { Package, Star, Shield, Sword, Eye, EyeOff } from 'lucide-react';
import { useState } from 'react';

interface ItemCardProps {
  item: {
    id: number;
    name: string;
    item_type: string;
    item_subtype?: string | null;
    rarity?: string | null;
    is_magical?: boolean | null;
    is_sentient?: boolean | null;
    requires_attunement?: boolean | null;
    description?: string | null;
    weight_pounds?: number | null;
    value_gp?: number | null;
    damage_dice?: string | null;
    armor_class?: number | null;
    properties?: string[] | null;
    pc_significance?: string | null;
    history?: string | null;
    creator?: string | null;
  };
}

export function ItemCard({ item }: ItemCardProps) {
  const [showDetails, setShowDetails] = useState(false);

  const getItemIcon = (type: string) => {
    switch (type.toLowerCase()) {
      case 'weapon': return Sword;
      case 'armor': return Shield;
      case 'treasure': return Star;
      default: return Package;
    }
  };

  const getRarityColor = (rarity?: string | null) => {
    const colors = {
      'common': 'text-gray-400',
      'uncommon': 'text-green-400',
      'rare': 'text-blue-400',
      'very rare': 'text-purple-400',
      'legendary': 'text-orange-400',
      'artifact': 'text-red-400',
    } as const;
    
    return colors[rarity?.toLowerCase() as keyof typeof colors] || 'text-gray-400';
  };

  const Icon = getItemIcon(item.item_type);

  return (
    <div className="card hover:border-dnd-gold transition-colors">
      {/* Header */}
      <div className="flex items-start space-x-4 mb-4">
        <div className="flex-shrink-0">
          <div className={`w-12 h-12 ${getRarityColor(item.rarity)} bg-opacity-20 rounded-full flex items-center justify-center`}
               style={{ backgroundColor: 'currentColor', opacity: 0.1 }}>
            <Icon className={`w-6 h-6 ${getRarityColor(item.rarity)}`} />
          </div>
        </div>
        <div className="flex-grow">
          <h3 className="text-lg font-bold text-white mb-1">{item.name}</h3>
          <div className="flex items-center space-x-2">
            <span className="inline-block px-2 py-1 bg-gray-700 text-gray-300 text-xs rounded-full">
              {item.item_type}
            </span>
            {item.item_subtype && (
              <span className="inline-block px-2 py-1 bg-gray-600 text-gray-300 text-xs rounded-full">
                {item.item_subtype}
              </span>
            )}
            {item.rarity && (
              <span className={`inline-block px-2 py-1 ${getRarityColor(item.rarity)} bg-opacity-20 text-xs rounded-full`}
                    style={{ backgroundColor: 'currentColor', opacity: 0.2 }}>
                {item.rarity}
              </span>
            )}
          </div>
        </div>
      </div>

      {/* Magical Properties */}
      {(item.is_magical || item.is_sentient || item.requires_attunement) && (
        <div className="mb-4 flex flex-wrap gap-2">
          {item.is_magical && (
            <span className="px-2 py-1 bg-purple-600 bg-opacity-30 text-purple-300 text-xs rounded">
              Magical
            </span>
          )}
          {item.is_sentient && (
            <span className="px-2 py-1 bg-yellow-600 bg-opacity-30 text-yellow-300 text-xs rounded">
              Sentient
            </span>
          )}
          {item.requires_attunement && (
            <span className="px-2 py-1 bg-red-600 bg-opacity-30 text-red-300 text-xs rounded">
              Requires Attunement
            </span>
          )}
        </div>
      )}

      {/* Description */}
      {item.description && (
        <p className="text-gray-300 mb-4 leading-relaxed">{item.description}</p>
      )}

      {/* Item Stats */}
      <div className="grid grid-cols-2 gap-4 mb-4 text-sm">
        {item.value_gp && (
          <div>
            <span className="text-gray-400">Value:</span>
            <span className="text-white ml-2">{item.value_gp} gp</span>
          </div>
        )}
        {item.weight_pounds && (
          <div>
            <span className="text-gray-400">Weight:</span>
            <span className="text-white ml-2">{item.weight_pounds} lbs</span>
          </div>
        )}
        {item.damage_dice && (
          <div>
            <span className="text-gray-400">Damage:</span>
            <span className="text-white ml-2">{item.damage_dice}</span>
          </div>
        )}
        {item.armor_class && (
          <div>
            <span className="text-gray-400">AC:</span>
            <span className="text-white ml-2">{item.armor_class}</span>
          </div>
        )}
      </div>

      {/* Properties */}
      {item.properties && item.properties.length > 0 && (
        <div className="mb-4">
          <h4 className="text-sm font-semibold text-gray-400 mb-2">Properties</h4>
          <div className="flex flex-wrap gap-1">
            {item.properties.map((property: string, index: number) => (
              <span
                key={index}
                className="px-2 py-1 bg-blue-600 bg-opacity-30 text-blue-300 text-xs rounded"
              >
                {property}
              </span>
            ))}
          </div>
        </div>
      )}

      {/* PC Significance */}
      {item.pc_significance && (
        <div className="mb-4 p-3 bg-dnd-purple bg-opacity-10 border border-dnd-purple border-opacity-30 rounded">
          <h4 className="text-sm font-semibold text-dnd-purple mb-2">Campaign Significance</h4>
          <p className="text-gray-300 text-sm">{item.pc_significance}</p>
        </div>
      )}

      {/* Additional Details */}
      {(item.history || item.creator) && (
        <div className="border-t border-gray-700 pt-3">
          <button
            onClick={() => setShowDetails(!showDetails)}
            className="flex items-center justify-between w-full text-left text-sm font-semibold text-gray-400 hover:text-white transition-colors"
          >
            <span>Item Details</span>
            {showDetails ? (
              <EyeOff className="w-4 h-4" />
            ) : (
              <Eye className="w-4 h-4" />
            )}
          </button>
          
          {showDetails && (
            <div className="mt-3 space-y-3 text-sm">
              {item.creator && (
                <div>
                  <span className="text-gray-400">Creator:</span>
                  <span className="text-white ml-2">{item.creator}</span>
                </div>
              )}
              {item.history && (
                <div>
                  <h5 className="text-gray-400 mb-1">History:</h5>
                  <p className="text-gray-300 leading-relaxed">{item.history}</p>
                </div>
              )}
            </div>
          )}
        </div>
      )}
    </div>
  );
}