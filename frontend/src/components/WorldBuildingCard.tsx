import { Crown, Globe, Calendar, Coins, Scale, Clock, Mountain } from 'lucide-react';
import { useState } from 'react';

interface WorldBuildingCardProps {
  pantheons: any[];
  deities: any[];
  planes: any[];
  geographyRegions: any[];
  economicSystems: any[];
  legalSystems: any[];
  historicalPeriods: any[];
  calendarSystems: any[];
}

export function WorldBuildingCard({ 
  pantheons, 
  deities, 
  planes, 
  geographyRegions, 
  economicSystems, 
  legalSystems, 
  historicalPeriods, 
  calendarSystems 
}: WorldBuildingCardProps) {
  const [activeSection, setActiveSection] = useState<string>('overview');

  const sections = [
    { id: 'overview', label: 'Overview', icon: Globe },
    { id: 'pantheons', label: 'Pantheons & Deities', icon: Crown, count: pantheons.length + deities.length },
    { id: 'planes', label: 'Planes of Existence', icon: Globe, count: planes.length },
    { id: 'geography', label: 'Geography', icon: Mountain, count: geographyRegions.length },
    { id: 'history', label: 'History', icon: Clock, count: historicalPeriods.length },
    { id: 'economics', label: 'Economics', icon: Coins, count: economicSystems.length },
    { id: 'legal', label: 'Legal Systems', icon: Scale, count: legalSystems.length },
    { id: 'calendar', label: 'Calendar', icon: Calendar, count: calendarSystems.length },
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
          <div className="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
            {/* Quick Stats */}
            <div className="card">
              <h3 className="text-lg font-bold text-white mb-4">World Statistics</h3>
              <div className="space-y-3 text-sm">
                <div className="flex justify-between">
                  <span className="text-gray-400">Pantheons:</span>
                  <span className="text-white">{pantheons.length}</span>
                </div>
                <div className="flex justify-between">
                  <span className="text-gray-400">Deities:</span>
                  <span className="text-white">{deities.length}</span>
                </div>
                <div className="flex justify-between">
                  <span className="text-gray-400">Planes:</span>
                  <span className="text-white">{planes.length}</span>
                </div>
                <div className="flex justify-between">
                  <span className="text-gray-400">Regions:</span>
                  <span className="text-white">{geographyRegions.length}</span>
                </div>
                <div className="flex justify-between">
                  <span className="text-gray-400">Economic Systems:</span>
                  <span className="text-white">{economicSystems.length}</span>
                </div>
                <div className="flex justify-between">
                  <span className="text-gray-400">Legal Systems:</span>
                  <span className="text-white">{legalSystems.length}</span>
                </div>
              </div>
            </div>

            {/* Recent History */}
            {historicalPeriods.length > 0 && (
              <div className="card md:col-span-2">
                <h3 className="text-lg font-bold text-white mb-4">Historical Timeline</h3>
                <div className="space-y-3">
                  {historicalPeriods.slice(0, 3).map((period) => (
                    <div key={period.id} className="border-l-2 border-dnd-purple pl-4">
                      <h4 className="font-semibold text-white">{period.period_name}</h4>
                      <p className="text-gray-400 text-sm">
                        {period.start_year} - {period.end_year}
                      </p>
                      <p className="text-gray-300 text-sm mt-1">{period.description}</p>
                    </div>
                  ))}
                  {historicalPeriods.length > 3 && (
                    <button
                      onClick={() => setActiveSection('history')}
                      className="text-dnd-purple hover:text-purple-300 text-sm"
                    >
                      View all {historicalPeriods.length} periods →
                    </button>
                  )}
                </div>
              </div>
            )}
          </div>
        )}

        {activeSection === 'pantheons' && (
          <div className="space-y-6">
            {/* Pantheons */}
            {pantheons.length > 0 && (
              <div>
                <h3 className="text-xl font-bold text-white mb-4">Pantheons</h3>
                <div className="grid gap-4 md:grid-cols-2">
                  {pantheons.map((pantheon) => (
                    <div key={pantheon.id} className="card">
                      <div className="flex items-start justify-between mb-3">
                        <h4 className="font-bold text-white">{pantheon.name}</h4>
                        <span className="px-2 py-1 bg-yellow-500 bg-opacity-20 text-yellow-400 text-xs rounded-full">
                          {pantheon.pantheon_type}
                        </span>
                      </div>
                      <p className="text-gray-300 text-sm mb-3">{pantheon.description}</p>
                      <div className="grid grid-cols-2 gap-3 text-xs">
                        <div>
                          <span className="text-gray-400">Alignment:</span>
                          <span className="text-white ml-1">{pantheon.dominant_alignment}</span>
                        </div>
                        <div>
                          <span className="text-gray-400">Influence:</span>
                          <span className="text-white ml-1">{pantheon.cultural_influence}</span>
                        </div>
                      </div>
                      {pantheon.primary_worshipers && pantheon.primary_worshipers.length > 0 && (
                        <div className="mt-3">
                          <span className="text-gray-400 text-xs">Worshipers: </span>
                          <span className="text-gray-300 text-xs">
                            {pantheon.primary_worshipers.join(', ')}
                          </span>
                        </div>
                      )}
                    </div>
                  ))}
                </div>
              </div>
            )}

            {/* Deities */}
            {deities.length > 0 && (
              <div>
                <h3 className="text-xl font-bold text-white mb-4">Deities</h3>
                <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
                  {deities.map((deity) => (
                    <div key={deity.id} className="card">
                      <div className="flex items-start justify-between mb-2">
                        <div>
                          <h4 className="font-bold text-white">{deity.name}</h4>
                          {deity.title && (
                            <p className="text-gray-400 text-sm">{deity.title}</p>
                          )}
                        </div>
                        <span className="px-2 py-1 bg-purple-500 bg-opacity-20 text-purple-400 text-xs rounded-full">
                          {deity.divine_rank}
                        </span>
                      </div>
                      <p className="text-gray-300 text-sm mb-3">{deity.description}</p>
                      <div className="space-y-2 text-xs">
                        <div>
                          <span className="text-gray-400">Alignment:</span>
                          <span className="text-white ml-1">{deity.alignment}</span>
                        </div>
                        {deity.domains && deity.domains.length > 0 && (
                          <div>
                            <span className="text-gray-400">Domains:</span>
                            <div className="flex flex-wrap gap-1 mt-1">
                              {deity.domains.map((domain: string, index: number) => (
                                <span
                                  key={index}
                                  className="px-2 py-1 bg-gray-700 text-gray-300 text-xs rounded"
                                >
                                  {domain}
                                </span>
                              ))}
                            </div>
                          </div>
                        )}
                        {deity.symbol && (
                          <div>
                            <span className="text-gray-400">Symbol:</span>
                            <span className="text-white ml-1">{deity.symbol}</span>
                          </div>
                        )}
                      </div>
                    </div>
                  ))}
                </div>
              </div>
            )}
          </div>
        )}

        {activeSection === 'planes' && (
          <div>
            <h3 className="text-xl font-bold text-white mb-4">Planes of Existence</h3>
            <div className="grid gap-4 md:grid-cols-2">
              {planes.map((plane) => (
                <div key={plane.id} className="card">
                  <div className="flex items-start justify-between mb-3">
                    <h4 className="font-bold text-white">{plane.name}</h4>
                    <span className="px-2 py-1 bg-blue-500 bg-opacity-20 text-blue-400 text-xs rounded-full">
                      {plane.plane_type}
                    </span>
                  </div>
                  <p className="text-gray-300 text-sm mb-3">{plane.description}</p>
                  {plane.access_methods && plane.access_methods.length > 0 && (
                    <div className="mb-3">
                      <span className="text-gray-400 text-xs">Access Methods:</span>
                      <div className="flex flex-wrap gap-1 mt-1">
                        {plane.access_methods.map((method: string, index: number) => (
                          <span
                            key={index}
                            className="px-2 py-1 bg-green-600 bg-opacity-30 text-green-300 text-xs rounded"
                          >
                            {method}
                          </span>
                        ))}
                      </div>
                    </div>
                  )}
                  {plane.native_creatures && plane.native_creatures.length > 0 && (
                    <div className="text-xs">
                      <span className="text-gray-400">Native Creatures:</span>
                      <span className="text-gray-300 ml-1">
                        {plane.native_creatures.slice(0, 3).join(', ')}
                        {plane.native_creatures.length > 3 && ` +${plane.native_creatures.length - 3} more`}
                      </span>
                    </div>
                  )}
                </div>
              ))}
            </div>
          </div>
        )}

        {activeSection === 'geography' && (
          <div>
            <h3 className="text-xl font-bold text-white mb-4">Geographic Regions</h3>
            <div className="grid gap-4 md:grid-cols-2">
              {geographyRegions.map((region) => (
                <div key={region.id} className="card">
                  <div className="flex items-start justify-between mb-3">
                    <h4 className="font-bold text-white">{region.name}</h4>
                    <span className="px-2 py-1 bg-green-500 bg-opacity-20 text-green-400 text-xs rounded-full">
                      {region.region_type}
                    </span>
                  </div>
                  <p className="text-gray-300 text-sm mb-3">{region.description}</p>
                  <div className="grid grid-cols-2 gap-3 text-xs mb-3">
                    {region.climate && (
                      <div>
                        <span className="text-gray-400">Climate:</span>
                        <span className="text-white ml-1">{region.climate}</span>
                      </div>
                    )}
                    {region.population_density && (
                      <div>
                        <span className="text-gray-400">Population:</span>
                        <span className="text-white ml-1">{region.population_density}</span>
                      </div>
                    )}
                  </div>
                  {region.terrain_types && region.terrain_types.length > 0 && (
                    <div className="mb-2">
                      <span className="text-gray-400 text-xs">Terrain:</span>
                      <div className="flex flex-wrap gap-1 mt-1">
                        {region.terrain_types.map((terrain: string, index: number) => (
                          <span
                            key={index}
                            className="px-2 py-1 bg-gray-700 text-gray-300 text-xs rounded"
                          >
                            {terrain}
                          </span>
                        ))}
                      </div>
                    </div>
                  )}
                  {region.natural_resources && region.natural_resources.length > 0 && (
                    <div className="text-xs">
                      <span className="text-gray-400">Resources:</span>
                      <span className="text-gray-300 ml-1">
                        {region.natural_resources.slice(0, 3).join(', ')}
                        {region.natural_resources.length > 3 && ` +${region.natural_resources.length - 3} more`}
                      </span>
                    </div>
                  )}
                </div>
              ))}
            </div>
          </div>
        )}

        {activeSection === 'history' && (
          <div>
            <h3 className="text-xl font-bold text-white mb-4">Historical Periods</h3>
            <div className="space-y-4">
              {historicalPeriods.map((period) => (
                <div key={period.id} className="card">
                  <div className="flex items-start justify-between mb-3">
                    <h4 className="font-bold text-white">{period.period_name}</h4>
                    <span className="px-2 py-1 bg-yellow-500 bg-opacity-20 text-yellow-400 text-xs rounded-full">
                      {period.start_year} - {period.end_year}
                    </span>
                  </div>
                  <p className="text-gray-300 text-sm mb-3">{period.description}</p>
                  <div className="grid grid-cols-2 gap-3 text-xs">
                    {period.technological_level && (
                      <div>
                        <span className="text-gray-400">Technology:</span>
                        <span className="text-white ml-1">{period.technological_level}</span>
                      </div>
                    )}
                    {period.political_structure && (
                      <div>
                        <span className="text-gray-400">Politics:</span>
                        <span className="text-white ml-1">{period.political_structure}</span>
                      </div>
                    )}
                  </div>
                  {period.key_figures && period.key_figures.length > 0 && (
                    <div className="mt-3 text-xs">
                      <span className="text-gray-400">Key Figures:</span>
                      <span className="text-gray-300 ml-1">
                        {period.key_figures.join(', ')}
                      </span>
                    </div>
                  )}
                </div>
              ))}
            </div>
          </div>
        )}

        {activeSection === 'economics' && (
          <div>
            <h3 className="text-xl font-bold text-white mb-4">Economic Systems</h3>
            <div className="grid gap-4 md:grid-cols-2">
              {economicSystems.map((system) => (
                <div key={system.id} className="card">
                  <div className="flex items-start justify-between mb-3">
                    <h4 className="font-bold text-white">{system.currency_name}</h4>
                    <span className="px-2 py-1 bg-emerald-500 bg-opacity-20 text-emerald-400 text-xs rounded-full">
                      {system.economic_model}
                    </span>
                  </div>
                  <div className="space-y-2 text-sm">
                    {system.currency_abbreviation && (
                      <div>
                        <span className="text-gray-400">Currency:</span>
                        <span className="text-white ml-1">{system.currency_abbreviation}</span>
                      </div>
                    )}
                    {system.banking_system && (
                      <div>
                        <span className="text-gray-400">Banking:</span>
                        <span className="text-white ml-1">{system.banking_system}</span>
                      </div>
                    )}
                  </div>
                  {system.guilds && system.guilds.length > 0 && (
                    <div className="mt-3">
                      <span className="text-gray-400 text-xs">Guilds:</span>
                      <div className="flex flex-wrap gap-1 mt-1">
                        {system.guilds.map((guild: string, index: number) => (
                          <span
                            key={index}
                            className="px-2 py-1 bg-gray-700 text-gray-300 text-xs rounded"
                          >
                            {guild}
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

        {activeSection === 'legal' && (
          <div>
            <h3 className="text-xl font-bold text-white mb-4">Legal Systems</h3>
            <div className="grid gap-4 md:grid-cols-2">
              {legalSystems.map((system) => (
                <div key={system.id} className="card">
                  <div className="flex items-start justify-between mb-3">
                    <h4 className="font-bold text-white">{system.jurisdiction_name}</h4>
                    <span className="px-2 py-1 bg-red-500 bg-opacity-20 text-red-400 text-xs rounded-full">
                      {system.law_type}
                    </span>
                  </div>
                  <div className="space-y-2 text-sm">
                    {system.enforcement_agency && (
                      <div>
                        <span className="text-gray-400">Enforcement:</span>
                        <span className="text-white ml-1">{system.enforcement_agency}</span>
                      </div>
                    )}
                  </div>
                  {system.legal_codes && system.legal_codes.length > 0 && (
                    <div className="mt-3">
                      <span className="text-gray-400 text-xs">Legal Codes:</span>
                      <div className="flex flex-wrap gap-1 mt-1">
                        {system.legal_codes.slice(0, 3).map((code: string, index: number) => (
                          <span
                            key={index}
                            className="px-2 py-1 bg-gray-700 text-gray-300 text-xs rounded"
                          >
                            {code}
                          </span>
                        ))}
                        {system.legal_codes.length > 3 && (
                          <span className="px-2 py-1 bg-gray-600 text-gray-400 text-xs rounded">
                            +{system.legal_codes.length - 3} more
                          </span>
                        )}
                      </div>
                    </div>
                  )}
                </div>
              ))}
            </div>
          </div>
        )}

        {activeSection === 'calendar' && (
          <div>
            <h3 className="text-xl font-bold text-white mb-4">Calendar Systems</h3>
            <div className="grid gap-4 md:grid-cols-2">
              {calendarSystems.map((calendar) => (
                <div key={calendar.id} className="card">
                  <h4 className="font-bold text-white mb-3">{calendar.name}</h4>
                  <div className="grid grid-cols-2 gap-3 text-sm mb-3">
                    <div>
                      <span className="text-gray-400">Days/Week:</span>
                      <span className="text-white ml-1">{calendar.days_per_week}</span>
                    </div>
                    <div>
                      <span className="text-gray-400">Weeks/Month:</span>
                      <span className="text-white ml-1">{calendar.weeks_per_month}</span>
                    </div>
                    <div>
                      <span className="text-gray-400">Months/Year:</span>
                      <span className="text-white ml-1">{calendar.months_per_year}</span>
                    </div>
                  </div>
                  {calendar.month_names && calendar.month_names.length > 0 && (
                    <div className="mb-3">
                      <span className="text-gray-400 text-xs">Months:</span>
                      <div className="flex flex-wrap gap-1 mt-1">
                        {calendar.month_names.slice(0, 6).map((month: string, index: number) => (
                          <span
                            key={index}
                            className="px-2 py-1 bg-gray-700 text-gray-300 text-xs rounded"
                          >
                            {month}
                          </span>
                        ))}
                        {calendar.month_names.length > 6 && (
                          <span className="px-2 py-1 bg-gray-600 text-gray-400 text-xs rounded">
                            +{calendar.month_names.length - 6} more
                          </span>
                        )}
                      </div>
                    </div>
                  )}
                  {calendar.current_calendar_date && (
                    <div className="text-xs">
                      <span className="text-gray-400">Current Date:</span>
                      <span className="text-white ml-1">
                        {typeof calendar.current_calendar_date === 'object'
                          ? JSON.stringify(calendar.current_calendar_date)
                          : calendar.current_calendar_date
                        }
                      </span>
                    </div>
                  )}
                </div>
              ))}
            </div>
          </div>
        )}

        {/* Empty States */}
        {((activeSection === 'pantheons' && pantheons.length === 0 && deities.length === 0) ||
          (activeSection === 'planes' && planes.length === 0) ||
          (activeSection === 'geography' && geographyRegions.length === 0) ||
          (activeSection === 'history' && historicalPeriods.length === 0) ||
          (activeSection === 'economics' && economicSystems.length === 0) ||
          (activeSection === 'legal' && legalSystems.length === 0) ||
          (activeSection === 'calendar' && calendarSystems.length === 0)) && (
          <div className="text-center py-12">
            <Globe className="w-16 h-16 text-gray-500 mx-auto mb-4" />
            <h3 className="text-xl font-bold text-white mb-2">No Data Available</h3>
            <p className="text-gray-400">
              This world building data will appear once the campaign generation is complete.
            </p>
          </div>
        )}
      </div>
    </div>
  );
}