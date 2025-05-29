'use client';

import { useState } from 'react';
import { useRouter } from 'next/navigation';
import { useCampaignStore, PlayerCharacter } from '@/stores/campaignStore';
import { ChevronLeft, ChevronRight, Plus, Trash2, Wand2 } from 'lucide-react';
import clsx from 'clsx';

const WIZARD_STEPS = [
  { id: 0, name: 'Basic Information', description: 'Name your campaign' },
  { id: 1, name: 'Player Characters', description: 'Add your players' },
  { id: 2, name: 'World & Themes', description: 'Set the stage' },
  { id: 3, name: 'Review & Generate', description: 'Create your campaign' },
];

const CHARACTER_CLASSES = [
  'Fighter', 'Wizard', 'Rogue', 'Cleric', 'Ranger', 'Paladin', 
  'Barbarian', 'Sorcerer', 'Warlock', 'Druid', 'Monk', 'Bard'
];

const CHARACTER_RACES = [
  'Human', 'Elf', 'Dwarf', 'Halfling', 'Dragonborn', 'Gnome', 
  'Half-Elf', 'Half-Orc', 'Tiefling'
];

const CAMPAIGN_THEMES = [
  'Political Intrigue', 'War', 'Mystery', 'Exploration', 'Horror', 
  'Comedy', 'Romance', 'Apocalyptic', 'Urban', 'Wilderness', 
  'Underdark', 'Planar Travel'
];

export function CampaignWizard() {
  const router = useRouter();
  const [isGenerating, setIsGenerating] = useState(false);
  const [error, setError] = useState<string | null>(null);
  
  const {
    currentStep,
    name,
    setting,
    themes,
    playerCharacters,
    setName,
    setSetting,
    setThemes,
    addPlayerCharacter,
    updatePlayerCharacter,
    removePlayerCharacter,
    nextStep,
    prevStep,
    isStepValid,
    reset,
  } = useCampaignStore();

  const handleGenerate = async () => {
    setIsGenerating(true);
    setError(null);

    const campaignData = {
      name,
      setting,
      themes,
      player_characters: playerCharacters,
    };

    console.log('=== CAMPAIGN WIZARD SUBMISSION ===');
    console.log('Campaign Data:', JSON.stringify(campaignData, null, 2));
    console.log('==================================');

    try {
      // Create campaign via backend API
      const response = await fetch(`${process.env.NEXT_PUBLIC_API_URL}/api/campaigns`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(campaignData),
      });

      if (!response.ok) {
        throw new Error('Failed to create campaign');
      }

      const campaign = await response.json();
      
      // Reset the wizard state
      reset();
      
      // Redirect to generation progress page
      router.push(`/campaigns/${campaign.id}/generating`);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'An error occurred');
      setIsGenerating(false);
    }
  };

  const renderStepContent = () => {
    switch (currentStep) {
      case 0:
        return <StepBasicInfo name={name} setName={setName} />;
      case 1:
        return (
          <StepPlayerCharacters
            characters={playerCharacters}
            onAdd={addPlayerCharacter}
            onUpdate={updatePlayerCharacter}
            onRemove={removePlayerCharacter}
          />
        );
      case 2:
        return (
          <StepWorldThemes
            setting={setting}
            themes={themes}
            setSetting={setSetting}
            setThemes={setThemes}
          />
        );
      case 3:
        return (
          <StepReview
            name={name}
            setting={setting}
            themes={themes}
            playerCharacters={playerCharacters}
          />
        );
      default:
        return null;
    }
  };

  return (
    <div className="max-w-4xl mx-auto">
      {/* Progress Bar */}
      <div className="mb-8">
        <div className="flex items-center justify-between mb-4">
          {WIZARD_STEPS.map((step, index) => (
            <div
              key={step.id}
              className={clsx(
                'flex items-center',
                index < WIZARD_STEPS.length - 1 && 'flex-1'
              )}
            >
              <div className="flex flex-col items-center">
                <div
                  className={clsx(
                    'w-10 h-10 rounded-full flex items-center justify-center text-sm font-semibold',
                    currentStep >= step.id
                      ? 'bg-dnd-purple text-white'
                      : 'bg-gray-700 text-gray-400'
                  )}
                >
                  {step.id + 1}
                </div>
                <span className="text-xs mt-1 text-gray-400">{step.name}</span>
              </div>
              {index < WIZARD_STEPS.length - 1 && (
                <div
                  className={clsx(
                    'flex-1 h-1 mx-2',
                    currentStep > step.id ? 'bg-dnd-purple' : 'bg-gray-700'
                  )}
                />
              )}
            </div>
          ))}
        </div>
        <h2 className="text-2xl font-bold text-center mb-2">
          {WIZARD_STEPS[currentStep].name}
        </h2>
        <p className="text-center text-gray-400">
          {WIZARD_STEPS[currentStep].description}
        </p>
      </div>

      {/* Step Content */}
      <div className="card p-8 mb-8">{renderStepContent()}</div>

      {/* Error Message */}
      {error && (
        <div className="bg-red-900/50 border border-red-700 text-red-200 p-4 rounded mb-4">
          {error}
        </div>
      )}

      {/* Navigation */}
      <div className="flex justify-between">
        <button
          onClick={prevStep}
          disabled={currentStep === 0}
          className="btn-secondary flex items-center gap-2"
        >
          <ChevronLeft className="w-4 h-4" />
          Previous
        </button>

        {currentStep < 3 ? (
          <button
            onClick={nextStep}
            disabled={!isStepValid(currentStep)}
            className="btn-primary flex items-center gap-2"
          >
            Next
            <ChevronRight className="w-4 h-4" />
          </button>
        ) : (
          <button
            onClick={handleGenerate}
            disabled={!isStepValid(currentStep) || isGenerating}
            className="btn-primary flex items-center gap-2"
          >
            {isGenerating ? (
              <>
                <div className="animate-spin rounded-full h-4 w-4 border-b-2 border-white" />
                Generating...
              </>
            ) : (
              <>
                <Wand2 className="w-4 h-4" />
                Generate Campaign
              </>
            )}
          </button>
        )}
      </div>
    </div>
  );
}

