var cacheName = 'client';

var filesToCache = [
	'./',
	'./index.html',
	'./pkg/bundle.js',
	'./pkg/client_bg.wasm',
	'./bulma.min.css'
];

/* Start the service worker and cache all app content */
self.addEventListener('install', function(e) {
	e.waitUntil(
		caches.open(cacheName).then(function(cache) {
			return cache.addAll(filesToCache);
		})
	);
});

/* Serve cached content when offline */
self.addEventListener('fetch', function(event) {
	caches.match(event.request).then((resp) => {
		return resp || fetch(event.request).then((response) => {
        		return caches.open(cacheName).then((cache) => {
				cache.put(event.request, response.clone());
				return response;
			});
	      	});
	})
});
