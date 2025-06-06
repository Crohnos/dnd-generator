import { Users, Shield, BookOpen, Languages } from 'lucide-react';
import { useState } from 'react';

interface CharacterBuildingCardProps {
  races: any[];
  classes: any[];
  backgrounds: any[];
  languages: any[];
}

export function CharacterBuildingCard({ 
  races, 
  classes, 
  backgrounds, 
  languages 
}: CharacterBuildingCardProps) {
  const [activeSection, setActiveSection] = useState<string>('overview');

  const sections = [
    { id: 'overview', label: 'Overview', icon: Users },
    { id: 'races', label: 'Races', icon: Users, count: races.length },
    { id: 'classes', label: 'Classes', icon: Shield, count: classes.length },
    { id: 'backgrounds', label: 'Backgrounds', icon: BookOpen, count: backgrounds.length },
    { id: 'languages', label: 'Languages', icon: Languages, count: languages.length },
  ];

  return (
    <div className="space-y-6">
      {/* Section Navigation */}
      <div className="flex flex-wrap gap-2">
        {sections.map((section) => {
          const Icon = section.icon;
          const isActive = activeSection === section.id;
          
          return (
            <button
              key={section.id}
              onClick={() => setActiveSection(section.id)}
              className={`flex items-center space-x-2 px-4 py-2 rounded-lg font-medium text-sm transition-colors ${
                isActive
                  ? 'bg-dnd-purple text-white'
                  : 'bg-gray-800 text-gray-300 hover:bg-gray-700'
              }`}
            >
              <Icon className="w-4 h-4" />
              <span>{section.label}</span>
              {section.count !== undefined && section.count > 0 && (
                <span className="bg-gray-700 text-gray-300 text-xs px-2 py-1 rounded-full">
                  {section.count}
                </span>
              )}
            </button>
          );
        })}
      </div>

      {/* Content */}
      <div className="min-h-96">
        {activeSection === 'overview' && (
          <div className="grid gap-6 md:grid-cols-2 lg:grid-cols-4">
            {/* Quick Stats */}
            <div className="card">
              <h3 className="text-lg font-bold text-white mb-4">Character Options</h3>
              <div className="space-y-3 text-sm">
                <div className="flex justify-between">
                  <span className="text-gray-400">Races:</span>
                  <span className="text-white">{races.length}</span>
                </div>
                <div className="flex justify-between">
                  <span className="text-gray-400">Classes:</span>
                  <span className="text-white">{classes.length}</span>
                </div>
                <div className="flex justify-between">
                  <span className="text-gray-400">Backgrounds:</span>
                  <span className="text-white">{backgrounds.length}</span>
                </div>
                <div className="flex justify-between">
                  <span className="text-gray-400">Languages:</span>
                  <span className="text-white">{languages.length}</span>
                </div>
              </div>
            </div>

            {/* Popular Races */}
            {races.length > 0 && (
              <div className="card">
                <h3 className="text-lg font-bold text-white mb-4">Featured Races</h3>
                <div className="space-y-2">
                  {races.slice(0, 4).map((race) => (
                    <div key={race.id} className="flex items-center justify-between">
                      <span className="text-gray-300 text-sm">{race.name}</span>
                      <span className="text-gray-400 text-xs">{race.size_category}</span>
                    </div>
                  ))}
                  {races.length > 4 && (
                    <button
                      onClick={() => setActiveSection('races')}
                      className="text-dnd-purple hover:text-purple-300 text-sm"
                    >
                      View all {races.length} races →
                    </button>
                  )}
                </div>
              </div>
            )}

            {/* Popular Classes */}
            {classes.length > 0 && (
              <div className="card">
                <h3 className="text-lg font-bold text-white mb-4">Featured Classes</h3>
                <div className="space-y-2">
                  {classes.slice(0, 4).map((characterClass) => (
                    <div key={characterClass.id} className="flex items-center justify-between">
                      <span className="text-gray-300 text-sm">{characterClass.name}</span>
                      <span className="text-gray-400 text-xs">d{characterClass.hit_die}</span>
                    </div>
                  ))}
                  {classes.length > 4 && (
                    <button
                      onClick={() => setActiveSection('classes')}
                      className="text-dnd-purple hover:text-purple-300 text-sm"
                    >
                      View all {classes.length} classes →
                    </button>
                  )}
                </div>
              </div>
            )}

            {/* Language Overview */}
            {languages.length > 0 && (
              <div className="card">
                <h3 className="text-lg font-bold text-white mb-4">Language Overview</h3>
                <div className="space-y-2">
                  {languages.slice(0, 4).map((language) => (
                    <div key={language.id} className="flex items-center justify-between">
                      <span className="text-gray-300 text-sm">{language.name}</span>
                      <span className="text-gray-400 text-xs">{language.language_type}</span>
                    </div>
                  ))}
                  {languages.length > 4 && (
                    <button
                      onClick={() => setActiveSection('languages')}
                      className="text-dnd-purple hover:text-purple-300 text-sm"
                    >
                      View all {languages.length} languages →
                    </button>
                  )}
                </div>
              </div>
            )}
          </div>
        )}

        {activeSection === 'races' && (
          <div>
            <h3 className="text-xl font-bold text-white mb-4">Playable Races</h3>
            <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
              {races.map((race) => (
                <div key={race.id} className="card">
                  <div className="flex items-start justify-between mb-3">
                    <h4 className="font-bold text-white">{race.name}</h4>
                    <span className="px-2 py-1 bg-blue-500 bg-opacity-20 text-blue-400 text-xs rounded-full">
                      {race.size_category}
                    </span>
                  </div>
                  <p className="text-gray-300 text-sm mb-3">{race.description}</p>
                  
                  <div className="space-y-2 text-xs mb-3">
                    <div className="grid grid-cols-2 gap-2">
                      <div>
                        <span className="text-gray-400">Speed:</span>
                        <span className="text-white ml-1">{race.speed} ft</span>
                      </div>
                      {race.lifespan_years && (
                        <div>
                          <span className="text-gray-400">Lifespan:</span>
                          <span className="text-white ml-1">{race.lifespan_years} years</span>
                        </div>
                      )}
                    </div>
                  </div>

                  {race.languages && race.languages.length > 0 && (
                    <div className="mb-3">
                      <span className="text-gray-400 text-xs">Languages:</span>
                      <div className="flex flex-wrap gap-1 mt-1">
                        {race.languages.map((language: string, index: number) => (
                          <span
                            key={index}
                            className="px-2 py-1 bg-gray-700 text-gray-300 text-xs rounded"
                          >
                            {language}
                          </span>
                        ))}
                      </div>
                    </div>
                  )}

                  {race.cultural_notes && (
                    <div className="text-xs">
                      <span className="text-gray-400">Culture:</span>
                      <p className="text-gray-300 mt-1">{race.cultural_notes}</p>
                    </div>
                  )}
                </div>
              ))}
            </div>
          </div>
        )}

        {activeSection === 'classes' && (
          <div>
            <h3 className="text-xl font-bold text-white mb-4">Character Classes</h3>
            <div className="grid gap-4 md:grid-cols-2">
              {classes.map((characterClass) => (
                <div key={characterClass.id} className="card">
                  <div className="flex items-start justify-between mb-3">
                    <h4 className="font-bold text-white">{characterClass.name}</h4>
                    <span className="px-2 py-1 bg-red-500 bg-opacity-20 text-red-400 text-xs rounded-full">
                      d{characterClass.hit_die}
                    </span>
                  </div>
                  <p className="text-gray-300 text-sm mb-3">{characterClass.description}</p>
                  
                  <div className="space-y-2 text-xs mb-3">
                    {characterClass.primary_ability && characterClass.primary_ability.length > 0 && (
                      <div>
                        <span className="text-gray-400">Primary Abilities:</span>
                        <span className="text-white ml-1">{characterClass.primary_ability.join(', ')}</span>
                      </div>
                    )}
                    {characterClass.saving_throw_proficiencies && characterClass.saving_throw_proficiencies.length > 0 && (
                      <div>
                        <span className="text-gray-400">Saving Throws:</span>
                        <span className="text-white ml-1">{characterClass.saving_throw_proficiencies.join(', ')}</span>
                      </div>
                    )}
                    {characterClass.spellcasting_ability && (
                      <div>
                        <span className="text-gray-400">Spellcasting:</span>
                        <span className="text-white ml-1">{characterClass.spellcasting_ability}</span>
                      </div>
                    )}
                  </div>

                  {characterClass.equipment_proficiencies && characterClass.equipment_proficiencies.length > 0 && (
                    <div className="mb-3">
                      <span className="text-gray-400 text-xs">Equipment Proficiencies:</span>
                      <div className="flex flex-wrap gap-1 mt-1">
                        {characterClass.equipment_proficiencies.slice(0, 4).map((prof: string, index: number) => (
                          <span
                            key={index}
                            className="px-2 py-1 bg-gray-700 text-gray-300 text-xs rounded"
                          >
                            {prof}
                          </span>
                        ))}
                        {characterClass.equipment_proficiencies.length > 4 && (
                          <span className="px-2 py-1 bg-gray-600 text-gray-400 text-xs rounded">
                            +{characterClass.equipment_proficiencies.length - 4} more
                          </span>
                        )}
                      </div>
                    </div>
                  )}

                  {characterClass.role_description && (
                    <div className="text-xs">
                      <span className="text-gray-400">Role:</span>
                      <p className="text-gray-300 mt-1">{characterClass.role_description}</p>
                    </div>
                  )}
                </div>
              ))}
            </div>
          </div>
        )}

        {activeSection === 'backgrounds' && (
          <div>
            <h3 className="text-xl font-bold text-white mb-4">Character Backgrounds</h3>
            <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
              {backgrounds.map((background) => (
                <div key={background.id} className="card">
                  <h4 className="font-bold text-white mb-3">{background.name}</h4>
                  <p className="text-gray-300 text-sm mb-3">{background.description}</p>
                  
                  <div className="space-y-2 text-xs mb-3">
                    {background.skill_proficiencies && background.skill_proficiencies.length > 0 && (
                      <div>
                        <span className="text-gray-400">Skills:</span>
                        <span className="text-white ml-1">{background.skill_proficiencies.join(', ')}</span>
                      </div>
                    )}
                    {background.tool_proficiencies && background.tool_proficiencies.length > 0 && (
                      <div>
                        <span className="text-gray-400">Tools:</span>
                        <span className="text-white ml-1">{background.tool_proficiencies.join(', ')}</span>
                      </div>
                    )}
                  </div>

                  {background.feature_name && (
                    <div className="mb-3 p-2 bg-purple-600 bg-opacity-20 rounded">
                      <h5 className="text-purple-300 text-xs font-semibold">{background.feature_name}</h5>
                      {background.feature_description && (
                        <p className="text-gray-300 text-xs mt-1">{background.feature_description}</p>
                      )}
                    </div>
                  )}

                  {background.variants && background.variants.length > 0 && (
                    <div className="text-xs">
                      <span className="text-gray-400">Variants:</span>
                      <div className="flex flex-wrap gap-1 mt-1">
                        {background.variants.map((variant: string, index: number) => (
                          <span
                            key={index}
                            className="px-2 py-1 bg-gray-700 text-gray-300 text-xs rounded"
                          >
                            {variant}
                          </span>
                        ))}
                      </div>
                    </div>
                  )}
                </div>
              ))}
            </div>
          </div>
        )}

        {activeSection === 'languages' && (
          <div>
            <h3 className="text-xl font-bold text-white mb-4">Languages</h3>
            <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
              {languages.map((language) => (
                <div key={language.id} className="card">
                  <div className="flex items-start justify-between mb-3">
                    <h4 className="font-bold text-white">{language.name}</h4>
                    <span className="px-2 py-1 bg-green-500 bg-opacity-20 text-green-400 text-xs rounded-full">
                      {language.language_type}
                    </span>
                  </div>
                  <p className="text-gray-300 text-sm mb-3">{language.description}</p>
                  
                  <div className="space-y-2 text-xs mb-3">
                    {language.script && (
                      <div>
                        <span className="text-gray-400">Script:</span>
                        <span className="text-white ml-1">{language.script}</span>
                      </div>
                    )}
                    {language.complexity && (
                      <div>
                        <span className="text-gray-400">Complexity:</span>
                        <span className="text-white ml-1">{language.complexity}</span>
                      </div>
                    )}
                    {language.writing_system && (
                      <div>
                        <span className="text-gray-400">Writing:</span>
                        <span className="text-white ml-1">{language.writing_system}</span>
                      </div>
                    )}
                  </div>

                  {language.speakers && language.speakers.length > 0 && (
                    <div className="mb-3">
                      <span className="text-gray-400 text-xs">Typical Speakers:</span>
                      <div className="flex flex-wrap gap-1 mt-1">
                        {language.speakers.map((speaker: string, index: number) => (
                          <span
                            key={index}
                            className="px-2 py-1 bg-blue-600 bg-opacity-30 text-blue-300 text-xs rounded"
                          >
                            {speaker}
                          </span>
                        ))}
                      </div>
                    </div>
                  )}

                  {language.regions && language.regions.length > 0 && (
                    <div className="text-xs">
                      <span className="text-gray-400">Regions:</span>
                      <span className="text-gray-300 ml-1">
                        {language.regions.slice(0, 3).join(', ')}
                        {language.regions.length > 3 && ` +${language.regions.length - 3} more`}
                      </span>
                    </div>
                  )}
                </div>
              ))}
            </div>
          </div>
        )}

        {/* Empty States */}
        {((activeSection === 'races' && races.length === 0) ||
          (activeSection === 'classes' && classes.length === 0) ||
          (activeSection === 'backgrounds' && backgrounds.length === 0) ||
          (activeSection === 'languages' && languages.length === 0)) && (
          <div className="text-center py-12">
            <Users className="w-16 h-16 text-gray-500 mx-auto mb-4" />
            <h3 className="text-xl font-bold text-white mb-2">No Data Available</h3>
            <p className="text-gray-400">
              Character building data will appear once the campaign generation is complete.
            </p>
          </div>
        )}
      </div>
    </div>
  );
}