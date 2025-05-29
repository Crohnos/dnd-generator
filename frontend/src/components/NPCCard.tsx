import { Eye, EyeOff, User } from 'lucide-react';
import { useState } from 'react';

interface NPCCardProps {
  npc: {
    id: number;
    name: string;
    role: string;
    description: string;
    personality: {
      traits: string[];
      motivation: string;
      fears?: string[];
      connections?: string[];
    };
    secret_info?: string;
  };
}

export function NPCCard({ npc }: NPCCardProps) {
  const [showSecrets, setShowSecrets] = useState(false);

  return (
    <div className="card">
      {/* Header */}
      <div className="flex items-start space-x-3 mb-4">
        <div className="flex-shrink-0">
          <User className="h-10 w-10 text-dnd-purple bg-dnd-purple bg-opacity-20 rounded-full p-2" />
        </div>
        <div className="flex-1">
          <h3 className="text-lg font-bold text-white">{npc.name}</h3>
          <p className="text-dnd-purple text-sm">{npc.role}</p>
        </div>
      </div>

      {/* Description */}
      <p className="text-gray-300 mb-4 leading-relaxed">{npc.description}</p>

      {/* Personality Traits */}
      {npc.personality.traits.length > 0 && (
        <div className="mb-4">
          <h4 className="text-sm font-semibold text-white mb-2">Personality Traits</h4>
          <div className="flex flex-wrap gap-2">
            {npc.personality.traits.map((trait, index) => (
              <span
                key={index}
                className="px-2 py-1 bg-blue-500 bg-opacity-20 text-blue-300 text-xs rounded-full"
              >
                {trait}
              </span>
            ))}
          </div>
        </div>
      )}

      {/* Motivation */}
      {npc.personality.motivation && (
        <div className="mb-4">
          <h4 className="text-sm font-semibold text-white mb-2">Motivation</h4>
          <p className="text-gray-300 text-sm">{npc.personality.motivation}</p>
        </div>
      )}

      {/* Fears */}
      {npc.personality.fears && npc.personality.fears.length > 0 && (
        <div className="mb-4">
          <h4 className="text-sm font-semibold text-white mb-2">Fears</h4>
          <ul className="text-gray-300 text-sm space-y-1">
            {npc.personality.fears.map((fear, index) => (
              <li key={index} className="flex items-start">
                <span className="text-red-400 mr-2">•</span>
                {fear}
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
            className="flex items-center space-x-2 text-yellow-400 hover:text-yellow-300 text-sm font-medium mb-2"
          >
            {showSecrets ? (
              <EyeOff className="h-4 w-4" />
            ) : (
              <Eye className="h-4 w-4" />
            )}
            <span>DM Secrets</span>
          </button>
          
          {showSecrets && (
            <div className="bg-yellow-500 bg-opacity-10 border border-yellow-500 border-opacity-30 rounded-lg p-3">
              <p className="text-yellow-200 text-sm">{npc.secret_info}</p>
            </div>
          )}
        </div>
      )}
    </div>
  );
}