import { Users, Shield, Heart, Sword, Brain, User } from 'lucide-react';

interface EntityCardProps {
  entity: {
    id: number;
    name: string;
    entity_type: string;
    race_id?: number | null;
    class_id?: number | null;
    background_id?: number | null;
    level_or_cr?: string | null;
    ability_scores?: any;
    hit_points?: number | null;
    armor_class?: number | null;
    speed?: number | null;
    skills?: any;
    saving_throws?: any;
    damage_resistances?: string[] | null;
    damage_immunities?: string[] | null;
    condition_immunities?: string[] | null;
    senses?: any;
    languages?: string[] | null;
    special_abilities?: any[] | null;
    spells_known?: any[] | null;
    personality_traits?: string[] | null;
    ideals?: string[] | null;
    bonds?: string[] | null;
    flaws?: string[] | null;
    appearance?: string | null;
    backstory?: string | null;
    motivations?: string[] | null;
    secrets?: string[] | null;
    notes?: string | null;
    pc_connection_type?: string | null;
    pc_connection_description?: string | null;
  };
}

export function EntityCard({ entity }: EntityCardProps) {
  const getEntityTypeIcon = (type: string) => {
    switch (type) {
      case 'npc': return User;
      case 'creature': return Users;
      case 'monster': return Sword;
      default: return User;
    }
  };

  const getEntityTypeColor = (type: string) => {
    switch (type) {
      case 'npc': return 'text-blue-400';
      case 'creature': return 'text-green-400';
      case 'monster': return 'text-red-400';
      default: return 'text-gray-400';
    }
  };

  const Icon = getEntityTypeIcon(entity.entity_type);

  return (
    <div className="card group hover:border-gray-600 transition-all duration-200">
      <div className="flex items-start justify-between mb-3">
        <div className="flex items-center space-x-3">
          <div className={`p-2 rounded-lg bg-gray-800 ${getEntityTypeColor(entity.entity_type)}`}>
            <Icon className="w-5 h-5" />
          </div>
          <div>
            <h3 className="font-bold text-white group-hover:text-dnd-purple transition-colors">
              {entity.name}
            </h3>
            <p className="text-gray-400 text-sm capitalize">
              {entity.entity_type}
              {entity.level_or_cr && ` (${entity.level_or_cr})`}
            </p>
          </div>
        </div>
        
        {/* Stats */}
        <div className="flex space-x-3 text-xs">
          {entity.hit_points && (
            <div className="flex items-center space-x-1 text-red-400">
              <Heart className="w-3 h-3" />
              <span>{entity.hit_points}</span>
            </div>
          )}
          {entity.armor_class && (
            <div className="flex items-center space-x-1 text-blue-400">
              <Shield className="w-3 h-3" />
              <span>{entity.armor_class}</span>
            </div>
          )}
          {entity.speed && (
            <div className="flex items-center space-x-1 text-green-400">
              <span className="text-xs">🏃</span>
              <span>{entity.speed}ft</span>
            </div>
          )}
        </div>
      </div>

      {/* PC Connection */}
      {entity.pc_connection_type && (
        <div className="mb-3 p-3 bg-dnd-purple bg-opacity-10 border border-dnd-purple border-opacity-30 rounded-lg">
          <div className="flex items-center space-x-2 mb-1">
            <Heart className="w-4 h-4 text-dnd-purple" />
            <span className="text-dnd-purple font-medium text-sm">
              PC Connection: {entity.pc_connection_type}
            </span>
          </div>
          {entity.pc_connection_description && (
            <p className="text-gray-300 text-sm">
              {entity.pc_connection_description}
            </p>
          )}
        </div>
      )}

      {/* Appearance */}
      {entity.appearance && (
        <div className="mb-3">
          <p className="text-gray-300 text-sm">
            {entity.appearance}
          </p>
        </div>
      )}

      {/* Backstory */}
      {entity.backstory && (
        <div className="mb-3">
          <h4 className="text-white font-medium text-sm mb-1">Backstory</h4>
          <p className="text-gray-300 text-sm">
            {entity.backstory.length > 150 
              ? `${entity.backstory.substring(0, 150)}...` 
              : entity.backstory
            }
          </p>
        </div>
      )}

      {/* Personality Traits */}
      {entity.personality_traits && entity.personality_traits.length > 0 && (
        <div className="mb-3">
          <h4 className="text-white font-medium text-sm mb-2">Personality</h4>
          <div className="flex flex-wrap gap-1">
            {entity.personality_traits.slice(0, 4).map((trait, index) => (
              <span
                key={index}
                className="px-2 py-1 bg-gray-700 text-gray-300 text-xs rounded-full"
              >
                {trait}
              </span>
            ))}
            {entity.personality_traits.length > 4 && (
              <span className="px-2 py-1 bg-gray-700 text-gray-400 text-xs rounded-full">
                +{entity.personality_traits.length - 4} more
              </span>
            )}
          </div>
        </div>
      )}

      {/* Motivations */}
      {entity.motivations && entity.motivations.length > 0 && (
        <div className="mb-3">
          <h4 className="text-white font-medium text-sm mb-1">Motivations</h4>
          <ul className="text-gray-300 text-sm space-y-1">
            {entity.motivations.slice(0, 2).map((motivation, index) => (
              <li key={index} className="flex items-start space-x-2">
                <span className="text-dnd-gold mt-1">•</span>
                <span>{motivation}</span>
              </li>
            ))}
          </ul>
        </div>
      )}

      {/* Languages */}
      {entity.languages && entity.languages.length > 0 && (
        <div className="mb-3">
          <h4 className="text-white font-medium text-sm mb-1">Languages</h4>
          <div className="flex flex-wrap gap-1">
            {entity.languages.map((language, index) => (
              <span
                key={index}
                className="px-2 py-1 bg-blue-600 bg-opacity-30 text-blue-300 text-xs rounded"
              >
                {language}
              </span>
            ))}
          </div>
        </div>
      )}

      {/* Special Abilities */}
      {entity.special_abilities && entity.special_abilities.length > 0 && (
        <div className="mb-3">
          <h4 className="text-white font-medium text-sm mb-1">Special Abilities</h4>
          <div className="text-gray-300 text-sm">
            {entity.special_abilities.length} abilities available
          </div>
        </div>
      )}

      {/* Resistances/Immunities */}
      {(entity.damage_resistances?.length || entity.damage_immunities?.length || entity.condition_immunities?.length) && (
        <div className="mb-3">
          <h4 className="text-white font-medium text-sm mb-1">Defenses</h4>
          <div className="space-y-1 text-xs">
            {entity.damage_resistances && entity.damage_resistances.length > 0 && (
              <div>
                <span className="text-gray-400">Resistant: </span>
                <span className="text-orange-300">{entity.damage_resistances.join(', ')}</span>
              </div>
            )}
            {entity.damage_immunities && entity.damage_immunities.length > 0 && (
              <div>
                <span className="text-gray-400">Immune: </span>
                <span className="text-green-300">{entity.damage_immunities.join(', ')}</span>
              </div>
            )}
            {entity.condition_immunities && entity.condition_immunities.length > 0 && (
              <div>
                <span className="text-gray-400">Condition Immune: </span>
                <span className="text-blue-300">{entity.condition_immunities.join(', ')}</span>
              </div>
            )}
          </div>
        </div>
      )}

      {/* Quick Stats */}
      <div className="pt-3 border-t border-gray-700 mt-auto">
        <div className="grid grid-cols-2 gap-3 text-xs">
          {entity.ideals && entity.ideals.length > 0 && (
            <div>
              <span className="text-gray-400">Ideals:</span>
              <span className="text-white ml-1">{entity.ideals.length}</span>
            </div>
          )}
          {entity.bonds && entity.bonds.length > 0 && (
            <div>
              <span className="text-gray-400">Bonds:</span>
              <span className="text-white ml-1">{entity.bonds.length}</span>
            </div>
          )}
          {entity.flaws && entity.flaws.length > 0 && (
            <div>
              <span className="text-gray-400">Flaws:</span>
              <span className="text-white ml-1">{entity.flaws.length}</span>
            </div>
          )}
          {entity.secrets && entity.secrets.length > 0 && (
            <div>
              <span className="text-gray-400">Secrets:</span>
              <span className="text-white ml-1">{entity.secrets.length}</span>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}