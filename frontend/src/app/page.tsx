import Link from 'next/link';
import { Sparkles, Users, MapPin, Scroll, ArrowRight } from 'lucide-react';

export default function Home() {
  return (
    <div className="min-h-screen">
      {/* Hero Section */}
      <section className="relative py-20 px-4 sm:px-6 lg:px-8">
        <div className="max-w-7xl mx-auto text-center">
          <h1 className="text-4xl md:text-6xl font-bold text-white mb-6">
            Create Epic{' '}
            <span className="text-dnd-purple">D&D Campaigns</span>{' '}
            with AI
          </h1>
          <p className="text-xl text-gray-300 mb-8 max-w-3xl mx-auto">
            Generate rich, interconnected campaigns with compelling NPCs, immersive locations, 
            and engaging quest hooks. Let AI craft the perfect adventure for your table.
          </p>
          <div className="flex flex-col sm:flex-row gap-4 justify-center">
            <Link href="/campaigns/new" className="btn-primary text-lg px-8 py-3 inline-flex items-center">
              <Sparkles className="mr-2 h-5 w-5" />
              Generate Campaign
            </Link>
            <Link href="/campaigns" className="btn-secondary text-lg px-8 py-3 inline-flex items-center">
              <Scroll className="mr-2 h-5 w-5" />
              View Campaigns
            </Link>
          </div>
        </div>
      </section>

      {/* Feature Cards */}
      <section className="py-16 px-4 sm:px-6 lg:px-8">
        <div className="max-w-7xl mx-auto">
          <h2 className="text-3xl font-bold text-center text-white mb-12">
            Everything You Need for Epic Adventures
          </h2>
          <div className="grid md:grid-cols-3 gap-8">
            {/* NPCs Feature */}
            <div className="card text-center">
              <div className="flex justify-center mb-4">
                <Users className="h-12 w-12 text-dnd-purple" />
              </div>
              <h3 className="text-xl font-bold text-white mb-4">Rich NPCs</h3>
              <p className="text-gray-300">
                AI-generated characters with deep personalities, motivations, secrets, and 
                interconnected relationships that drive your story forward.
              </p>
            </div>

            {/* Locations Feature */}
            <div className="card text-center">
              <div className="flex justify-center mb-4">
                <MapPin className="h-12 w-12 text-dnd-gold" />
              </div>
              <h3 className="text-xl font-bold text-white mb-4">Immersive Locations</h3>
              <p className="text-gray-300">
                Detailed locations with atmospheric descriptions, notable features, 
                hidden secrets, and logical connections to create a living world.
              </p>
            </div>

            {/* Quest Hooks Feature */}
            <div className="card text-center">
              <div className="flex justify-center mb-4">
                <Scroll className="h-12 w-12 text-dnd-purple" />
              </div>
              <h3 className="text-xl font-bold text-white mb-4">Engaging Quests</h3>
              <p className="text-gray-300">
                Compelling quest hooks that weave together your NPCs and locations, 
                creating cohesive storylines with meaningful rewards and challenges.
              </p>
            </div>
          </div>
        </div>
      </section>

      {/* Call to Action */}
      <section className="py-16 px-4 sm:px-6 lg:px-8 bg-gray-800">
        <div className="max-w-4xl mx-auto text-center">
          <h2 className="text-3xl font-bold text-white mb-6">
            Ready to Start Your Next Adventure?
          </h2>
          <p className="text-xl text-gray-300 mb-8">
            Create your first AI-generated campaign in minutes. No prep time required.
          </p>
          <Link 
            href="/campaigns/new" 
            className="btn-primary text-lg px-8 py-3 inline-flex items-center"
          >
            Get Started
            <ArrowRight className="ml-2 h-5 w-5" />
          </Link>
        </div>
      </section>
    </div>
  );
}