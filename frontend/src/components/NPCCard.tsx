import { Eye, EyeOff, User } from 'lucide-react';
import { useState } from 'react';

interface NPCCardProps {
  npc: {
    id: number;
    name: string;
    role?: string | null;
    description?: string | null;
    personality?: any;
    secret_info?: string | null;
  };
}

export function NPCCard({ npc }: NPCCardProps) {
  const [showSecrets, setShowSecrets] = useState(false);

  const getRoleColor = (role?: string | null) => {
    if (!role) return 'bg-gray-100 text-gray-800';
    
    const roleColors: Record<string, string> = {
      'Noble': 'bg-purple-100 text-purple-800',
      'Merchant': 'bg-yellow-100 text-yellow-800',
      'Guard': 'bg-blue-100 text-blue-800',
      'Priest': 'bg-indigo-100 text-indigo-800',
      'Villain': 'bg-red-100 text-red-800',
      'Scholar': 'bg-green-100 text-green-800',
      'Commoner': 'bg-gray-100 text-gray-800',
    };

    for (const [key, color] of Object.entries(roleColors)) {
      if (role.toLowerCase().includes(key.toLowerCase())) {
        return color;
      }
    }
    
    return 'bg-gray-100 text-gray-800';
  };

  return (
    <div className="card hover:border-dnd-purple transition-colors">
      {/* Header */}
      <div className="flex items-start space-x-4 mb-4">
        <div className="flex-shrink-0">
          <div className="w-12 h-12 bg-dnd-purple bg-opacity-20 rounded-full flex items-center justify-center">
            <User className="w-6 h-6 text-dnd-purple" />
          </div>
        </div>
        <div className="flex-grow">
          <h3 className="text-lg font-bold text-white mb-1">{npc.name}</h3>
          {npc.role && (
            <span className={`inline-block px-2 py-1 text-xs rounded-full ${getRoleColor(npc.role)}`}>
              {npc.role}
            </span>
          )}
        </div>
      </div>

      {/* Description */}
      {npc.description && (
        <p className="text-gray-300 mb-4 leading-relaxed">{npc.description}</p>
      )}

      {/* Personality Traits */}
      {npc.personality?.traits && npc.personality.traits.length > 0 && (
        <div className="mb-4">
          <h4 className="text-sm font-semibold text-gray-400 mb-2">Personality Traits</h4>
          <div className="flex flex-wrap gap-1">
            {npc.personality.traits.map((trait: string, index: number) => (
              <span
                key={index}
                className="px-2 py-1 bg-dnd-purple bg-opacity-10 text-dnd-purple text-xs rounded"
              >
                {trait}
              </span>
            ))}
          </div>
        </div>
      )}

      {/* Motivation */}
      {npc.personality?.motivation && (
        <div className="mb-4">
          <h4 className="text-sm font-semibold text-gray-400 mb-2">Motivation</h4>
          <p className="text-gray-300 text-sm">{npc.personality.motivation}</p>
        </div>
      )}

      {/* Fears */}
      {npc.personality?.fears && npc.personality.fears.length > 0 && (
        <div className="mb-4">
          <h4 className="text-sm font-semibold text-gray-400 mb-2">Fears</h4>
          <ul className="text-gray-300 text-sm space-y-1">
            {npc.personality.fears.map((fear: string, index: number) => (
              <li key={index} className="flex items-start">
                <span className="text-red-400 mr-2">•</span>
                {fear}
              </li>
            ))}
          </ul>
        </div>
      )}

      {/* Connections */}
      {npc.personality?.connections && npc.personality.connections.length > 0 && (
        <div className="mb-4">
          <h4 className="text-sm font-semibold text-gray-400 mb-2">Connections</h4>
          <ul className="text-gray-300 text-sm space-y-1">
            {npc.personality.connections.map((connection: string, index: number) => (
              <li key={index} className="flex items-start">
                <span className="text-dnd-purple mr-2">•</span>
                {connection}
              </li>
            ))}
          </ul>
        </div>
      )}

      {/* DM Secrets */}
      {npc.secret_info && (
        <div className="border-t border-gray-700 pt-4">
          <button
            onClick={() => setShowSecrets(!showSecrets)}
            className="flex items-center justify-between w-full text-left text-sm font-semibold text-yellow-400 hover:text-yellow-300 transition-colors"
          >
            <span>DM Secrets</span>
            {showSecrets ? (
              <EyeOff className="w-4 h-4" />
            ) : (
              <Eye className="w-4 h-4" />
            )}
          </button>
          
          {showSecrets && (
            <div className="mt-3 p-3 bg-yellow-900 bg-opacity-20 border border-yellow-600 border-opacity-30 rounded">
              <p className="text-yellow-100 text-sm leading-relaxed">
                {npc.secret_info}
              </p>
            </div>
          )}
        </div>
      )}
    </div>
  );
}