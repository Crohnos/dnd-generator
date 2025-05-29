import { Scroll, Star, Users, MapPin } from 'lucide-react';

interface QuestHookCardProps {
  quest: {
    id: number;
    title: string;
    description: string;
    difficulty: 'easy' | 'medium' | 'hard';
    reward?: string;
    status: 'available' | 'active' | 'completed';
    related_npc_ids?: number[];
    related_location_ids?: number[];
  };
}

export function QuestHookCard({ quest }: QuestHookCardProps) {
  const getDifficultyBadge = (difficulty: string) => {
    const styles = {
      easy: 'bg-green-100 text-green-800 border-green-200',
      medium: 'bg-yellow-100 text-yellow-800 border-yellow-200', 
      hard: 'bg-red-100 text-red-800 border-red-200',
    } as const;
    
    return styles[difficulty as keyof typeof styles] || styles.medium;
  };

  const getStatusBadge = (status: string) => {
    const styles = {
      available: 'bg-blue-100 text-blue-800 border-blue-200',
      active: 'bg-orange-100 text-orange-800 border-orange-200',
      completed: 'bg-green-100 text-green-800 border-green-200',
    } as const;
    
    return styles[status as keyof typeof styles] || styles.available;
  };

  return (
    <div className="card">
      {/* Header */}
      <div className="flex items-start space-x-3 mb-4">
        <div className="flex-shrink-0">
          <Scroll className="h-10 w-10 text-dnd-purple bg-dnd-purple bg-opacity-20 rounded-full p-2" />
        </div>
        <div className="flex-1">
          <h3 className="text-lg font-bold text-white mb-2">{quest.title}</h3>
          <div className="flex flex-wrap gap-2">
            <span className={`px-2 py-1 text-xs font-medium rounded-full border ${getDifficultyBadge(quest.difficulty)}`}>
              {quest.difficulty}
            </span>
            <span className={`px-2 py-1 text-xs font-medium rounded-full border ${getStatusBadge(quest.status)}`}>
              {quest.status}
            </span>
          </div>
        </div>
      </div>

      {/* Description */}
      <p className="text-gray-300 mb-4 leading-relaxed">{quest.description}</p>

      {/* Reward */}
      {quest.reward && (
        <div className="mb-4">
          <div className="flex items-center space-x-2 mb-2">
            <Star className="h-4 w-4 text-dnd-gold" />
            <h4 className="text-sm font-semibold text-white">Reward</h4>
          </div>
          <p className="text-gray-300 text-sm bg-gray-800 bg-opacity-50 rounded-lg p-3">
            {quest.reward}
          </p>
        </div>
      )}

      {/* Related Information */}
      <div className="space-y-3">
        {/* Related NPCs */}
        {quest.related_npc_ids && quest.related_npc_ids.length > 0 && (
          <div>
            <div className="flex items-center space-x-2 mb-2">
              <Users className="h-4 w-4 text-gray-400" />
              <span className="text-sm text-gray-400">Related NPCs</span>
            </div>
            <div className="flex flex-wrap gap-2">
              {quest.related_npc_ids.map((npcId) => (
                <span
                  key={npcId}
                  className="px-2 py-1 bg-purple-500 bg-opacity-20 text-purple-300 text-xs rounded-full"
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
              <MapPin className="h-4 w-4 text-gray-400" />
              <span className="text-sm text-gray-400">Related Locations</span>
            </div>
            <div className="flex flex-wrap gap-2">
              {quest.related_location_ids.map((locationId) => (
                <span
                  key={locationId}
                  className="px-2 py-1 bg-blue-500 bg-opacity-20 text-blue-300 text-xs rounded-full"
                >
                  Location #{locationId}
                </span>
              ))}
            </div>
          </div>
        )}
      </div>
    </div>
  );
}