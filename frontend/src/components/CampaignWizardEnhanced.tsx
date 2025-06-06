'use client';

import { useState } from 'react';
import { useRouter } from 'next/navigation';
import { useCampaignStore, PlayerCharacter } from '@/stores/campaignStore';
import { ChevronLeft, ChevronRight, Plus, Trash2, Wand2, Info } from 'lucide-react';
import clsx from 'clsx';
import { TagInput } from './TagInput';

const WIZARD_STEPS = [
  { id: 0, name: 'Basic Information', description: 'Name your campaign' },
  { id: 1, name: 'Player Characters', description: 'Add your players' },
  { id: 2, name: 'World & Themes', description: 'Set the stage' },
  { id: 3, name: 'Character Details', description: 'Expand character backgrounds' },
  { id: 4, name: 'Campaign Settings', description: 'Configure gameplay options' },
  { id: 5, name: 'World Building', description: 'Define your world' },
  { id: 6, name: 'Review & Generate', description: 'Create your campaign' },
];

const CHARACTER_CLASSES = [
  'Fighter', 'Wizard', 'Rogue', 'Cleric', 'Ranger', 'Paladin', 
  'Barbarian', 'Sorcerer', 'Warlock', 'Druid', 'Monk', 'Bard',
  'Artificer', 'Blood Hunter'
];

const CHARACTER_RACES = [
  'Human', 'Elf', 'Dwarf', 'Halfling', 'Dragonborn', 'Gnome', 
  'Half-Elf', 'Half-Orc', 'Tiefling', 'Aasimar', 'Genasi', 
  'Goliath', 'Tabaxi', 'Firbolg', 'Kenku', 'Lizardfolk'
];

const CHARACTER_BACKGROUNDS = [
  'Acolyte', 'Criminal', 'Folk Hero', 'Noble', 'Sage', 'Soldier',
  'Charlatan', 'Entertainer', 'Guild Artisan', 'Hermit', 'Outlander',
  'Sailor', 'Urchin', 'Far Traveler', 'Haunted One', 'Investigator'
];

const CAMPAIGN_THEMES = [
  'Political Intrigue', 'War', 'Mystery', 'Exploration', 'Horror', 
  'Comedy', 'Romance', 'Apocalyptic', 'Urban', 'Wilderness', 
  'Underdark', 'Planar Travel', 'Pirates', 'Steampunk', 'Gothic',
  'High Fantasy', 'Low Fantasy', 'Dark Fantasy', 'Epic Fantasy'
];

