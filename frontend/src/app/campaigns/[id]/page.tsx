'use client';

import { useParams } from 'next/navigation';
import Link from 'next/link';
import { ArrowLeft, Users, MapPin, Scroll, BarChart3, AlertCircle, Sparkles } from 'lucide-react';
import { useState } from 'react';
import { NPCCard } from '@/components/NPCCard';
import { LocationCard } from '@/components/LocationCard';
import { QuestHookCard } from '@/components/QuestHookCard';
import { useGetCampaignQuery } from '@/generated/graphql';

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
      description: 'A cunning lord who seeks to claim the throne through manipulation and dark magic. His piercing blue eyes seem to see through every lie, while his silver tongue has earned him many allies—and enemies.',
      personality: {
        traits: ['Ambitious', 'Manipulative', 'Charming', 'Ruthless'],
        motivation: 'To claim the throne and restore what he believes is rightful order to the realm',
        fears: ['Loss of control', 'Being exposed as a fraud'],
        connections: ['Has spies in the royal court', 'Secretly funding rebel groups']
      },
      secret_info: 'Aldric is actually the bastard son of the previous king and believes he has a legitimate claim to the throne. He possesses a cursed amulet that grants him influence over weak-willed individuals.'
    },
    {
      id: 2,
      name: 'Sister Marianne',
      role: 'Temple Priest',
      description: 'A devoted cleric who tends to the spiritual needs of the people while harboring doubts about the divine.',
      personality: {
        traits: ['Compassionate', 'Doubting', 'Protective', 'Wise'],
        motivation: 'To protect the innocent and find true meaning in her faith',
        fears: ['That the gods have abandoned them', 'Failing those who depend on her'],
        connections: ['Leads the underground resistance', 'Has visions of the future']
      },
      secret_info: 'Marianne is losing her divine powers due to her crisis of faith, but she\'s discovered she has latent sorcerous abilities that she\'s afraid to use.'
    },
    {
      id: 3,
      name: 'Captain Marcus Ironhold',
      role: 'Guard Captain',
      description: 'A grizzled veteran who maintains order in the capital while questioning his loyalties.',
      personality: {
        traits: ['Honorable', 'Conflicted', 'Loyal', 'Weary'],
        motivation: 'To serve justice and protect the people, regardless of who sits on the throne',
        fears: ['Civil war destroying everything he\'s sworn to protect'],
        connections: ['Commands the city watch', 'Secret meetings with rebel leaders']
      },
      secret_info: 'Marcus knows the location of the true heir and is secretly protecting them while trying to decide if they\'re worthy of the crown.'
    }
  ],
  locations: [
    {
      id: 1,
      name: 'Shadowmere Castle',
      type: 'Castle',
      description: 'An imposing fortress shrouded in mystery and ancient power. Its black stone walls seem to absorb light, and strange whispers echo through its corridors.',
      properties: {
        atmosphere: 'Dark and foreboding, with an undercurrent of magical energy that makes visitors uneasy',
        notable_features: ['Ancient magical wards', 'Hidden passages', 'Throne room with enchanted crown', 'Underground dungeons'],
        secrets: ['Contains a portal to the Shadowfell in the deepest dungeon', 'The castle itself is sentient and chooses its rulers'],
        connections: ['Connected to the capital by the King\'s Road', 'Has secret tunnels leading to the nearby forest']
      }
    },
    {
      id: 2,
      name: 'The Whispering Woods',
      type: 'Forest',
      description: 'A dense woodland where the trees seem to murmur secrets and the paths shift when no one is looking.',
      properties: {
        atmosphere: 'Mystical and alive, filled with the rustle of leaves that sound almost like voices',
        notable_features: ['Talking trees', 'Pools of starlight', 'Ruins of an ancient druid circle', 'Wildlife that seems unusually intelligent'],
        secrets: ['The true heir is hidden in a cottage deep within', 'Ancient fey magic still lingers here'],
        connections: ['Borders Shadowmere Castle', 'Hidden paths to the capital']
      }
    },
    {
      id: 3,
      name: 'The Broken Crown Tavern',
      type: 'Tavern',
      description: 'A bustling inn where information flows as freely as the ale, and every patron seems to have a secret.',
      properties: {
        atmosphere: 'Warm but tense, filled with hushed conversations and watchful eyes',
        notable_features: ['Secret meeting rooms upstairs', 'A bard who knows every rumor', 'Customers from all walks of life', 'Hidden messages in the menu'],
        secrets: ['Serves as headquarters for the resistance', 'The innkeeper is a retired spy'],
        connections: ['Located in the capital city', 'Has tunnels connecting to the temple district']
      }
    }
  ],
  quest_hooks: [
    {
      id: 1,
      title: 'The Lost Heir',
      description: 'Investigate rumors of a surviving heir hidden in the countryside. Multiple sources claim they\'ve seen someone bearing the royal birthmark.',
      difficulty: 'medium' as const,
      status: 'available' as const,
      reward: '500 gold pieces and a royal pardon for any past crimes',
      related_npc_ids: [3],
      related_location_ids: [2]
    },
    {
      id: 2,
      title: 'The Cursed Amulet',
      description: 'Strange reports of people acting against their will have been linked to a mysterious noble. Discover the source of this magical influence.',
      difficulty: 'hard' as const,
      status: 'available' as const,
      reward: 'Ancient spellbook and the gratitude of the resistance',
      related_npc_ids: [1],
      related_location_ids: [1]
    },
    {
      id: 3,
      title: 'Crisis of Faith',
      description: 'Help a troubled cleric who is losing her divine powers while mysterious magical events occur around her.',
      difficulty: 'easy' as const,
      status: 'active' as const,
      reward: 'Divine blessing and access to temple resources',
      related_npc_ids: [2],
      related_location_ids: [3]
    }
  ],
};

