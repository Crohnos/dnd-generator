'use client';

import Link from 'next/link';
import { Plus, Calendar, Sparkles, Users, MapPin, Scroll } from 'lucide-react';

// Mock data for development - will be replaced with GraphQL queries
const mockCampaigns = [
  {
    id: 1,
    name: 'The Shattered Crown',
    setting: 'A war-torn kingdom where ancient magic clashes with political intrigue',
    themes: ['political intrigue', 'war', 'mystery'],
    status: 'ready' as const,
    created_at: '2024-01-15T10:30:00Z',
    npc_count: 8,
    location_count: 6,
    quest_count: 5,
  },
  {
    id: 2,
    name: 'Shadows of the Underdark',
    setting: 'Deep underground tunnels filled with ancient secrets and dark creatures',
    themes: ['horror', 'exploration', 'underdark'],
    status: 'generating' as const,
    created_at: '2024-01-16T14:22:00Z',
    npc_count: 0,
    location_count: 0,
    quest_count: 0,
  },
];

function CampaignCard({ campaign }: { campaign: typeof mockCampaigns[0] }) {
  const formatDate = (dateString: string) => {
    return new Date(dateString).toLocaleDateString('en-US', {
      year: 'numeric',
      month: 'short',
      day: 'numeric',
    });
  };

  const getStatusBadge = (status: string) => {
    switch (status) {
      case 'ready':
        return (
          <span className="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-green-100 text-green-800">
            Ready
          </span>
        );
      case 'generating':
        return (
          <span className="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-yellow-100 text-yellow-800">
            <Sparkles className="w-3 h-3 mr-1 animate-spin" />
            Generating
          </span>
        );
      case 'error':
        return (
          <span className="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-red-100 text-red-800">
            Error
          </span>
        );
      default:
        return null;
    }
  };

  return (
    <Link href={`/campaigns/${campaign.id}`}>
      <div className="card hover:bg-gray-750 cursor-pointer">
        <div className="flex justify-between items-start mb-4">
          <div>
            <h3 className="text-xl font-bold text-white mb-2">{campaign.name}</h3>
            {getStatusBadge(campaign.status)}
          </div>
          <div className="text-sm text-gray-400">
            <Calendar className="w-4 h-4 inline mr-1" />
            {formatDate(campaign.created_at)}
          </div>
        </div>

        <p className="text-gray-300 mb-4 line-clamp-2">{campaign.setting}</p>

        <div className="flex flex-wrap gap-2 mb-4">
          {campaign.themes.map((theme) => (
            <span
              key={theme}
              className="px-2 py-1 bg-dnd-purple bg-opacity-20 text-dnd-purple text-xs rounded-full"
            >
              {theme}
            </span>
          ))}
        </div>

        {campaign.status === 'ready' && (
          <div className="flex justify-between text-sm text-gray-400">
            <span className="flex items-center">
              <Users className="w-4 h-4 mr-1" />
              {campaign.npc_count} NPCs
            </span>
            <span className="flex items-center">
              <MapPin className="w-4 h-4 mr-1" />
              {campaign.location_count} Locations
            </span>
            <span className="flex items-center">
              <Scroll className="w-4 h-4 mr-1" />
              {campaign.quest_count} Quests
            </span>
          </div>
        )}
      </div>
    </Link>
  );
}

function EmptyState() {
  return (
    <div className="text-center py-16">
      <Sparkles className="mx-auto h-16 w-16 text-gray-400 mb-6" />
      <h3 className="text-xl font-bold text-white mb-4">No campaigns yet</h3>
      <p className="text-gray-400 mb-8 max-w-md mx-auto">
        Create your first AI-generated D&D campaign to get started. 
        It only takes a few minutes to generate a complete world.
      </p>
      <Link href="/campaigns/new" className="btn-primary">
        <Plus className="mr-2 h-5 w-5" />
        Create Your First Campaign
      </Link>
    </div>
  );
}

export default function CampaignsPage() {
  // TODO: Replace with actual GraphQL query
  const campaigns = mockCampaigns;

  return (
    <div className="min-h-screen py-8 px-4 sm:px-6 lg:px-8">
      <div className="max-w-7xl mx-auto">
        <div className="flex justify-between items-center mb-8">
          <div>
            <h1 className="text-3xl font-bold text-white">Your Campaigns</h1>
            <p className="text-gray-400 mt-2">
              Manage your AI-generated D&D campaigns
            </p>
          </div>
          <Link href="/campaigns/new" className="btn-primary">
            <Plus className="mr-2 h-5 w-5" />
            New Campaign
          </Link>
        </div>

        {campaigns.length === 0 ? (
          <EmptyState />
        ) : (
          <div className="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
            {campaigns.map((campaign) => (
              <CampaignCard key={campaign.id} campaign={campaign} />
            ))}
          </div>
        )}
      </div>
    </div>
  );
}