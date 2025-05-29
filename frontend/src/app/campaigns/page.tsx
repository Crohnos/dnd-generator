'use client';

import Link from 'next/link';
import { Plus, Calendar, Sparkles, Users, MapPin, Scroll, AlertCircle } from 'lucide-react';
import { useGetCampaignsQuery } from '@/generated/graphql';

type Campaign = {
  id: number;
  name: string;
  setting?: string | null;
  themes?: string[] | null;
  status: string;
  created_at: string;
  npcs_aggregate: { aggregate: { count: number } | null } | null;
  locations_aggregate: { aggregate: { count: number } | null } | null;
  quest_hooks_aggregate: { aggregate: { count: number } | null } | null;
};

function CampaignCard({ campaign }: { campaign: Campaign }) {
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

        {campaign.setting && (
          <p className="text-gray-300 mb-4 line-clamp-2">{campaign.setting}</p>
        )}

        {campaign.themes && campaign.themes.length > 0 && (
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
        )}

        {campaign.status === 'ready' && (
          <div className="flex justify-between text-sm text-gray-400">
            <span className="flex items-center">
              <Users className="w-4 h-4 mr-1" />
              {campaign.npcs_aggregate?.aggregate?.count || 0} NPCs
            </span>
            <span className="flex items-center">
              <MapPin className="w-4 h-4 mr-1" />
              {campaign.locations_aggregate?.aggregate?.count || 0} Locations
            </span>
            <span className="flex items-center">
              <Scroll className="w-4 h-4 mr-1" />
              {campaign.quest_hooks_aggregate?.aggregate?.count || 0} Quests
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
  const [{ data, fetching, error }] = useGetCampaignsQuery();

  if (error) {
    return (
      <div className="min-h-screen flex items-center justify-center py-8 px-4 sm:px-6 lg:px-8">
        <div className="text-center max-w-lg">
          <AlertCircle className="w-16 h-16 text-red-500 mx-auto mb-4" />
          <h1 className="text-2xl font-bold text-white mb-2">Failed to Load Campaigns</h1>
          <p className="text-gray-400 mb-4">
            There was an error connecting to the GraphQL endpoint.
          </p>
          <div className="text-left bg-gray-800 p-4 rounded mb-6">
            <p className="text-sm text-gray-300 mb-2">Error details:</p>
            <p className="text-xs text-red-400 font-mono">{error.message}</p>
          </div>
          <div className="text-sm text-gray-400 mb-6">
            <p>Make sure:</p>
            <ul className="list-disc list-inside mt-2 space-y-1">
              <li>Hasura is running on localhost:8080</li>
              <li>The Next.js dev server has been restarted</li>
              <li>Environment variables are set correctly</li>
            </ul>
          </div>
          <button
            onClick={() => window.location.reload()}
            className="btn-primary"
          >
            Refresh Page
          </button>
        </div>
      </div>
    );
  }

  const campaigns = data?.campaigns || [];

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

        {fetching ? (
          <div className="flex items-center justify-center py-16">
            <Sparkles className="w-8 h-8 text-dnd-purple animate-spin mr-3" />
            <span className="text-gray-400">Loading campaigns...</span>
          </div>
        ) : campaigns.length === 0 ? (
          <EmptyState />
        ) : (
          <div className="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
            {campaigns.map((campaign) => (
              <CampaignCard key={campaign.id} campaign={campaign as Campaign} />
            ))}
          </div>
        )}
      </div>
    </div>
  );
}