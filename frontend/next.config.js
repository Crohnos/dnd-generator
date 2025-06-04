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
  experimental: {
    // Memory optimizations
    workerThreads: false,
    cpus: 1,
  },
  // Webpack memory optimizations
  webpack: (config, { dev }) => {
    if (dev) {
      // Reduce memory usage in development
      config.optimization.splitChunks = {
        chunks: 'all',
        cacheGroups: {
          default: false,
          vendors: false,
          vendor: {
            chunks: 'all',
            test: /node_modules/,
            name: 'vendor',
            enforce: true,
          },
        },
      };
      
      // Limit parallel processing
      config.parallelism = 1;
      
      // Reduce watch options
      config.watchOptions = {
        ignored: /node_modules/,
        poll: 3000,
      };
    }
    return config;
  },
}

module.exports = nextConfig