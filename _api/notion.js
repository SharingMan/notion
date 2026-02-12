/**
 * Vercel Edge Function - Notion API Proxy
 * Solves CORS issues when calling Notion API from frontend
 */

export default async function handler(request) {
  const url = new URL(request.url);

  // Extract the Notion API path
  const path = url.pathname.replace('/api/notion/', '');
  const notionUrl = `https://api.notion.com/v1/${path}${url.search}`;

  // Handle CORS preflight
  if (request.method === 'OPTIONS') {
    return new Response(null, {
      status: 204,
      headers: {
        'Access-Control-Allow-Origin': '*',
        'Access-Control-Allow-Methods': 'GET, POST, PATCH, DELETE, OPTIONS',
        'Access-Control-Allow-Headers': 'Authorization, Notion-Version, Content-Type',
        'Access-Control-Max-Age': '86400',
      },
    });
  }

  try {
    // Forward request to Notion API
    const response = await fetch(notionUrl, {
      method: request.method,
      headers: {
        'Authorization': request.headers.get('Authorization') || '',
        'Notion-Version': request.headers.get('Notion-Version') || '2022-06-28',
        'Content-Type': 'application/json',
      },
      body: request.method !== 'GET' && request.method !== 'HEAD'
        ? request.body
        : undefined,
    });

    // Get response body
    const data = await response.text();

    // Return with CORS headers
    return new Response(data, {
      status: response.status,
      statusText: response.statusText,
      headers: {
        'Content-Type': 'application/json',
        'Access-Control-Allow-Origin': '*',
        'Access-Control-Allow-Methods': 'GET, POST, PATCH, DELETE, OPTIONS',
        'Access-Control-Allow-Headers': 'Authorization, Notion-Version, Content-Type',
      },
    });
  } catch (error) {
    console.error('Proxy error:', error);
    return new Response(
      JSON.stringify({
        error: 'Proxy Error',
        message: error.message
      }),
      {
        status: 500,
        headers: {
          'Content-Type': 'application/json',
          'Access-Control-Allow-Origin': '*',
        },
      }
    );
  }
}

export const config = {
  runtime: 'edge',
};
