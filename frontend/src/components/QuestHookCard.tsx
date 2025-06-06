import { Scroll, Star, Users, MapPin } from 'lucide-react';

interface QuestHookCardProps {
  quest: {
    id: number;
    title: string;
    description?: string | null;
    quest_type?: string | null;
    difficulty?: string | null;
    estimated_sessions?: number | null;
    reward?: string | null;
    related_entity_ids?: number[] | null;
    related_location_ids?: number[] | null;
    prerequisites?: string[] | null;
    consequences?: string[] | null;
    status?: string | null;
    pc_hook_type?: string | null;
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
            {quest.quest_type && (
              <span className="px-2 py-1 bg-dnd-purple bg-opacity-20 text-dnd-purple text-xs rounded-full">
                {quest.quest_type}
              </span>
            )}
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
            {quest.pc_hook_type && (
              <span className="px-2 py-1 bg-yellow-600 bg-opacity-20 text-yellow-300 text-xs rounded-full">
                {quest.pc_hook_type}
              </span>
            )}
          </div>
        </div>
      </div>

      {/* Description */}
      {quest.description && (
        <p className="text-gray-300 mb-4 leading-relaxed">{quest.description}</p>
      )}

      {/* Quest Details */}
      <div className="grid grid-cols-2 gap-4 mb-4 text-sm">
        {quest.estimated_sessions && (
          <div>
            <span className="text-gray-400">Sessions:</span>
            <span className="text-white ml-2">{quest.estimated_sessions}</span>
          </div>
        )}
      </div>

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

      {/* Prerequisites */}
      {quest.prerequisites && quest.prerequisites.length > 0 && (
        <div className="mb-4">
          <h4 className="text-sm font-semibold text-gray-400 mb-2">Prerequisites</h4>
          <ul className="text-gray-300 text-sm space-y-1">
            {quest.prerequisites.map((prereq: string, index: number) => (
              <li key={index} className="flex items-start">
                <span className="text-yellow-400 mr-2">⚠</span>
                {prereq}
              </li>
            ))}
          </ul>
        </div>
      )}

      {/* Consequences */}
      {quest.consequences && quest.consequences.length > 0 && (
        <div className="mb-4">
          <h4 className="text-sm font-semibold text-gray-400 mb-2">Potential Consequences</h4>
          <ul className="text-gray-300 text-sm space-y-1">
            {quest.consequences.map((consequence: string, index: number) => (
              <li key={index} className="flex items-start">
                <span className="text-red-400 mr-2">⚡</span>
                {consequence}
              </li>
            ))}
          </ul>
        </div>
      )}

      {/* Related Information */}
      {((quest.related_entity_ids && quest.related_entity_ids.length > 0) || 
        (quest.related_location_ids && quest.related_location_ids.length > 0)) && (
        <div className="space-y-3">
          {/* Related Entities */}
          {quest.related_entity_ids && quest.related_entity_ids.length > 0 && (
            <div>
              <div className="flex items-center space-x-2 mb-2">
                <Users className="w-4 h-4 text-gray-400" />
                <span className="text-sm font-semibold text-gray-400">Related Entities</span>
              </div>
              <div className="flex flex-wrap gap-1">
                {quest.related_entity_ids.map((entityId) => (
                  <span
                    key={entityId}
                    className="px-2 py-1 bg-dnd-purple bg-opacity-10 text-dnd-purple text-xs rounded"
                  >
                    Entity #{entityId}
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