import { create } from 'zustand';

export interface PlayerCharacter {
  name: string;
  class: string;
  race: string;
  background?: string;
  personalityTraits: string[];
}

interface CampaignState {
  // Form state
  name: string;
  setting: string;
  themes: string[];
  playerCharacters: PlayerCharacter[];
  currentStep: number;

  // Actions for each field
  setName: (name: string) => void;
  setSetting: (setting: string) => void;
  setThemes: (themes: string[]) => void;
  setPlayerCharacters: (characters: PlayerCharacter[]) => void;
  addPlayerCharacter: (character: PlayerCharacter) => void;
  updatePlayerCharacter: (index: number, character: PlayerCharacter) => void;
  removePlayerCharacter: (index: number) => void;
  setCurrentStep: (step: number) => void;
  nextStep: () => void;
  prevStep: () => void;
  reset: () => void;
  
  // Validation
  isStepValid: (step: number) => boolean;
}

const initialState = {
  name: '',
  setting: '',
  themes: [],
  playerCharacters: [],
  currentStep: 0,
};

export const useCampaignStore = create<CampaignState>((set, get) => ({
  ...initialState,

  setName: (name: string) => set({ name }),
  
  setSetting: (setting: string) => set({ setting }),
  
  setThemes: (themes: string[]) => set({ themes }),
  
  setPlayerCharacters: (characters: PlayerCharacter[]) => 
    set({ playerCharacters: characters }),
  
  addPlayerCharacter: (character: PlayerCharacter) =>
    set((state) => ({ 
      playerCharacters: [...state.playerCharacters, character] 
    })),
  
  updatePlayerCharacter: (index: number, character: PlayerCharacter) =>
    set((state) => ({
      playerCharacters: state.playerCharacters.map((char, i) => 
        i === index ? character : char
      ),
    })),
  
  removePlayerCharacter: (index: number) =>
    set((state) => ({
      playerCharacters: state.playerCharacters.filter((_, i) => i !== index),
    })),
  
  setCurrentStep: (step: number) => set({ currentStep: step }),
  
  nextStep: () => set((state) => ({ 
    currentStep: Math.min(state.currentStep + 1, 3) 
  })),
  
  prevStep: () => set((state) => ({ 
    currentStep: Math.max(state.currentStep - 1, 0) 
  })),
  
  reset: () => set(initialState),

  isStepValid: (step: number) => {
    const state = get();
    
    switch (step) {
      case 0: // Basic Information
        return state.name.trim().length > 0;
      
      case 1: // Player Characters
        return state.playerCharacters.length > 0 && 
               state.playerCharacters.every(char => 
                 char.name.trim().length > 0 && 
                 char.class.trim().length > 0 && 
                 char.race.trim().length > 0
               );
      
      case 2: // World & Themes
        return state.setting.trim().length > 0 && state.themes.length > 0;
      
      case 3: // Review & Generate
        return state.isStepValid(0) && state.isStepValid(1) && state.isStepValid(2);
      
      default:
        return false;
    }
  },
}));