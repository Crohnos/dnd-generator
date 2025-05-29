import { Scroll, Star, Users, MapPin } from 'lucide-react';

interface QuestHookCardProps {
  quest: {
    id: number;
    title: string;
    description?: string | null;
    difficulty?: string | null;
    reward?: string | null;
    status?: string | null;
    related_npc_ids?: number[] | null;
    related_location_ids?: number[] | null;
  };
}

export function QuestHookCard({ quest }: QuestHookCardProps) {
  const getDifficultyBadge = (difficulty?: string | null) => {
    if (!difficulty) return 'bg-gray-100 text-gray-800 border-gray-200';
    
    const styles = {
      easy: 'bg-green-100 text-green-800 border-green-200',
      medium: 'bg-yellow-100 text-yellow-800 border-yellow-200', 
      hard: 'bg-red-100 text-red-800 border-red-200',
    } as const;
    
    return styles[difficulty as keyof typeof styles] || styles.medium;
  };

  const getStatusBadge = (status?: string | null) => {
    if (!status) return 'bg-gray-100 text-gray-800 border-gray-200';
    
    const styles = {
      available: 'bg-blue-100 text-blue-800 border-blue-200',
      active: 'bg-orange-100 text-orange-800 border-orange-200',
      completed: 'bg-green-100 text-green-800 border-green-200',
    } as const;
    
    return styles[status as keyof typeof styles] || styles.available;
  };

  return (
    <div className="card hover:border-dnd-purple transition-colors">
      {/* Header */}
      <div className="flex items-start space-x-4 mb-4">
        <div className="flex-shrink-0">
          <div className="w-12 h-12 bg-dnd-purple bg-opacity-20 rounded-full flex items-center justify-center">
            <Scroll className="w-6 h-6 text-dnd-purple" />
          </div>
        </div>
        <div className="flex-grow">
          <h3 className="text-lg font-bold text-white mb-2">{quest.title}</h3>
          <div className="flex flex-wrap gap-2">
            {quest.difficulty && (
              <span className={`px-2 py-1 text-xs font-medium rounded-full border ${getDifficultyBadge(quest.difficulty)}`}>
                {quest.difficulty}
              </span>
            )}
            {quest.status && (
              <span className={`px-2 py-1 text-xs font-medium rounded-full border ${getStatusBadge(quest.status)}`}>
                {quest.status}
              </span>
            )}
          </div>
        </div>
      </div>

      {/* Description */}
      {quest.description && (
        <p className="text-gray-300 mb-4 leading-relaxed">{quest.description}</p>
      )}

      {/* Reward */}
      {quest.reward && (
        <div className="mb-4">
          <div className="flex items-center space-x-2 mb-2">
            <Star className="w-4 h-4 text-dnd-gold" />
            <h4 className="text-sm font-semibold text-gray-400">Reward</h4>
          </div>
          <p className="text-gray-300 text-sm bg-gray-800 bg-opacity-50 rounded p-3 leading-relaxed">
            {quest.reward}
          </p>
        </div>
      )}

      {/* Related Information */}
      {((quest.related_npc_ids && quest.related_npc_ids.length > 0) || 
        (quest.related_location_ids && quest.related_location_ids.length > 0)) && (
        <div className="space-y-3">
          {/* Related NPCs */}
          {quest.related_npc_ids && quest.related_npc_ids.length > 0 && (
            <div>
              <div className="flex items-center space-x-2 mb-2">
                <Users className="w-4 h-4 text-gray-400" />
                <span className="text-sm font-semibold text-gray-400">Related NPCs</span>
              </div>
              <div className="flex flex-wrap gap-1">
                {quest.related_npc_ids.map((npcId) => (
                  <span
                    key={npcId}
                    className="px-2 py-1 bg-dnd-purple bg-opacity-10 text-dnd-purple text-xs rounded"
                  >
                    NPC #{npcId}
                  </span>
                ))}
              </div>
            </div>
          )}

          {/* Related Locations */}
          {quest.related_location_ids && quest.related_location_ids.length > 0 && (
            <div>
              <div className="flex items-center space-x-2 mb-2">
                <MapPin className="w-4 h-4 text-gray-400" />
                <span className="text-sm font-semibold text-gray-400">Related Locations</span>
              </div>
              <div className="flex flex-wrap gap-1">
                {quest.related_location_ids.map((locationId) => (
                  <span
                    key={locationId}
                    className="px-2 py-1 bg-dnd-gold bg-opacity-10 text-dnd-gold text-xs rounded"
                  >
                    Location #{locationId}
                  </span>
                ))}
              </div>
            </div>
          )}
        </div>
      )}
    </div>
  );
}