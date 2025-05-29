/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  swcMinify: true,
  output: 'standalone', // Enable for Docker production builds
  typescript: {
    // We'll handle TypeScript errors during build
    ignoreBuildErrors: false,
  },
  eslint: {
    // We'll handle ESLint errors during build
    ignoreDuringBuilds: false,
  },
  // Environment variables that should be available on the client
  env: {
    NEXT_PUBLIC_GRAPHQL_URL: process.env.NEXT_PUBLIC_GRAPHQL_URL,
    NEXT_PUBLIC_API_URL: process.env.NEXT_PUBLIC_API_URL,
    NEXT_PUBLIC_WS_URL: process.env.NEXT_PUBLIC_WS_URL,
    NEXT_PUBLIC_HASURA_ADMIN_SECRET: process.env.NEXT_PUBLIC_HASURA_ADMIN_SECRET,
  },
  // Performance optimizations
  // experimental: {
  //   optimizeCss: true, // Disabled due to critters dependency issue
  // },
}

module.exports = nextConfig