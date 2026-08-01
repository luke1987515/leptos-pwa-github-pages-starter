importScripts('https://storage.googleapis.com/workbox-cdn/releases/7.0.0/workbox-sw.js');

if (workbox) {
  console.log('Workbox v7 loaded successfully');

  // Inject precache manifest here
  workbox.precaching.precacheAndRoute([{"revision":"9304fffd2a260dea94beb185a2f0e221","url":"style-9ecabd239819f552.css"},{"revision":"227bc00a0d699bb6636ba3eb13349258","url":"manifest.json"},{"revision":"7547ccd990eeaafdf89be7dc3b8523bb","url":"leptos-pwa-github-pages-starter-604a834a08ae6815_bg.wasm"},{"revision":"8a51f245be9e7407428ea28e8098a430","url":"leptos-pwa-github-pages-starter-604a834a08ae6815.js"},{"revision":"bb0eb2dcd7c925a0c6ac425fb14a4d0b","url":"index.html"},{"revision":"c8570d0e8c8a045b44efd6b72a852a36","url":"icons/icon.svg"}] || []);

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
