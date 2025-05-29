'use client';

import { useState } from 'react';
import { ChevronLeft, ChevronRight, Sparkles } from 'lucide-react';
import { useCampaignStore } from '@/stores/campaignStore';

const TOTAL_STEPS = 4;

export function CampaignWizard() {
  const {
    currentStep,
    name,
    setting,
    themes,
    playerCharacters,
    setName,
    setSetting,
    setThemes,
    setCurrentStep,
    nextStep,
    prevStep,
    isStepValid,
    reset,
  } = useCampaignStore();

  const [isGenerating, setIsGenerating] = useState(false);

  const availableThemes = [
    'political intrigue',
    'war',
    'mystery',
    'exploration',
    'horror',
    'comedy',
    'romance',
    'apocalyptic',
    'urban',
    'wilderness',
    'underdark',
    'planar travel',
  ];

  const handleThemeToggle = (theme: string) => {
    const newThemes = themes.includes(theme)
      ? themes.filter(t => t !== theme)
      : [...themes, theme];
    setThemes(newThemes);
  };

  const handleGenerate = async () => {
    setIsGenerating(true);
    
    // TODO: Replace with actual API call to create campaign
    await new Promise(resolve => setTimeout(resolve, 2000));
    
    // Mock campaign creation - redirect to generation page
    const mockCampaignId = Math.floor(Math.random() * 1000);
    window.location.href = `/campaigns/${mockCampaignId}/generating`;
  };

  const renderProgressBar = () => (
    <div className="mb-8">
      <div className="flex justify-between text-sm text-gray-400 mb-2">
        <span>Step {currentStep + 1} of {TOTAL_STEPS}</span>
        <span>{Math.round(((currentStep + 1) / TOTAL_STEPS) * 100)}%</span>
      </div>
      <div className="w-full bg-gray-700 rounded-full h-2">
        <div
          className="bg-dnd-purple h-2 rounded-full transition-all duration-300"
          style={{ width: `${((currentStep + 1) / TOTAL_STEPS) * 100}%` }}
        />
      </div>
    </div>
  );

  const renderStep = () => {
    switch (currentStep) {
      case 0:
        return (
          <div className="space-y-6">
            <div>
              <h2 className="text-2xl font-bold text-white mb-4">Basic Information</h2>
              <p className="text-gray-400 mb-6">Let&apos;s start with the basics for your campaign.</p>
            </div>
            
            <div>
              <label className="block text-sm font-medium text-gray-300 mb-2">
                Campaign Name *
              </label>
              <input
                type="text"
                value={name}
                onChange={(e) => setName(e.target.value)}
                placeholder="Enter a memorable name for your campaign"
                className="input w-full"
                required
              />
            </div>
          </div>
        );

      case 1:
        return (
          <div className="space-y-6">
            <div>
              <h2 className="text-2xl font-bold text-white mb-4">Player Characters</h2>
              <p className="text-gray-400 mb-6">
                Tell us about the player characters that will be part of this campaign.
              </p>
            </div>
            
            <div className="card">
              <p className="text-gray-300 text-center">
                Player character management will be implemented in Phase 7.
                For now, this step is considered valid by default.
              </p>
            </div>
          </div>
        );

      case 2:
        return (
          <div className="space-y-6">
            <div>
              <h2 className="text-2xl font-bold text-white mb-4">World & Themes</h2>
              <p className="text-gray-400 mb-6">
                Describe your campaign setting and choose themes that will shape the story.
              </p>
            </div>
            
            <div>
              <label className="block text-sm font-medium text-gray-300 mb-2">
                Campaign Setting *
              </label>
              <textarea
                value={setting}
                onChange={(e) => setSetting(e.target.value)}
                placeholder="Describe the world, time period, and general atmosphere..."
                className="textarea w-full h-32"
                required
              />
            </div>

            <div>
              <label className="block text-sm font-medium text-gray-300 mb-4">
                Themes * (Select at least one)
              </label>
              <div className="grid grid-cols-2 md:grid-cols-3 gap-3">
                {availableThemes.map((theme) => (
                  <button
                    key={theme}
                    onClick={() => handleThemeToggle(theme)}
                    className={`p-3 rounded-lg border text-sm font-medium transition-all ${
                      themes.includes(theme)
                        ? 'bg-dnd-purple border-dnd-purple text-white'
                        : 'bg-gray-800 border-gray-700 text-gray-300 hover:border-gray-600'
                    }`}
                  >
                    {theme}
                  </button>
                ))}
              </div>
            </div>
          </div>
        );

      case 3:
        return (
          <div className="space-y-6">
            <div>
              <h2 className="text-2xl font-bold text-white mb-4">Review & Generate</h2>
              <p className="text-gray-400 mb-6">
                Review your campaign details and generate your AI-powered D&D campaign.
              </p>
            </div>

            <div className="space-y-4">
              <div className="card">
                <h3 className="font-semibold text-white mb-2">Campaign Name</h3>
                <p className="text-gray-300">{name}</p>
              </div>

              <div className="card">
                <h3 className="font-semibold text-white mb-2">Setting</h3>
                <p className="text-gray-300">{setting}</p>
              </div>

              <div className="card">
                <h3 className="font-semibold text-white mb-2">Themes</h3>
                <div className="flex flex-wrap gap-2">
                  {themes.map((theme) => (
                    <span
                      key={theme}
                      className="px-2 py-1 bg-dnd-purple bg-opacity-20 text-dnd-purple text-sm rounded-full"
                    >
                      {theme}
                    </span>
                  ))}
                </div>
              </div>
            </div>

            <div className="pt-4">
              <button
                onClick={handleGenerate}
                disabled={isGenerating || !isStepValid(3)}
                className="btn-primary w-full py-3 text-lg disabled:opacity-50 disabled:cursor-not-allowed"
              >
                {isGenerating ? (
                  <>
                    <Sparkles className="mr-2 h-5 w-5 animate-spin" />
                    Generating Campaign...
                  </>
                ) : (
                  <>
                    <Sparkles className="mr-2 h-5 w-5" />
                    Generate Campaign
                  </>
                )}
              </button>
            </div>
          </div>
        );

      default:
        return null;
    }
  };

  const renderNavigation = () => {
    if (currentStep === 3) return null; // Hide navigation on final step

    return (
      <div className="flex justify-between pt-6">
        <button
          onClick={prevStep}
          disabled={currentStep === 0}
          className="btn-secondary disabled:opacity-50 disabled:cursor-not-allowed"
        >
          <ChevronLeft className="mr-2 h-4 w-4" />
          Previous
        </button>

        <button
          onClick={nextStep}
          disabled={!isStepValid(currentStep)}
          className="btn-primary disabled:opacity-50 disabled:cursor-not-allowed"
        >
          Next
          <ChevronRight className="ml-2 h-4 w-4" />
        </button>
      </div>
    );
  };

  return (
    <div className="max-w-2xl mx-auto">
      {renderProgressBar()}
      
      <div className="card">
        {renderStep()}
        {renderNavigation()}
      </div>
    </div>
  );
}