export function CampaignWizardEnhanced() {
  const router = useRouter();
  const [isGenerating, setIsGenerating] = useState(false);
  const [error, setError] = useState<string | null>(null);
  
  // Enhanced world building state
  const [worldBuilding, setWorldBuilding] = useState({
    calendarName: '',
    monthsPerYear: 12,
    daysPerMonth: 30,
    pantheonName: '',
    deities: [] as string[],
    continents: [] as string[],
    majorCities: [] as string[],
    currencyTypes: { copper: 'Copper', silver: 'Silver', gold: 'Gold', platinum: 'Platinum' },
    governmentType: 'monarchy',
    historicalEvents: [] as string[],
  });
  
  const [campaignSpecifics, setCampaignSpecifics] = useState({
    startingLocation: '',
    initialQuestHooks: [] as string[],
    recurringVillains: [] as { name: string; type: string; goals: string }[],
    majorLocations: [] as { name: string; type: string; significance: string }[],
  });
  
  const [generationPreferences, setGenerationPreferences] = useState({
    npcDepth: 'detailed',
    locationDetail: 'comprehensive',
    questComplexity: 'interconnected',
    encounterVariety: 'mixed_combat_social_exploration',
    magicItemFrequency: 'moderate',
    factionInvolvement: 'heavy',
  });
  
  const {
    currentStep,
    name,
    setting,
    themes,
    playerCharacters,
    progressionType,
    tone,
    difficulty,
    startingLevel,
    campaignLength,
    additionalNotes,
    setName,
    setSetting,
    setThemes,
    addPlayerCharacter,
    updatePlayerCharacter,
    removePlayerCharacter,
    setProgressionType,
    setTone,
    setDifficulty,
    setStartingLevel,
    setCampaignLength,
    setAdditionalNotes,
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
      progression_type: progressionType,
      tone,
      difficulty,
      starting_level: startingLevel,
      campaign_length: campaignLength,
      additional_notes: additionalNotes,
      world_building: {
        calendar_system: worldBuilding.calendarName ? {
          name: worldBuilding.calendarName,
          months_per_year: worldBuilding.monthsPerYear,
          days_per_month: worldBuilding.daysPerMonth,
          current_year: 1247,
          special_days: [],
        } : undefined,
        pantheon: worldBuilding.pantheonName ? {
          name: worldBuilding.pantheonName,
          deities: worldBuilding.deities.map(d => ({
            name: d,
            domains: [],
            alignment: 'Neutral',
          })),
        } : undefined,
        geography: {
          continents: worldBuilding.continents,
          major_cities: worldBuilding.majorCities,
          climate_zones: [],
        },
        economic_system: {
          currency: worldBuilding.currencyTypes,
          major_trade_goods: [],
          trade_routes: [],
        },
        political_landscape: {
          government_type: worldBuilding.governmentType,
          ruling_factions: [],
        },
        historical_context: {
          major_events: worldBuilding.historicalEvents.map((e, i) => ({
            name: e,
            years_ago: (i + 1) * 50,
            description: e,
          })),
        },
      },
      campaign_specifics: {
        starting_location: campaignSpecifics.startingLocation,
        initial_quest_hooks: campaignSpecifics.initialQuestHooks,
        recurring_villains: campaignSpecifics.recurringVillains,
        major_locations: campaignSpecifics.majorLocations,
      },
      generation_preferences: generationPreferences,
    };

    console.log('=== ENHANCED CAMPAIGN WIZARD SUBMISSION ===');
    console.log('Campaign Data:', JSON.stringify(campaignData, null, 2));
    console.log('==========================================');

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
          <StepCharacterDetails
            characters={playerCharacters}
            onUpdate={updatePlayerCharacter}
          />
        );
      case 4:
        return (
          <StepCampaignSettings
            progressionType={progressionType}
            tone={tone}
            difficulty={difficulty}
            startingLevel={startingLevel}
            campaignLength={campaignLength}
            additionalNotes={additionalNotes}
            setProgressionType={setProgressionType}
            setTone={setTone}
            setDifficulty={setDifficulty}
            setStartingLevel={setStartingLevel}
            setCampaignLength={setCampaignLength}
            setAdditionalNotes={setAdditionalNotes}
          />
        );
      case 5:
        return (
          <StepWorldBuilding
            worldBuilding={worldBuilding}
            setWorldBuilding={setWorldBuilding}
            campaignSpecifics={campaignSpecifics}
            setCampaignSpecifics={setCampaignSpecifics}
          />
        );
      case 6:
        return (
          <StepReviewEnhanced
            name={name}
            setting={setting}
            themes={themes}
            playerCharacters={playerCharacters}
            progressionType={progressionType}
            tone={tone}
            difficulty={difficulty}
            startingLevel={startingLevel}
            campaignLength={campaignLength}
            additionalNotes={additionalNotes}
            worldBuilding={worldBuilding}
            campaignSpecifics={campaignSpecifics}
            generationPreferences={generationPreferences}
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
                <span className="text-xs mt-1 text-gray-400 text-center">{step.name}</span>
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

        {currentStep < 6 ? (
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

// Step 0: Basic Information (unchanged)
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

// Step 1: Player Characters (unchanged)
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
              <select
                value={newCharacter.background || ''}
                onChange={(e) =>
                  setNewCharacter({ ...newCharacter, background: e.target.value })
                }
                className="input w-full"
              >
                <option value="">Select a background</option>
                {CHARACTER_BACKGROUNDS.map((bg) => (
                  <option key={bg} value={bg}>
                    {bg}
                  </option>
                ))}
              </select>
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

// Step 2: World & Themes (unchanged)
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

// Step 3: Character Details (NEW)
function StepCharacterDetails({
  characters,
  onUpdate,
}: {
  characters: PlayerCharacter[];
  onUpdate: (index: number, character: PlayerCharacter) => void;
}) {
  const [selectedCharacter, setSelectedCharacter] = useState<number | null>(null);

  if (characters.length === 0) {
    return (
      <div className="text-center text-gray-400">
        <Info className="w-12 h-12 mx-auto mb-4 opacity-50" />
        <p>No characters to expand. You can skip this step or go back to add characters.</p>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      <div>
        <h3 className="text-lg font-semibold mb-4">Expand Character Backgrounds</h3>
        <p className="text-gray-400 mb-6">
          Add more details to help the AI create richer, more connected NPCs and storylines.
        </p>
      </div>

      <div className="space-y-4">
        {characters.map((character, index) => (
          <div key={index} className="bg-gray-800 p-4 rounded-lg">
            <div className="flex justify-between items-center mb-4">
              <h4 className="font-semibold text-lg">{character.name}</h4>
              <button
                onClick={() => setSelectedCharacter(selectedCharacter === index ? null : index)}
                className="btn-secondary text-sm"
              >
                {selectedCharacter === index ? 'Collapse' : 'Expand Details'}
              </button>
            </div>

            {selectedCharacter === index && (
              <div className="space-y-4 mt-4">
                <div>
                  <label className="block text-sm font-medium mb-1">
                    Backstory
                  </label>
                  <textarea
                    value={character.backstory || ''}
                    onChange={(e) =>
                      onUpdate(index, { ...character, backstory: e.target.value })
                    }
                    placeholder="Describe their background, origins, and important life events..."
                    className="input w-full h-24 resize-none"
                  />
                </div>

                <div>
                  <label className="block text-sm font-medium mb-1">
                    Connections to Other Characters/NPCs
                  </label>
                  <textarea
                    value={character.connections || ''}
                    onChange={(e) =>
                      onUpdate(index, { ...character, connections: e.target.value })
                    }
                    placeholder="Family, friends, mentors, enemies, organizations they belong to..."
                    className="input w-full h-20 resize-none"
                  />
                </div>

                <div className="grid grid-cols-2 gap-4">
                  <div>
                    <label className="block text-sm font-medium mb-1">
                      Ideals
                    </label>
                    <TagInput
                      tags={character.ideals || []}
                      onChange={(ideals) => onUpdate(index, { ...character, ideals })}
                      placeholder="Justice, Freedom, Knowledge..."
                      maxTags={5}
                    />
                  </div>
                  <div>
                    <label className="block text-sm font-medium mb-1">
                      Bonds
                    </label>
                    <TagInput
                      tags={character.bonds || []}
                      onChange={(bonds) => onUpdate(index, { ...character, bonds })}
                      placeholder="Family, Hometown, Mentor..."
                      maxTags={5}
                    />
                  </div>
                </div>

                <div>
                  <label className="block text-sm font-medium mb-1">
                    Flaws
                  </label>
                  <TagInput
                    tags={character.flaws || []}
                    onChange={(flaws) => onUpdate(index, { ...character, flaws })}
                    placeholder="Prideful, Reckless, Greedy..."
                    maxTags={5}
                  />
                </div>
              </div>
            )}
          </div>
        ))}
      </div>
    </div>
  );
}

// Step 4: Campaign Settings (NEW)
function StepCampaignSettings({
  progressionType,
  tone,
  difficulty,
  startingLevel,
  campaignLength,
  additionalNotes,
  setProgressionType,
  setTone,
  setDifficulty,
  setStartingLevel,
  setCampaignLength,
  setAdditionalNotes,
}: {
  progressionType: string;
  tone: string;
  difficulty: string;
  startingLevel: number;
  campaignLength: string;
  additionalNotes: string;
  setProgressionType: (type: string) => void;
  setTone: (tone: string) => void;
  setDifficulty: (difficulty: string) => void;
  setStartingLevel: (level: number) => void;
  setCampaignLength: (length: string) => void;
  setAdditionalNotes: (notes: string) => void;
}) {
  return (
    <div className="space-y-6">
      <div>
        <h3 className="text-lg font-semibold mb-4">Campaign Configuration</h3>
        <p className="text-gray-400 mb-6">
          Configure how your campaign will be structured and played.
        </p>
      </div>

      <div className="grid grid-cols-2 gap-6">
        <div>
          <label className="block text-sm font-medium mb-2">
            Progression Type
          </label>
          <select
            value={progressionType}
            onChange={(e) => setProgressionType(e.target.value)}
            className="input w-full"
          >
            <option value="milestone">Milestone-Based</option>
            <option value="experience">Experience Points</option>
            <option value="hybrid">Hybrid (Both)</option>
          </select>
          <p className="text-xs text-gray-400 mt-1">
            How characters will level up
          </p>
        </div>

        <div>
          <label className="block text-sm font-medium mb-2">
            Campaign Tone
          </label>
          <select
            value={tone}
            onChange={(e) => setTone(e.target.value)}
            className="input w-full"
          >
            <option value="serious">Serious & Dramatic</option>
            <option value="balanced">Balanced</option>
            <option value="lighthearted">Lighthearted & Fun</option>
            <option value="dark">Dark & Gritty</option>
            <option value="heroic">Heroic & Epic</option>
          </select>
        </div>

        <div>
          <label className="block text-sm font-medium mb-2">
            Difficulty Level
          </label>
          <select
            value={difficulty}
            onChange={(e) => setDifficulty(e.target.value)}
            className="input w-full"
          >
            <option value="easy">Easy (Casual Play)</option>
            <option value="medium">Medium (Standard)</option>
            <option value="hard">Hard (Challenging)</option>
            <option value="deadly">Deadly (Expert)</option>
          </select>
        </div>

        <div>
          <label className="block text-sm font-medium mb-2">
            Starting Level
          </label>
          <select
            value={startingLevel}
            onChange={(e) => setStartingLevel(parseInt(e.target.value))}
            className="input w-full"
          >
            {Array.from({ length: 20 }, (_, i) => i + 1).map(level => (
              <option key={level} value={level}>Level {level}</option>
            ))}
          </select>
        </div>
      </div>

      <div>
        <label className="block text-sm font-medium mb-2">
          Expected Campaign Length
        </label>
        <div className="grid grid-cols-4 gap-2">
          {[
            { value: 'short', label: 'Short (1-5 sessions)' },
            { value: 'medium', label: 'Medium (6-15 sessions)' },
            { value: 'long', label: 'Long (16-30 sessions)' },
            { value: 'epic', label: 'Epic (30+ sessions)' },
          ].map(option => (
            <button
              key={option.value}
              onClick={() => setCampaignLength(option.value)}
              className={clsx(
                'p-3 rounded text-sm transition-colors',
                campaignLength === option.value
                  ? 'bg-dnd-purple text-white'
                  : 'bg-gray-800 text-gray-300 hover:bg-gray-700'
              )}
            >
              {option.label}
            </button>
          ))}
        </div>
      </div>

      <div>
        <label className="block text-sm font-medium mb-2">
          Additional Notes
        </label>
        <textarea
          value={additionalNotes}
          onChange={(e) => setAdditionalNotes(e.target.value)}
          placeholder="Any specific requests, house rules, or important details for the AI to consider..."
          className="input w-full h-24 resize-none"
        />
        <p className="text-xs text-gray-400 mt-1">
          Optional: Provide any additional context to help create the perfect campaign
        </p>
      </div>
    </div>
  );
}

// Step 5: World Building (NEW)
function StepWorldBuilding({
  worldBuilding,
  setWorldBuilding,
  campaignSpecifics,
  setCampaignSpecifics,
}: {
  worldBuilding: any;
  setWorldBuilding: (value: any) => void;
  campaignSpecifics: any;
  setCampaignSpecifics: (value: any) => void;
}) {
  const [newDeity, setNewDeity] = useState('');
  const [newContinent, setNewContinent] = useState('');
  const [newCity, setNewCity] = useState('');
  const [newEvent, setNewEvent] = useState('');
  const [newQuestHook, setNewQuestHook] = useState('');
  const [newVillain, setNewVillain] = useState({ name: '', type: '', goals: '' });
  const [newLocation, setNewLocation] = useState({ name: '', type: '', significance: '' });

  return (
    <div className="space-y-6">
      <div>
        <h3 className="text-lg font-semibold mb-4">World Building (Optional)</h3>
        <p className="text-gray-400 mb-6">
          Add details about your world to create a richer campaign. All fields are optional.
        </p>
      </div>

      {/* Calendar System */}
      <div className="bg-gray-800 p-4 rounded-lg space-y-4">
        <h4 className="font-medium text-dnd-purple">Calendar System</h4>
        <div className="grid grid-cols-3 gap-4">
          <div>
            <label className="block text-sm font-medium mb-1">Calendar Name</label>
            <input
              type="text"
              value={worldBuilding.calendarName}
              onChange={(e) => setWorldBuilding({ ...worldBuilding, calendarName: e.target.value })}
              placeholder="e.g., Eldorian Reckoning"
              className="input w-full"
            />
          </div>
          <div>
            <label className="block text-sm font-medium mb-1">Months per Year</label>
            <input
              type="number"
              value={worldBuilding.monthsPerYear}
              onChange={(e) => setWorldBuilding({ ...worldBuilding, monthsPerYear: parseInt(e.target.value) || 12 })}
              className="input w-full"
            />
          </div>
          <div>
            <label className="block text-sm font-medium mb-1">Days per Month</label>
            <input
              type="number"
              value={worldBuilding.daysPerMonth}
              onChange={(e) => setWorldBuilding({ ...worldBuilding, daysPerMonth: parseInt(e.target.value) || 30 })}
              className="input w-full"
            />
          </div>
        </div>
      </div>

      {/* Pantheon */}
      <div className="bg-gray-800 p-4 rounded-lg space-y-4">
        <h4 className="font-medium text-dnd-purple">Pantheon & Deities</h4>
        <div>
          <label className="block text-sm font-medium mb-1">Pantheon Name</label>
          <input
            type="text"
            value={worldBuilding.pantheonName}
            onChange={(e) => setWorldBuilding({ ...worldBuilding, pantheonName: e.target.value })}
            placeholder="e.g., The Celestial Court"
            className="input w-full"
          />
        </div>
        <div>
          <label className="block text-sm font-medium mb-1">Deities</label>
          <div className="flex gap-2 mb-2">
            <input
              type="text"
              value={newDeity}
              onChange={(e) => setNewDeity(e.target.value)}
              placeholder="Add a deity name"
              className="input flex-1"
              onKeyPress={(e) => {
                if (e.key === 'Enter' && newDeity) {
                  setWorldBuilding({ 
                    ...worldBuilding, 
                    deities: [...worldBuilding.deities, newDeity] 
                  });
                  setNewDeity('');
                }
              }}
            />
            <button
              onClick={() => {
                if (newDeity) {
                  setWorldBuilding({ 
                    ...worldBuilding, 
                    deities: [...worldBuilding.deities, newDeity] 
                  });
                  setNewDeity('');
                }
              }}
              className="btn-secondary"
            >
              Add
            </button>
          </div>
          <div className="flex flex-wrap gap-2">
            {worldBuilding.deities.map((deity: string, index: number) => (
              <span key={index} className="px-2 py-1 bg-gray-700 rounded text-sm flex items-center gap-1">
                {deity}
                <button
                  onClick={() => setWorldBuilding({
                    ...worldBuilding,
                    deities: worldBuilding.deities.filter((_: any, i: number) => i !== index)
                  })}
                  className="text-red-400 hover:text-red-300"
                >
                  ×
                </button>
              </span>
            ))}
          </div>
        </div>
      </div>

      {/* Geography */}
      <div className="bg-gray-800 p-4 rounded-lg space-y-4">
        <h4 className="font-medium text-dnd-purple">Geography</h4>
        <div className="grid grid-cols-2 gap-4">
          <div>
            <label className="block text-sm font-medium mb-1">Continents</label>
            <div className="flex gap-2">
              <input
                type="text"
                value={newContinent}
                onChange={(e) => setNewContinent(e.target.value)}
                placeholder="Add continent"
                className="input flex-1"
              />
              <button
                onClick={() => {
                  if (newContinent) {
                    setWorldBuilding({ 
                      ...worldBuilding, 
                      continents: [...worldBuilding.continents, newContinent] 
                    });
                    setNewContinent('');
                  }
                }}
                className="btn-secondary text-sm"
              >
                Add
              </button>
            </div>
            <div className="mt-2 space-y-1">
              {worldBuilding.continents.map((c: string, i: number) => (
                <div key={i} className="text-sm text-gray-300">• {c}</div>
              ))}
            </div>
          </div>
          <div>
            <label className="block text-sm font-medium mb-1">Major Cities</label>
            <div className="flex gap-2">
              <input
                type="text"
                value={newCity}
                onChange={(e) => setNewCity(e.target.value)}
                placeholder="Add city"
                className="input flex-1"
              />
              <button
                onClick={() => {
                  if (newCity) {
                    setWorldBuilding({ 
                      ...worldBuilding, 
                      majorCities: [...worldBuilding.majorCities, newCity] 
                    });
                    setNewCity('');
                  }
                }}
                className="btn-secondary text-sm"
              >
                Add
              </button>
            </div>
            <div className="mt-2 space-y-1">
              {worldBuilding.majorCities.map((c: string, i: number) => (
                <div key={i} className="text-sm text-gray-300">• {c}</div>
              ))}
            </div>
          </div>
        </div>
      </div>

      {/* Campaign Specifics */}
      <div className="bg-gray-800 p-4 rounded-lg space-y-4">
        <h4 className="font-medium text-dnd-purple">Campaign Specifics</h4>
        <div>
          <label className="block text-sm font-medium mb-1">Starting Location</label>
          <input
            type="text"
            value={campaignSpecifics.startingLocation}
            onChange={(e) => setCampaignSpecifics({ ...campaignSpecifics, startingLocation: e.target.value })}
            placeholder="e.g., The town of Millhaven"
            className="input w-full"
          />
        </div>
        <div>
          <label className="block text-sm font-medium mb-1">Initial Quest Hooks</label>
          <div className="flex gap-2">
            <input
              type="text"
              value={newQuestHook}
              onChange={(e) => setNewQuestHook(e.target.value)}
              placeholder="Add a quest hook"
              className="input flex-1"
            />
            <button
              onClick={() => {
                if (newQuestHook) {
                  setCampaignSpecifics({ 
                    ...campaignSpecifics, 
                    initialQuestHooks: [...campaignSpecifics.initialQuestHooks, newQuestHook] 
                  });
                  setNewQuestHook('');
                }
              }}
              className="btn-secondary text-sm"
            >
              Add
            </button>
          </div>
          <div className="mt-2 space-y-1">
            {campaignSpecifics.initialQuestHooks.map((q: string, i: number) => (
              <div key={i} className="text-sm text-gray-300">• {q}</div>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
}

// Step 6: Review & Generate (ENHANCED)
function StepReviewEnhanced({
  name,
  setting,
  themes,
  playerCharacters,
  progressionType,
  tone,
  difficulty,
  startingLevel,
  campaignLength,
  additionalNotes,
  worldBuilding,
  campaignSpecifics,
  generationPreferences,
}: {
  name: string;
  setting: string;
  themes: string[];
  playerCharacters: PlayerCharacter[];
  progressionType: string;
  tone: string;
  difficulty: string;
  startingLevel: number;
  campaignLength: string;
  additionalNotes: string;
  worldBuilding: any;
  campaignSpecifics: any;
  generationPreferences: any;
}) {
  return (
    <div className="space-y-6">
      <div>
        <h3 className="text-lg font-semibold mb-4">Campaign Summary</h3>
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
          <div className="space-y-4">
            <div>
              <span className="text-gray-400 text-sm">Campaign Name:</span>
              <p className="font-medium">{name}</p>
            </div>

            <div>
              <span className="text-gray-400 text-sm">Setting:</span>
              <p className="mt-1 text-sm">{setting}</p>
            </div>

            <div>
              <span className="text-gray-400 text-sm">Themes:</span>
              <div className="flex flex-wrap gap-2 mt-1">
                {themes.map((theme) => (
                  <span
                    key={theme}
                    className="px-2 py-1 bg-dnd-purple/20 text-dnd-purple rounded text-xs"
                  >
                    {theme.replace('_', ' ')}
                  </span>
                ))}
              </div>
            </div>

            <div>
              <span className="text-gray-400 text-sm">Configuration:</span>
              <div className="mt-1 text-sm space-y-1">
                <p>Progression: <span className="text-gray-200">{progressionType}</span></p>
                <p>Tone: <span className="text-gray-200">{tone}</span></p>
                <p>Difficulty: <span className="text-gray-200">{difficulty}</span></p>
                <p>Starting Level: <span className="text-gray-200">{startingLevel}</span></p>
                <p>Length: <span className="text-gray-200">{campaignLength}</span></p>
              </div>
            </div>
          </div>

          <div className="space-y-4">
            <div>
              <span className="text-gray-400 text-sm">Player Characters ({playerCharacters.length}):</span>
              <div className="mt-2 space-y-3">
                {playerCharacters.map((character, index) => (
                  <div key={index} className="bg-gray-800 p-3 rounded">
                    <p className="font-medium text-sm">{character.name}</p>
                    <p className="text-xs text-gray-400">
                      {character.race} {character.class}
                      {character.background && ` • ${character.background}`}
                    </p>
                    {character.backstory && (
                      <p className="text-xs text-gray-500 mt-1 truncate">
                        {character.backstory.substring(0, 100)}...
                      </p>
                    )}
                  </div>
                ))}
              </div>
            </div>

            {additionalNotes && (
              <div>
                <span className="text-gray-400 text-sm">Additional Notes:</span>
                <p className="mt-1 text-sm text-gray-300">{additionalNotes}</p>
              </div>
            )}
          </div>
        </div>
      </div>

      <div className="bg-dnd-purple/10 border border-dnd-purple/30 p-6 rounded-lg">
        <h4 className="font-semibold mb-2">Enhanced Generation Process</h4>
        <p className="text-sm text-gray-300 mb-3">
          Your campaign will be generated in nine comprehensive phases:
        </p>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-2 text-sm">
          <div className="flex items-center gap-2">
            <div className="w-2 h-2 bg-dnd-purple rounded-full"></div>
            <span className="text-gray-300">
              <strong>Phase 1A:</strong> Core World Systems
            </span>
          </div>
          <div className="flex items-center gap-2">
            <div className="w-2 h-2 bg-dnd-purple rounded-full"></div>
            <span className="text-gray-300">
              <strong>Phase 1B:</strong> Character Building
            </span>
          </div>
          <div className="flex items-center gap-2">
            <div className="w-2 h-2 bg-dnd-purple rounded-full"></div>
            <span className="text-gray-300">
              <strong>Phase 1C:</strong> Social Framework
            </span>
          </div>
          <div className="flex items-center gap-2">
            <div className="w-2 h-2 bg-dnd-purple rounded-full"></div>
            <span className="text-gray-300">
              <strong>Phase 2A:</strong> PC Entities
            </span>
          </div>
          <div className="flex items-center gap-2">
            <div className="w-2 h-2 bg-dnd-purple rounded-full"></div>
            <span className="text-gray-300">
              <strong>Phase 2B:</strong> PC Locations
            </span>
          </div>
          <div className="flex items-center gap-2">
            <div className="w-2 h-2 bg-dnd-purple rounded-full"></div>
            <span className="text-gray-300">
              <strong>Phase 2C:</strong> PC Items
            </span>
          </div>
          <div className="flex items-center gap-2">
            <div className="w-2 h-2 bg-dnd-purple rounded-full"></div>
            <span className="text-gray-300">
              <strong>Phase 3A:</strong> Quest Hooks & Encounters
            </span>
          </div>
          <div className="flex items-center gap-2">
            <div className="w-2 h-2 bg-dnd-purple rounded-full"></div>
            <span className="text-gray-300">
              <strong>Phase 3B:</strong> World Population
            </span>
          </div>
          <div className="flex items-center gap-2 md:col-span-2 md:justify-center">
            <div className="w-2 h-2 bg-dnd-purple rounded-full"></div>
            <span className="text-gray-300">
              <strong>Phase 3C:</strong> Final Relationships
            </span>
          </div>
        </div>
        <p className="text-sm text-center mt-4 text-gray-400">
          This comprehensive process creates deeply interconnected campaigns with rich backstories, complex relationships, and immersive world-building.
        </p>
      </div>
    </div>
  );
}