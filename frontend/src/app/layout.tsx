import type { Metadata } from 'next'
import { Inter } from 'next/font/google'
import './globals.css'
import { UrqlProvider } from '@/components/UrqlProvider'
import { Navigation } from '@/components/Navigation'

const inter = Inter({ subsets: ['latin'] })

export const metadata: Metadata = {
  title: 'D&D Campaign Generator',
  description: 'AI-powered campaign generator for Dungeons & Dragons',
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en" className="dark">
      <body className={inter.className}>
        <UrqlProvider>
          <div className="min-h-screen bg-gray-900">
            <Navigation />
            <main>{children}</main>
          </div>
        </UrqlProvider>
      </body>
    </html>
  )
}