type TabType = 'overview' | 'npcs' | 'locations' | 'quests';

export default function CampaignDetailPage() {
  const params = useParams();
  const [activeTab, setActiveTab] = useState<TabType>('overview');
  
  const campaignId = parseInt(params.id as string);
  const [{ data, fetching, error }] = useGetCampaignQuery({
    variables: { id: campaignId },
    pause: !campaignId || isNaN(campaignId),
  });

  if (error) {
    return (
      <div className="min-h-screen flex items-center justify-center py-8 px-4 sm:px-6 lg:px-8">
        <div className="text-center">
          <AlertCircle className="w-16 h-16 text-red-500 mx-auto mb-4" />
          <h1 className="text-2xl font-bold text-white mb-2">Campaign Not Found</h1>
          <p className="text-gray-400 mb-6">
            The requested campaign could not be loaded.
          </p>
          <Link href="/campaigns" className="btn-primary">
            Back to Campaigns
          </Link>
        </div>
      </div>
    );
  }

  if (fetching) {
    return (
      <div className="min-h-screen flex items-center justify-center py-8 px-4 sm:px-6 lg:px-8">
        <div className="text-center">
          <Sparkles className="w-16 h-16 text-dnd-purple animate-spin mx-auto mb-4" />
          <h1 className="text-2xl font-bold text-white mb-2">Loading Campaign</h1>
          <p className="text-gray-400">
            Fetching campaign details...
          </p>
        </div>
      </div>
    );
  }

  const campaign = data?.campaigns_by_pk;
  
  if (!campaign) {
    return (
      <div className="min-h-screen flex items-center justify-center py-8 px-4 sm:px-6 lg:px-8">
        <div className="text-center">
          <AlertCircle className="w-16 h-16 text-gray-500 mx-auto mb-4" />
          <h1 className="text-2xl font-bold text-white mb-2">Campaign Not Found</h1>
          <p className="text-gray-400 mb-6">
            The campaign with ID {campaignId} does not exist.
          </p>
          <Link href="/campaigns" className="btn-primary">
            Back to Campaigns
          </Link>
        </div>
      </div>
    );
  }

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
            
            {campaign.themes && campaign.themes.length > 0 && (
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
            )}
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
              {campaign.metadata?.plot_summary && (
                <div className="card">
                  <h3 className="text-xl font-bold text-white mb-4">Plot Summary</h3>
                  <p className="text-gray-300">{campaign.metadata.plot_summary}</p>
                </div>
              )}

              {/* Central Conflict */}
              {campaign.metadata?.central_conflict && (
                <div className="card">
                  <h3 className="text-xl font-bold text-white mb-4">Central Conflict</h3>
                  <p className="text-gray-300">{campaign.metadata.central_conflict}</p>
                </div>
              )}
            </div>
          )}

          {activeTab === 'npcs' && (
            <div className="grid gap-6 md:grid-cols-2 xl:grid-cols-3">
              {campaign.npcs && campaign.npcs.length > 0 ? (
                campaign.npcs.map((npc) => (
                  <NPCCard key={npc.id} npc={npc} />
                ))
              ) : (
                <div className="col-span-full text-center py-12">
                  <Users className="w-16 h-16 text-gray-500 mx-auto mb-4" />
                  <h3 className="text-xl font-bold text-white mb-2">No NPCs Yet</h3>
                  <p className="text-gray-400">
                    NPCs will appear here once the campaign generation is complete.
                  </p>
                </div>
              )}
            </div>
          )}

          {activeTab === 'locations' && (
            <div className="grid gap-6 lg:grid-cols-2">
              {campaign.locations && campaign.locations.length > 0 ? (
                campaign.locations.map((location) => (
                  <LocationCard key={location.id} location={location} />
                ))
              ) : (
                <div className="col-span-full text-center py-12">
                  <MapPin className="w-16 h-16 text-gray-500 mx-auto mb-4" />
                  <h3 className="text-xl font-bold text-white mb-2">No Locations Yet</h3>
                  <p className="text-gray-400">
                    Locations will appear here once the campaign generation is complete.
                  </p>
                </div>
              )}
            </div>
          )}

          {activeTab === 'quests' && (
            <div className="grid gap-6 md:grid-cols-2 xl:grid-cols-3">
              {campaign.quest_hooks && campaign.quest_hooks.length > 0 ? (
                campaign.quest_hooks.map((quest) => (
                  <QuestHookCard key={quest.id} quest={quest} />
                ))
              ) : (
                <div className="col-span-full text-center py-12">
                  <Scroll className="w-16 h-16 text-gray-500 mx-auto mb-4" />
                  <h3 className="text-xl font-bold text-white mb-2">No Quest Hooks Yet</h3>
                  <p className="text-gray-400">
                    Quest hooks will appear here once the campaign generation is complete.
                  </p>
                </div>
              )}
            </div>
          )}
        </div>
      </div>
    </div>
  );
}