// Step 0: Basic Information
function StepBasicInfo({
  name,
  setName,
}: {
  name: string;
  setName: (name: string) => void;
}) {
  return (
    <div>
      <label htmlFor="campaign-name" className="block text-sm font-medium mb-2">
        Campaign Name <span className="text-red-500">*</span>
      </label>
      <input
        id="campaign-name"
        type="text"
        value={name}
        onChange={(e) => setName(e.target.value)}
        placeholder="Enter your campaign name..."
        className="input w-full"
        autoFocus
      />
      <p className="text-sm text-gray-400 mt-2">
        Choose a memorable name for your campaign
      </p>
    </div>
  );
}

// Step 1: Player Characters
function StepPlayerCharacters({
  characters,
  onAdd,
  onUpdate,
  onRemove,
}: {
  characters: PlayerCharacter[];
  onAdd: (character: PlayerCharacter) => void;
  onUpdate: (index: number, character: PlayerCharacter) => void;
  onRemove: (index: number) => void;
}) {
  const [isAdding, setIsAdding] = useState(false);
  const [newCharacter, setNewCharacter] = useState<PlayerCharacter>({
    name: '',
    class: '',
    race: '',
    background: '',
    personalityTraits: [],
  });

  const handleAdd = () => {
    if (newCharacter.name && newCharacter.class && newCharacter.race) {
      onAdd(newCharacter);
      setNewCharacter({
        name: '',
        class: '',
        race: '',
        background: '',
        personalityTraits: [],
      });
      setIsAdding(false);
    }
  };

  return (
    <div>
      <div className="space-y-4 mb-4">
        {characters.map((character, index) => (
          <div key={index} className="bg-gray-800 p-4 rounded-lg">
            <div className="flex justify-between items-start">
              <div>
                <h4 className="font-semibold">{character.name}</h4>
                <p className="text-sm text-gray-400">
                  {character.race} {character.class}
                  {character.background && ` • ${character.background}`}
                </p>
              </div>
              <button
                onClick={() => onRemove(index)}
                className="text-red-500 hover:text-red-400"
              >
                <Trash2 className="w-4 h-4" />
              </button>
            </div>
          </div>
        ))}
      </div>

      {isAdding ? (
        <div className="bg-gray-800 p-4 rounded-lg space-y-4">
          <div className="grid grid-cols-2 gap-4">
            <div>
              <label className="block text-sm font-medium mb-1">
                Character Name <span className="text-red-500">*</span>
              </label>
              <input
                type="text"
                value={newCharacter.name}
                onChange={(e) =>
                  setNewCharacter({ ...newCharacter, name: e.target.value })
                }
                className="input w-full"
                placeholder="e.g., Aragorn"
              />
            </div>
            <div>
              <label className="block text-sm font-medium mb-1">
                Background
              </label>
              <input
                type="text"
                value={newCharacter.background || ''}
                onChange={(e) =>
                  setNewCharacter({ ...newCharacter, background: e.target.value })
                }
                className="input w-full"
                placeholder="e.g., Folk Hero"
              />
            </div>
          </div>

          <div className="grid grid-cols-2 gap-4">
            <div>
              <label className="block text-sm font-medium mb-1">
                Class <span className="text-red-500">*</span>
              </label>
              <select
                value={newCharacter.class}
                onChange={(e) =>
                  setNewCharacter({ ...newCharacter, class: e.target.value })
                }
                className="input w-full"
              >
                <option value="">Select a class</option>
                {CHARACTER_CLASSES.map((cls) => (
                  <option key={cls} value={cls}>
                    {cls}
                  </option>
                ))}
              </select>
            </div>
            <div>
              <label className="block text-sm font-medium mb-1">
                Race <span className="text-red-500">*</span>
              </label>
              <select
                value={newCharacter.race}
                onChange={(e) =>
                  setNewCharacter({ ...newCharacter, race: e.target.value })
                }
                className="input w-full"
              >
                <option value="">Select a race</option>
                {CHARACTER_RACES.map((race) => (
                  <option key={race} value={race}>
                    {race}
                  </option>
                ))}
              </select>
            </div>
          </div>

          <div className="flex gap-2">
            <button onClick={handleAdd} className="btn-primary">
              Add Character
            </button>
            <button
              onClick={() => {
                setIsAdding(false);
                setNewCharacter({
                  name: '',
                  class: '',
                  race: '',
                  background: '',
                  personalityTraits: [],
                });
              }}
              className="btn-secondary"
            >
              Cancel
            </button>
          </div>
        </div>
      ) : (
        <button
          onClick={() => setIsAdding(true)}
          className="btn-secondary w-full flex items-center justify-center gap-2"
        >
          <Plus className="w-4 h-4" />
          Add Player Character
        </button>
      )}

      {characters.length === 0 && (
        <p className="text-sm text-yellow-500 mt-4">
          Add at least one player character to continue
        </p>
      )}
    </div>
  );
}

