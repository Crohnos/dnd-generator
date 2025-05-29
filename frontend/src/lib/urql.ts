import { createClient, cacheExchange, fetchExchange, subscriptionExchange } from 'urql';
import { createClient as createWSClient } from 'graphql-ws';

const wsClient = typeof window !== 'undefined' ? createWSClient({
  url: process.env.NEXT_PUBLIC_WS_URL || 'ws://localhost:8080/v1/graphql',
  connectionParams: {
    headers: {
      'x-hasura-admin-secret': 'hasura-admin-secret', // Development only
    },
  },
}) : null;

export const urqlClient = createClient({
  url: process.env.NEXT_PUBLIC_GRAPHQL_URL || 'http://localhost:8080/v1/graphql',
  exchanges: [
    cacheExchange,
    fetchExchange,
    ...(wsClient ? [subscriptionExchange({
      forwardSubscription(request) {
        const input = { ...request, query: request.query || '' };
        return {
          subscribe: (sink) => {
            const unsubscribe = wsClient.subscribe(input, sink);
            return { unsubscribe };
          },
        };
      },
    })] : []),
  ],
  fetchOptions: () => {
    return {
      headers: {
        'x-hasura-admin-secret': 'hasura-admin-secret', // Development only
      },
    };
  },
});