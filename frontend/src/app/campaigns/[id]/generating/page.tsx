'use client';

import { useParams, useRouter } from 'next/navigation';
import { useEffect, useState } from 'react';
import { Wand2, Users, MapPin, Scroll, CheckCircle } from 'lucide-react';

export default function GeneratingPage() {
  const params = useParams();
  const router = useRouter();
  const [progress, setProgress] = useState(0);
  const [currentPhase, setCurrentPhase] = useState(0);

  const phases = [
    { name: 'Generating NPCs', icon: Users, description: 'Creating compelling characters with rich backgrounds' },
    { name: 'Creating Locations', icon: MapPin, description: 'Building immersive locations and environments' },
    { name: 'Crafting Quest Hooks', icon: Scroll, description: 'Designing engaging quests and storylines' },
    { name: 'Finalizing Campaign', icon: CheckCircle, description: 'Connecting everything together' },
  ];

  useEffect(() => {
    // Simulate generation progress
    const interval = setInterval(() => {
      setProgress((prev) => {
        if (prev >= 100) {
          clearInterval(interval);
          // Redirect to campaign page when complete
          setTimeout(() => {
            router.push(`/campaigns/${params.id}`);
          }, 1000);
          return 100;
        }
        
        const newProgress = prev + Math.random() * 5;
        const phase = Math.floor((newProgress / 100) * phases.length);
        setCurrentPhase(Math.min(phase, phases.length - 1));
        
        return Math.min(newProgress, 100);
      });
    }, 200);

    return () => clearInterval(interval);
  }, [params.id, router, phases.length]);

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