// Step 2: World & Themes
function StepWorldThemes({
  setting,
  themes,
  setSetting,
  setThemes,
}: {
  setting: string;
  themes: string[];
  setSetting: (setting: string) => void;
  setThemes: (themes: string[]) => void;
}) {
  const toggleTheme = (theme: string) => {
    if (themes.includes(theme)) {
      setThemes(themes.filter((t) => t !== theme));
    } else {
      setThemes([...themes, theme]);
    }
  };

  return (
    <div className="space-y-6">
      <div>
        <label htmlFor="setting" className="block text-sm font-medium mb-2">
          Setting Description <span className="text-red-500">*</span>
        </label>
        <textarea
          id="setting"
          value={setting}
          onChange={(e) => setSetting(e.target.value)}
          placeholder="Describe the world your campaign takes place in..."
          className="input w-full h-32 resize-none"
        />
        <p className="text-sm text-gray-400 mt-2">
          Provide details about the world, time period, and atmosphere
        </p>
      </div>

      <div>
        <label className="block text-sm font-medium mb-2">
          Campaign Themes <span className="text-red-500">*</span>
        </label>
        <div className="grid grid-cols-3 gap-2">
          {CAMPAIGN_THEMES.map((theme) => (
            <button
              key={theme}
              onClick={() => toggleTheme(theme.toLowerCase().replace(' ', '_'))}
              className={clsx(
                'p-2 rounded text-sm transition-colors',
                themes.includes(theme.toLowerCase().replace(' ', '_'))
                  ? 'bg-dnd-purple text-white'
                  : 'bg-gray-800 text-gray-300 hover:bg-gray-700'
              )}
            >
              {theme}
            </button>
          ))}
        </div>
        {themes.length === 0 && (
          <p className="text-sm text-yellow-500 mt-2">
            Select at least one theme for your campaign
          </p>
        )}
      </div>
    </div>
  );
}

// Step 3: Review & Generate
function StepReview({
  name,
  setting,
  themes,
  playerCharacters,
}: {
  name: string;
  setting: string;
  themes: string[];
  playerCharacters: PlayerCharacter[];
}) {
  return (
    <div className="space-y-6">
      <div>
        <h3 className="text-lg font-semibold mb-2">Campaign Summary</h3>
        <div className="space-y-4">
          <div>
            <span className="text-gray-400">Name:</span>
            <p className="font-medium">{name}</p>
          </div>

          <div>
            <span className="text-gray-400">Setting:</span>
            <p className="mt-1">{setting}</p>
          </div>

          <div>
            <span className="text-gray-400">Themes:</span>
            <div className="flex flex-wrap gap-2 mt-1">
              {themes.map((theme) => (
                <span
                  key={theme}
                  className="px-3 py-1 bg-dnd-purple/20 text-dnd-purple rounded-full text-sm"
                >
                  {theme.replace('_', ' ')}
                </span>
              ))}
            </div>
          </div>

          <div>
            <span className="text-gray-400">Player Characters:</span>
            <div className="mt-2 space-y-2">
              {playerCharacters.map((character, index) => (
                <div key={index} className="bg-gray-800 p-3 rounded">
                  <p className="font-medium">{character.name}</p>
                  <p className="text-sm text-gray-400">
                    {character.race} {character.class}
                    {character.background && ` • ${character.background}`}
                  </p>
                </div>
              ))}
            </div>
          </div>
        </div>
      </div>

      <div className="bg-dnd-purple/10 border border-dnd-purple/30 p-4 rounded-lg">
        <p className="text-sm text-center">
          Ready to generate your campaign? Click the button below to create a unique
          world filled with NPCs, locations, and quest hooks tailored to your
          specifications.
        </p>
      </div>
    </div>
  );
}