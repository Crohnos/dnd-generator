'use client';

import { useParams } from 'next/navigation';
import Link from 'next/link';
import { 
  ArrowLeft, Users, MapPin, Scroll, BarChart3, AlertCircle, Sparkles, 
  Globe, Crown, Building2, Sword, BookOpen, Languages, Coins
} from 'lucide-react';
import { useState } from 'react';
import { 
  useGetCampaignQuery, 
  useGetNpCsQuery,
  useGetLocationsQuery,
  useGetQuestHooksQuery,
  useGetEncountersQuery,
  useGetWorldBuildingDataQuery,
  useGetCharacterBuildingDataQuery,
  useGetEntitiesAndRelationshipsQuery
} from '@/generated/graphql';

// Import components (will create these)
import { EntityCard } from '@/components/EntityCard';
import { LocationCard } from '@/components/LocationCard';
import { QuestHookCard } from '@/components/QuestHookCard';
import { WorldBuildingCard } from '@/components/WorldBuildingCard';
import { CharacterBuildingCard } from '@/components/CharacterBuildingCard';

type TabType = 'overview' | 'entities' | 'locations' | 'quests' | 'world' | 'characters' | 'relationships';

export default function CampaignDetailPage() {
  const params = useParams();
  const [activeTab, setActiveTab] = useState<TabType>('overview');
  
  const campaignId = parseInt(params.id as string);
  
  // Main campaign query
  const [{ data: campaignData, fetching, error }] = useGetCampaignQuery({
    variables: { id: campaignId },
    pause: !campaignId || isNaN(campaignId),
  });

  // Entity queries - only fetch when tabs are active
  const [{ data: entitiesData }] = useGetNpCsQuery({
    variables: { campaignId },
    pause: !campaignId || isNaN(campaignId),
  });

  const [{ data: locationsData }] = useGetLocationsQuery({
    variables: { campaignId },
    pause: !campaignId || isNaN(campaignId),
  });

  const [{ data: questsData }] = useGetQuestHooksQuery({
    variables: { campaignId },
    pause: !campaignId || isNaN(campaignId),
  });

  const [{ data: encountersData }] = useGetEncountersQuery({
    variables: { campaignId },
    pause: !campaignId || isNaN(campaignId),
  });

  const [{ data: worldData }] = useGetWorldBuildingDataQuery({
    variables: { campaignId },
    pause: !campaignId || isNaN(campaignId),
  });

  const [{ data: characterData }] = useGetCharacterBuildingDataQuery({
    variables: { campaignId },
    pause: !campaignId || isNaN(campaignId),
  });

  const [{ data: relationshipsData }] = useGetEntitiesAndRelationshipsQuery({
    variables: { campaignId },
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

  const campaign = campaignData?.campaigns_by_pk;
  
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

  // Calculate statistics
  const entities = entitiesData?.entities || [];
  const locations = locationsData?.locations || [];
  const quests = questsData?.quest_hooks || [];
  const encounters = encountersData?.encounters || [];
  
  const races = characterData?.races || [];
  const classes = characterData?.character_classes || [];
  const backgrounds = characterData?.backgrounds || [];
  const languages = characterData?.languages || [];
  
  const pantheons = worldData?.pantheons || [];
  const deities = worldData?.deities || [];
  const planes = worldData?.planes || [];
  const geographyRegions = worldData?.geography_regions || [];
  const economicSystems = worldData?.economic_systems || [];
  const legalSystems = worldData?.legal_systems || [];
  const historicalPeriods = worldData?.historical_periods || [];
  const calendarSystems = worldData?.calendar_systems || [];

  const tabs = [
    { id: 'overview' as TabType, label: 'Overview', icon: BarChart3, count: null },
    { id: 'entities' as TabType, label: 'NPCs & Entities', icon: Users, count: entities.length },
    { id: 'locations' as TabType, label: 'Locations', icon: MapPin, count: locations.length },
    { id: 'quests' as TabType, label: 'Adventures', icon: Scroll, count: quests.length },
    { id: 'world' as TabType, label: 'World Building', icon: Globe, count: pantheons.length + deities.length + planes.length },
    { id: 'characters' as TabType, label: 'Character Options', icon: BookOpen, count: races.length + classes.length },
    { id: 'relationships' as TabType, label: 'Relationships', icon: Crown, count: relationshipsData?.entity_relationships?.length || 0 },
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
              
              {/* Generation Status */}
              <div className="mt-4 flex items-center space-x-4">
                <span className={`px-3 py-1 rounded-full text-sm font-medium ${
                  campaign.status === 'completed' 
                    ? 'bg-green-500 bg-opacity-20 text-green-400' 
                    : campaign.status === 'generating'
                    ? 'bg-yellow-500 bg-opacity-20 text-yellow-400'
                    : campaign.status === 'error'
                    ? 'bg-red-500 bg-opacity-20 text-red-400'
                    : 'bg-gray-500 bg-opacity-20 text-gray-400'
                }`}>
                  {campaign.status === 'completed' ? 'Ready to Play' : 
                   campaign.status === 'generating' ? 'Generating...' :
                   campaign.status === 'error' ? 'Generation Failed' : 'Created'}
                </span>
                
                {campaign.generation_phase && (
                  <span className="text-gray-400 text-sm">
                    Phase {campaign.generation_phase} of {campaign.total_phases || 9}
                  </span>
                )}
              </div>
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
          <nav className="-mb-px flex flex-wrap gap-2 md:gap-0">
            {tabs.map((tab) => {
              const Icon = tab.icon;
              const isActive = activeTab === tab.id;
              
              return (
                <button
                  key={tab.id}
                  onClick={() => setActiveTab(tab.id)}
                  className={`flex items-center space-x-2 py-4 px-3 md:px-1 border-b-2 font-medium text-sm transition-colors whitespace-nowrap ${
                    isActive
                      ? 'border-dnd-purple text-dnd-purple'
                      : 'border-transparent text-gray-400 hover:text-gray-300 hover:border-gray-300'
                  }`}
                >
                  <Icon className="w-5 h-5" />
                  <span>{tab.label}</span>
                  {tab.count !== null && tab.count > 0 && (
                    <span className="bg-gray-700 text-gray-300 text-xs px-2 py-1 rounded-full">
                      {tab.count}
                    </span>
                  )}
                </button>
              );
            })}
          </nav>
        </div>

        {/* Tab Content */}
        <div className="tab-content">
          {activeTab === 'overview' && (
            <div className="space-y-8">
              {/* Quick Stats Grid */}
              <div className="grid grid-cols-2 md:grid-cols-4 lg:grid-cols-6 gap-4">
                <div className="card text-center">
                  <Users className="w-6 h-6 text-dnd-purple mx-auto mb-2" />
                  <div className="text-xl font-bold text-white">{entities.length}</div>
                  <div className="text-gray-400 text-sm">NPCs</div>
                </div>
                <div className="card text-center">
                  <MapPin className="w-6 h-6 text-dnd-gold mx-auto mb-2" />
                  <div className="text-xl font-bold text-white">{locations.length}</div>
                  <div className="text-gray-400 text-sm">Locations</div>
                </div>
                <div className="card text-center">
                  <Scroll className="w-6 h-6 text-green-400 mx-auto mb-2" />
                  <div className="text-xl font-bold text-white">{quests.length}</div>
                  <div className="text-gray-400 text-sm">Quests</div>
                </div>
                <div className="card text-center">
                  <Sword className="w-6 h-6 text-red-400 mx-auto mb-2" />
                  <div className="text-xl font-bold text-white">{encounters.length}</div>
                  <div className="text-gray-400 text-sm">Encounters</div>
                </div>
                <div className="card text-center">
                  <Crown className="w-6 h-6 text-yellow-400 mx-auto mb-2" />
                  <div className="text-xl font-bold text-white">{deities.length}</div>
                  <div className="text-gray-400 text-sm">Deities</div>
                </div>
                <div className="card text-center">
                  <Globe className="w-6 h-6 text-blue-400 mx-auto mb-2" />
                  <div className="text-xl font-bold text-white">{planes.length}</div>
                  <div className="text-gray-400 text-sm">Planes</div>
                </div>
              </div>

              {/* Campaign Configuration */}
              <div className="grid md:grid-cols-2 gap-6">
                <div className="card">
                  <h3 className="text-lg font-bold text-white mb-4">Campaign Details</h3>
                  <div className="space-y-3 text-sm">
                    {campaign.progression_type && (
                      <div className="flex justify-between">
                        <span className="text-gray-400">Progression:</span>
                        <span className="text-white capitalize">{campaign.progression_type}</span>
                      </div>
                    )}
                    {campaign.difficulty && (
                      <div className="flex justify-between">
                        <span className="text-gray-400">Difficulty:</span>
                        <span className="text-white capitalize">{campaign.difficulty}</span>
                      </div>
                    )}
                    {campaign.starting_level && (
                      <div className="flex justify-between">
                        <span className="text-gray-400">Starting Level:</span>
                        <span className="text-white">{campaign.starting_level}</span>
                      </div>
                    )}
                    {campaign.campaign_length && (
                      <div className="flex justify-between">
                        <span className="text-gray-400">Length:</span>
                        <span className="text-white capitalize">{campaign.campaign_length}</span>
                      </div>
                    )}
                    {campaign.tone && (
                      <div className="flex justify-between">
                        <span className="text-gray-400">Tone:</span>
                        <span className="text-white capitalize">{campaign.tone}</span>
                      </div>
                    )}
                  </div>
                </div>

                <div className="card">
                  <h3 className="text-lg font-bold text-white mb-4">World Summary</h3>
                  <div className="space-y-3 text-sm">
                    <div className="flex justify-between">
                      <span className="text-gray-400">Races Available:</span>
                      <span className="text-white">{races.length}</span>
                    </div>
                    <div className="flex justify-between">
                      <span className="text-gray-400">Classes Available:</span>
                      <span className="text-white">{classes.length}</span>
                    </div>
                    <div className="flex justify-between">
                      <span className="text-gray-400">Languages:</span>
                      <span className="text-white">{languages.length}</span>
                    </div>
                    <div className="flex justify-between">
                      <span className="text-gray-400">Pantheons:</span>
                      <span className="text-white">{pantheons.length}</span>
                    </div>
                    <div className="flex justify-between">
                      <span className="text-gray-400">Regions:</span>
                      <span className="text-white">{geographyRegions.length}</span>
                    </div>
                  </div>
                </div>
              </div>

              {/* Additional Notes */}
              {campaign.additional_notes && (
                <div className="card">
                  <h3 className="text-lg font-bold text-white mb-4">Additional Notes</h3>
                  <p className="text-gray-300 whitespace-pre-wrap">{campaign.additional_notes}</p>
                </div>
              )}
            </div>
          )}

          {activeTab === 'entities' && (
            <div className="grid gap-6 md:grid-cols-2 xl:grid-cols-3">
              {entities.length > 0 ? (
                entities.map((entity) => (
                  <EntityCard key={entity.id} entity={entity} />
                ))
              ) : (
                <div className="col-span-full text-center py-12">
                  <Users className="w-16 h-16 text-gray-500 mx-auto mb-4" />
                  <h3 className="text-xl font-bold text-white mb-2">No Entities Yet</h3>
                  <p className="text-gray-400">
                    NPCs and entities will appear here once the campaign generation is complete.
                  </p>
                </div>
              )}
            </div>
          )}

          {activeTab === 'locations' && (
            <div className="grid gap-6 lg:grid-cols-2">
              {locations.length > 0 ? (
                locations.map((location) => (
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
            <div className="space-y-6">
              {/* Quest Hooks */}
              <div>
                <h3 className="text-xl font-bold text-white mb-4">Quest Hooks</h3>
                <div className="grid gap-6 md:grid-cols-2 xl:grid-cols-3">
                  {quests.length > 0 ? (
                    quests.map((quest) => (
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
              </div>

              {/* Encounters */}
              {encounters.length > 0 && (
                <div>
                  <h3 className="text-xl font-bold text-white mb-4">Encounters</h3>
                  <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
                    {encounters.map((encounter) => (
                      <div key={encounter.id} className="card">
                        <div className="flex items-start justify-between mb-2">
                          <h4 className="font-semibold text-white">{encounter.name}</h4>
                          <span className={`px-2 py-1 text-xs rounded-full ${
                            encounter.difficulty === 'easy' ? 'bg-green-500 bg-opacity-20 text-green-400' :
                            encounter.difficulty === 'medium' ? 'bg-yellow-500 bg-opacity-20 text-yellow-400' :
                            encounter.difficulty === 'hard' ? 'bg-red-500 bg-opacity-20 text-red-400' :
                            'bg-purple-500 bg-opacity-20 text-purple-400'
                          }`}>
                            {encounter.difficulty}
                          </span>
                        </div>
                        <p className="text-gray-300 text-sm mb-3">{encounter.description}</p>
                        {encounter.encounter_type && (
                          <div className="text-gray-400 text-xs">
                            Type: {encounter.encounter_type}
                          </div>
                        )}
                      </div>
                    ))}
                  </div>
                </div>
              )}
            </div>
          )}

          {activeTab === 'world' && (
            <WorldBuildingCard 
              pantheons={pantheons}
              deities={deities}
              planes={planes}
              geographyRegions={geographyRegions}
              economicSystems={economicSystems}
              legalSystems={legalSystems}
              historicalPeriods={historicalPeriods}
              calendarSystems={calendarSystems}
            />
          )}

          {activeTab === 'characters' && (
            <CharacterBuildingCard 
              races={races}
              classes={classes}
              backgrounds={backgrounds}
              languages={languages}
            />
          )}

          {activeTab === 'relationships' && (
            <div className="space-y-6">
              {relationshipsData?.entity_relationships && relationshipsData.entity_relationships.length > 0 ? (
                <div>
                  <h3 className="text-xl font-bold text-white mb-4">Entity Relationships</h3>
                  <div className="grid gap-4 md:grid-cols-2">
                    {relationshipsData.entity_relationships.map((rel) => (
                      <div key={rel.id} className="card">
                        <div className="flex items-center justify-between mb-2">
                          <span className="text-white font-medium">{rel.relationship_type}</span>
                          <span className="text-gray-400 text-sm">
                            Strength: {rel.relationship_strength}
                          </span>
                        </div>
                        <p className="text-gray-300 text-sm">{rel.description}</p>
                        <div className="mt-2 text-xs text-gray-400">
                          Entity {rel.entity1_id} ↔ Entity {rel.entity2_id}
                        </div>
                      </div>
                    ))}
                  </div>
                </div>
              ) : (
                <div className="text-center py-12">
                  <Crown className="w-16 h-16 text-gray-500 mx-auto mb-4" />
                  <h3 className="text-xl font-bold text-white mb-2">No Relationships Yet</h3>
                  <p className="text-gray-400">
                    Entity relationships will appear here once the campaign generation is complete.
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