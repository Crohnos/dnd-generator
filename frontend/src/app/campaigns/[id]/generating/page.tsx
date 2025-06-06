'use client';

import { useParams, useRouter } from 'next/navigation';
import { useEffect, useState } from 'react';
import { Wand2, Users, MapPin, Scroll, CheckCircle, AlertCircle, Globe, Crown, Heart, Sword, Building } from 'lucide-react';
import { useCampaignProgressSubscription } from '@/generated/graphql';

export default function GeneratingPage() {
  const params = useParams();
  const router = useRouter();
  const [progress, setProgress] = useState(0);
  const [currentPhase, setCurrentPhase] = useState(0);
  const [error, setError] = useState<string | null>(null);
  
  const campaignId = parseInt(params.id as string);

  const phases = [
    { name: 'Core World Systems', icon: Globe, description: 'Creating calendar systems, history, and geography' },
    { name: 'Character Building', icon: Users, description: 'Establishing races, classes, and character options' },
    { name: 'Social Framework', icon: Crown, description: 'Building cultures, factions, and pantheons' },
    { name: 'PC-Connected Entities', icon: Heart, description: 'Generating NPCs from character backstories' },
    { name: 'PC-Connected Locations', icon: MapPin, description: 'Creating locations tied to player characters' },
    { name: 'PC-Connected Items', icon: Sword, description: 'Crafting items relevant to character stories' },
    { name: 'Quest Hooks & Encounters', icon: Scroll, description: 'Designing adventures and challenges' },
    { name: 'World Population', icon: Building, description: 'Adding shops, taverns, temples, and NPCs' },
    { name: 'Final Relationships', icon: CheckCircle, description: 'Connecting all entities and locations' },
  ];

  // Subscribe to campaign status updates
  const [{ data, fetching, error: subscriptionError }] = useCampaignProgressSubscription({
    variables: { id: campaignId },
    pause: !campaignId,
  });

  useEffect(() => {
    if (subscriptionError) {
      setError('Failed to connect to generation service');
    }
  }, [subscriptionError]);

  useEffect(() => {
    if (data?.campaigns_by_pk) {
      const campaign = data.campaigns_by_pk;

      if (campaign.status === 'completed') {
        // Campaign is completed, show final phase and set progress to 100%
        setCurrentPhase(phases.length - 1);
        setProgress(100);
        setTimeout(() => {
          router.push(`/campaigns/${campaignId}`);
        }, 1000);
      } else if (campaign.status === 'error') {
        setError(campaign.error_message || 'Failed to generate campaign. Please try again.');
      } else if (campaign.status === 'generating') {
        // Use real backend progress data
        const currentPhaseIndex = Math.max(0, Number(campaign.generation_phase || 1) - 1);
        const phaseProgress = campaign.phase_progress || 0;
        const totalPhases = campaign.total_phases || 9;
        
        // Calculate overall progress: (completed phases + current phase progress) / total phases
        const overallProgress = Math.min(99, ((currentPhaseIndex + phaseProgress / 100) / totalPhases) * 100);
        
        setCurrentPhase(Math.min(currentPhaseIndex, phases.length - 1));
        setProgress(overallProgress);
      }
    }
  }, [data, campaignId, router, phases.length]);

  // Progress is now handled by real backend data via the subscription effect above

  // Trigger generation on mount
  useEffect(() => {
    let isMounted = true;
    const triggerGeneration = async () => {
      try {
        if (!isMounted) return;
        
        await fetch(`${process.env.NEXT_PUBLIC_API_URL}/api/campaigns/${campaignId}/generate`, {
          method: 'POST',
        });
      } catch (err) {
        console.error('Failed to trigger generation:', err);
      }
    };

    if (campaignId && !isNaN(campaignId)) {
      triggerGeneration();
    }
    
    return () => {
      isMounted = false;
    };
  }, [campaignId]);

  if (error) {
    return (
      <div className="min-h-screen flex items-center justify-center py-8 px-4 sm:px-6 lg:px-8">
        <div className="text-center">
          <AlertCircle className="w-16 h-16 text-red-500 mx-auto mb-4" />
          <h1 className="text-2xl font-bold mb-2">Generation Failed</h1>
          <p className="text-gray-400 mb-6">{error}</p>
          <button
            onClick={() => router.push('/campaigns/new')}
            className="btn-primary"
          >
            Try Again
          </button>
        </div>
      </div>
    );
  }

  return (
    <div className="min-h-screen flex items-center justify-center py-8 px-4 sm:px-6 lg:px-8">
      <div className="max-w-lg w-full">
        <div className="text-center mb-8">
          <div className="flex justify-center mb-6">
            <Wand2 className="h-16 w-16 text-dnd-purple animate-pulse" />
          </div>
          <h1 className="text-3xl font-bold text-white mb-4">
            Generating Your Campaign
          </h1>
          <p className="text-gray-400">
            Our AI is crafting a unique D&D experience for you. This will take just a few moments.
          </p>
        </div>

        {/* Progress Bar */}
        <div className="mb-8">
          <div className="flex justify-between text-sm text-gray-400 mb-2">
            <span>Progress</span>
            <span>{Math.round(progress)}%</span>
          </div>
          <div className="w-full bg-gray-700 rounded-full h-3">
            <div 
              className="bg-gradient-to-r from-dnd-purple to-dnd-gold h-3 rounded-full transition-all duration-300 ease-out"
              style={{ width: `${progress}%` }}
            />
          </div>
        </div>

        {/* Current Phase */}
        <div className="space-y-4">
          {phases.map((phase, index) => {
            const Icon = phase.icon;
            const isActive = index === currentPhase;
            const isCompleted = index < currentPhase;
            
            return (
              <div 
                key={index}
                className={`flex items-start space-x-3 p-4 rounded-lg transition-all duration-300 ${
                  isActive ? 'bg-dnd-purple bg-opacity-20 border border-dnd-purple' :
                  isCompleted ? 'bg-green-500 bg-opacity-20 border border-green-500' :
                  'bg-gray-800 border border-gray-700'
                }`}
              >
                <div className={`flex-shrink-0 ${
                  isActive ? 'text-dnd-purple' :
                  isCompleted ? 'text-green-400' :
                  'text-gray-500'
                }`}>
                  <Icon className={`h-6 w-6 ${isActive ? 'animate-pulse' : ''}`} />
                </div>
                <div>
                  <h3 className={`font-medium ${
                    isActive ? 'text-dnd-purple' :
                    isCompleted ? 'text-green-400' :
                    'text-gray-400'
                  }`}>
                    {phase.name}
                    {isCompleted && ' ✓'}
                  </h3>
                  <p className="text-sm text-gray-400 mt-1">
                    {phase.description}
                  </p>
                </div>
              </div>
            );
          })}
        </div>

        {/* Feature Highlights */}
        <div className="mt-12 space-y-6">
          <h3 className="text-lg font-semibold text-white text-center">
            What&apos;s Being Generated
          </h3>
          <div className="grid grid-cols-3 gap-4 text-center">
            <div>
              <Users className="h-8 w-8 text-dnd-purple mx-auto mb-2" />
              <div className="text-sm text-gray-300">8-12 NPCs</div>
              <div className="text-xs text-gray-500">with personalities & secrets</div>
            </div>
            <div>
              <MapPin className="h-8 w-8 text-dnd-gold mx-auto mb-2" />
              <div className="text-sm text-gray-300">6-10 Locations</div>
              <div className="text-xs text-gray-500">with rich descriptions</div>
            </div>
            <div>
              <Scroll className="h-8 w-8 text-dnd-purple mx-auto mb-2" />
              <div className="text-sm text-gray-300">5-8 Quest Hooks</div>
              <div className="text-xs text-gray-500">interconnected storylines</div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}