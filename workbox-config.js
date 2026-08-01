module.exports = {
  globDirectory: 'dist/',
  globPatterns: [
    '**/*.{html,js,wasm,css,json,svg,png,ico,woff,woff2}'
  ],
  swSrc: 'static/sw-src.js',
  swDest: 'dist/sw.js',
  // WASM files can be large, increase limit to 15MB to ensure correct caching
  maximumFileSizeToCacheInBytes: 15 * 1024 * 1024
};
