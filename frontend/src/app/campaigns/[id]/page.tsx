'use client';

import { useParams } from 'next/navigation';
import Link from 'next/link';
import { ArrowLeft, Users, MapPin, Scroll, BarChart3 } from 'lucide-react';
import { useState } from 'react';

// Mock data - will be replaced with GraphQL query
const mockCampaign = {
  id: 1,
  name: 'The Shattered Crown',
  setting: 'A war-torn kingdom where ancient magic clashes with political intrigue',
  themes: ['political intrigue', 'war', 'mystery'],
  status: 'ready' as const,
  metadata: {
    plot_summary: 'The rightful heir to the throne has been murdered, and dark forces seek to claim power.',
    central_conflict: 'Ancient magical artifacts are being used to manipulate the succession.',
  },
  npcs: [
    {
      id: 1,
      name: 'Lord Aldric Blackwood',
      role: 'Noble Conspirator',
      description: 'A cunning lord who seeks to claim the throne through manipulation and dark magic.',
    },
  ],
  locations: [
    {
      id: 1,
      name: 'Shadowmere Castle',
      type: 'Castle',
      description: 'An imposing fortress shrouded in mystery and ancient power.',
    },
  ],
  quest_hooks: [
    {
      id: 1,
      title: 'The Lost Heir',
      description: 'Investigate rumors of a surviving heir hidden in the countryside.',
      difficulty: 'medium' as const,
      status: 'available' as const,
    },
  ],
};

type TabType = 'overview' | 'npcs' | 'locations' | 'quests';

export default function CampaignDetailPage() {
  const params = useParams();
  const [activeTab, setActiveTab] = useState<TabType>('overview');
  
  // TODO: Replace with actual GraphQL query using params.id
  const campaign = mockCampaign;

  const getDifficultyBadge = (difficulty: string) => {
    switch (difficulty) {
      case 'easy':
        return 'bg-green-100 text-green-800';
      case 'medium':
        return 'bg-yellow-100 text-yellow-800';
      case 'hard':
        return 'bg-red-100 text-red-800';
      default:
        return 'bg-gray-100 text-gray-800';
    }
  };

  const tabs = [
    { id: 'overview' as TabType, label: 'Overview', icon: BarChart3 },
    { id: 'npcs' as TabType, label: 'NPCs', icon: Users },
    { id: 'locations' as TabType, label: 'Locations', icon: MapPin },
    { id: 'quests' as TabType, label: 'Quests', icon: Scroll },
  ];

  return (
    <div className="min-h-screen py-8 px-4 sm:px-6 lg:px-8">
      <div className="max-w-7xl mx-auto">
        {/* Header */}
        <div className="mb-8">
          <Link 
            href="/campaigns" 
            className="inline-flex items-center text-gray-400 hover:text-white mb-4"
          >
            <ArrowLeft className="w-4 h-4 mr-2" />
            Back to Campaigns
          </Link>
          
          <div className="flex flex-col lg:flex-row lg:items-center lg:justify-between">
            <div>
              <h1 className="text-3xl font-bold text-white mb-2">{campaign.name}</h1>
              <p className="text-gray-300 max-w-3xl">{campaign.setting}</p>
            </div>
            
            <div className="mt-4 lg:mt-0">
              <div className="flex flex-wrap gap-2">
                {campaign.themes.map((theme) => (
                  <span
                    key={theme}
                    className="px-3 py-1 bg-dnd-purple bg-opacity-20 text-dnd-purple text-sm rounded-full"
                  >
                    {theme}
                  </span>
                ))}
              </div>
            </div>
          </div>
        </div>

        {/* Tab Navigation */}
        <div className="border-b border-gray-700 mb-8">
          <nav className="-mb-px flex space-x-8">
            {tabs.map((tab) => {
              const Icon = tab.icon;
              const isActive = activeTab === tab.id;
              
              return (
                <button
                  key={tab.id}
                  onClick={() => setActiveTab(tab.id)}
                  className={`flex items-center space-x-2 py-4 px-1 border-b-2 font-medium text-sm transition-colors ${
                    isActive
                      ? 'border-dnd-purple text-dnd-purple'
                      : 'border-transparent text-gray-400 hover:text-gray-300 hover:border-gray-300'
                  }`}
                >
                  <Icon className="w-5 h-5" />
                  <span>{tab.label}</span>
                </button>
              );
            })}
          </nav>
        </div>

        {/* Tab Content */}
        <div className="tab-content">
          {activeTab === 'overview' && (
            <div className="space-y-8">
              {/* Statistics */}
              <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
                <div className="card text-center">
                  <Users className="w-8 h-8 text-dnd-purple mx-auto mb-2" />
                  <div className="text-2xl font-bold text-white">{campaign.npcs.length}</div>
                  <div className="text-gray-400">NPCs</div>
                </div>
                <div className="card text-center">
                  <MapPin className="w-8 h-8 text-dnd-gold mx-auto mb-2" />
                  <div className="text-2xl font-bold text-white">{campaign.locations.length}</div>
                  <div className="text-gray-400">Locations</div>
                </div>
                <div className="card text-center">
                  <Scroll className="w-8 h-8 text-dnd-purple mx-auto mb-2" />
                  <div className="text-2xl font-bold text-white">{campaign.quest_hooks.length}</div>
                  <div className="text-gray-400">Quest Hooks</div>
                </div>
              </div>

              {/* Plot Summary */}
              {campaign.metadata.plot_summary && (
                <div className="card">
                  <h3 className="text-xl font-bold text-white mb-4">Plot Summary</h3>
                  <p className="text-gray-300">{campaign.metadata.plot_summary}</p>
                </div>
              )}

              {/* Central Conflict */}
              {campaign.metadata.central_conflict && (
                <div className="card">
                  <h3 className="text-xl font-bold text-white mb-4">Central Conflict</h3>
                  <p className="text-gray-300">{campaign.metadata.central_conflict}</p>
                </div>
              )}
            </div>
          )}

          {activeTab === 'npcs' && (
            <div className="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
              {campaign.npcs.map((npc) => (
                <div key={npc.id} className="card">
                  <h3 className="text-lg font-bold text-white mb-2">{npc.name}</h3>
                  <p className="text-dnd-purple text-sm mb-3">{npc.role}</p>
                  <p className="text-gray-300">{npc.description}</p>
                </div>
              ))}
            </div>
          )}

          {activeTab === 'locations' && (
            <div className="grid gap-6 md:grid-cols-2">
              {campaign.locations.map((location) => (
                <div key={location.id} className="card">
                  <h3 className="text-lg font-bold text-white mb-2">{location.name}</h3>
                  <p className="text-dnd-gold text-sm mb-3">{location.type}</p>
                  <p className="text-gray-300">{location.description}</p>
                </div>
              ))}
            </div>
          )}

          {activeTab === 'quests' && (
            <div className="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
              {campaign.quest_hooks.map((quest) => (
                <div key={quest.id} className="card">
                  <h3 className="text-lg font-bold text-white mb-2">{quest.title}</h3>
                  <div className="flex items-center space-x-2 mb-3">
                    <span className={`px-2 py-1 text-xs rounded-full ${getDifficultyBadge(quest.difficulty)}`}>
                      {quest.difficulty}
                    </span>
                  </div>
                  <p className="text-gray-300">{quest.description}</p>
                </div>
              ))}
            </div>
          )}
        </div>
      </div>
    </div>
  );
}