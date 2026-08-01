importScripts('https://storage.googleapis.com/workbox-cdn/releases/7.0.0/workbox-sw.js');

if (workbox) {
  console.log('Workbox v7 loaded successfully');

  // Inject precache manifest here
  workbox.precaching.precacheAndRoute(self.__WB_MANIFEST || []);

  // Single Page Application routing fallback:
  // Redirect all navigation requests to index.html so leptos-router can handle them.
  try {
    const handler = workbox.precaching.createHandlerBoundToURL('index.html');
    const navigationRoute = new workbox.routing.NavigationRoute(handler);
    workbox.routing.registerRoute(navigationRoute);
  } catch (error) {
    console.error('SPA navigation route fallback registration failed:', error);
  }

  // Force incoming Service Worker to become active immediately
  self.addEventListener('install', () => {
    self.skipWaiting();
  });

  self.addEventListener('activate', (event) => {
    event.waitUntil(self.clients.claim());
  });
} else {
  console.error('Workbox failed to load');
}
