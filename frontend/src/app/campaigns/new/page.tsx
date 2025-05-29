'use client';

import { CampaignWizard } from '@/components/CampaignWizard';

export default function NewCampaignPage() {
  return (
    <div className="min-h-screen py-8 px-4 sm:px-6 lg:px-8">
      <div className="max-w-4xl mx-auto">
        <div className="text-center mb-8">
          <h1 className="text-3xl font-bold text-white mb-4">Create New Campaign</h1>
          <p className="text-gray-400">
            Follow the steps below to generate your AI-powered D&D campaign
          </p>
        </div>
        
        <CampaignWizard />
      </div>
    </div>
  );
}