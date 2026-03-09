<p align="center">
  <a href="https://github.com/Steven-Qiang/fingerprint-wasm">
    <img src="https://img.shields.io/badge/FingerprintWASM-Rust%2BWASM-orange" alt="Fingerprint WASM logo" />
  </a>
</p>
<p align="center">
  <a href="https://www.npmjs.com/package/fingerprint-wasm"><img src="https://img.shields.io/npm/v/fingerprint-wasm.svg" alt="Current NPM version"></a>
  <a href="https://github.com/Steven-Qiang/fingerprint-wasm/blob/main/LICENSE"><img src="https://img.shields.io/github/license/Steven-Qiang/fingerprint-wasm" alt="License"></a>
</p>

# Fingerprint WASM

Fingerprint WASM is an open-source, client-side, browser fingerprinting library written in Rust and compiled to WebAssembly. It is a Rust + WASM port of the popular [FingerprintJS](https://github.com/fingerprintjs/fingerprintjs) library.

This library queries browser attributes and computes a hashed visitor identifier from them. Unlike cookies and local storage, a fingerprint stays the same in incognito/private mode and even when browser data is purged.

## Why WebAssembly?

Unlike pure JavaScript implementations, Fingerprint WASM compiles all core fingerprinting logic into WebAssembly:

- **Enhanced Security**: Core fingerprinting algorithms are compiled to WASM binary, making them significantly harder to reverse engineer, tamper with, or spoof compared to plain JavaScript
- **Code Protection**: The WASM binary obfuscates the fingerprinting logic, providing an additional layer of protection against malicious actors
- **Tamper Resistant**: Modifying WASM bytecode is far more difficult than modifying JavaScript source code

## Features

- **Secure Core**: All fingerprinting logic runs in WebAssembly, not JavaScript
- **Comprehensive Fingerprinting**: Collects 40+ browser entropy sources
- **TypeScript Support**: Full TypeScript type definitions included
- **Framework Agnostic**: Works with any JavaScript framework or vanilla JS
- **Small Bundle Size**: Optimized WASM output

## Demo

Clone the repository and run the playground:

```bash
git clone https://github.com/Steven-Qiang/fingerprint-wasm.git
cd fingerprint-wasm
pnpm install
pnpm build:wasm
cd playground
pnpm dev
```

Then open http://localhost:5173 in your browser to see your visitor identifier.

## Installation

### NPM

```bash
npm install fingerprint-wasm
# or
pnpm add fingerprint-wasm
# or
yarn add fingerprint-wasm
```

### CDN

```html
<script type="module">
  import init, { getFingerprint } from 'https://unpkg.com/fingerprint-wasm/dist/fingerprint_wasm.js';
  
  async function main() {
    await init();
    const result = await getFingerprint();
    console.log('Visitor ID:', result.visitorId);
  }
  
  main();
</script>
```

## Usage

### Basic Usage

```javascript
import init, { getFingerprint } from 'fingerprint-wasm';

async function getVisitorId() {
  // Initialize the WASM module
  await init();
  
  // Get the fingerprint
  const result = await getFingerprint();
  
  console.log('Visitor ID:', result.visitorId);
  console.log('Confidence:', result.confidence);
  console.log('Components:', result.componentsJson);
  
  return result.visitorId;
}

getVisitorId();
```

### With Vue.js

```vue
<script setup>
import { ref, onMounted } from 'vue';
import init, { getFingerprint } from 'fingerprint-wasm';

const visitorId = ref('');
const isLoading = ref(true);
const confidence = ref(null);

onMounted(async () => {
  await init();
  const result = await getFingerprint();
  visitorId.value = result.visitorId;
  confidence.value = result.confidence;
  isLoading.value = false;
});
</script>

<template>
  <div v-if="isLoading">Loading...</div>
  <div v-else>
    <p>Visitor ID: {{ visitorId }}</p>
    <p>Confidence: {{ confidence?.score }}</p>
  </div>
</template>
```

### With React

```jsx
import { useEffect, useState } from 'react';
import init, { getFingerprint } from 'fingerprint-wasm';

function App() {
  const [visitorId, setVisitorId] = useState('');
  const [confidence, setConfidence] = useState(null);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    async function loadFingerprint() {
      await init();
      const result = await getFingerprint();
      setVisitorId(result.visitorId);
      setConfidence(result.confidence);
      setIsLoading(false);
    }
    loadFingerprint();
  }, []);

  if (isLoading) return <div>Loading...</div>;
  return (
    <div>
      <p>Visitor ID: {visitorId}</p>
      <p>Confidence: {confidence?.score}</p>
    </div>
  );
}

export default App;
```

## API Reference

### `init()`

Initializes the WebAssembly module. Must be called before using `getFingerprint()`.

```javascript
import init from 'fingerprint-wasm';

await init();
```

### `getFingerprint(options?: FingerprintOptions): Promise<FingerprintResult>`

Returns a promise that resolves to a `FingerprintResult` object containing the visitor identifier and related data.

#### Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `options` | `FingerprintOptions` | No | Optional configuration for fingerprinting |

#### FingerprintOptions

| Property | Type | Description |
|----------|------|-------------|
| `debug` | `boolean` | Enable debug mode for detailed logging |

#### Returns

| Property | Type | Description |
|----------|------|-------------|
| `visitorId` | `string` | The unique visitor identifier |
| `confidence` | `ConfidenceResult` | Confidence score and comment |
| `componentsJson` | `string` | JSON string of all entropy components |
| `version` | `string` | Library version |

### `ConfidenceResult`

| Property | Type | Description |
|----------|------|-------------|
| `score` | `number` | Confidence score (0-1) |
| `comment` | `string \| undefined` | Optional comment about the confidence |

## Entropy Sources

The library collects the following entropy sources to generate the fingerprint:

| Source | Description |
|--------|-------------|
| `architecture` | CPU architecture |
| `audio` | Audio context fingerprint |
| `audioBaseLatency` | Audio context base latency |
| `applePay` | Apple Pay availability |
| `canvas` | Canvas 2D fingerprint |
| `colorDepth` | Screen color depth |
| `colorGamut` | Color gamut support |
| `contrast` | Contrast preference |
| `cookiesEnabled` | Cookies enabled status |
| `cpuClass` | CPU class (legacy) |
| `dateTimeLocale` | Date/time locale |
| `deviceMemory` | Device memory |
| `domBlockers` | Ad blocker detection |
| `fonts` | Available fonts |
| `fontPreferences` | Font preferences |
| `forcedColors` | Forced colors mode |
| `hardwareConcurrency` | CPU cores |
| `hdr` | HDR support |
| `indexedDB` | IndexedDB availability |
| `invertedColors` | Inverted colors mode |
| `languages` | Browser languages |
| `localStorage` | Local storage availability |
| `math` | Math precision fingerprint |
| `monochrome` | Monochrome display |
| `openDatabase` | WebSQL availability |
| `osCpu` | OS CPU info |
| `pdfViewerEnabled` | PDF viewer status |
| `platform` | Browser platform |
| `plugins` | Browser plugins |
| `privateClickMeasurement` | Private click measurement |
| `reducedMotion` | Reduced motion preference |
| `reducedTransparency` | Reduced transparency |
| `screenFrame` | Screen frame dimensions |
| `screenResolution` | Screen resolution |
| `sessionStorage` | Session storage availability |
| `timezone` | Timezone |
| `touchSupport` | Touch support |
| `vendor` | Browser vendor |
| `vendorFlavors` | Browser vendor flavors |
| `webGlBasics` | WebGL basic info |
| `webGlExtensions` | WebGL extensions |

## Building from Source

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.70+)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- [Node.js](https://nodejs.org/) (18+)
- [pnpm](https://pnpm.io/) (recommended)

### Build Steps

```bash
# Clone the repository
git clone https://github.com/Steven-Qiang/fingerprint-wasm.git
cd fingerprint-wasm

# Install dependencies
pnpm install

# Build the WASM module
pnpm build:wasm

# For optimized production build
pnpm build:wasm:optimized

# Run tests
pnpm test
```

## Testing

The project includes comprehensive Playwright tests for browser compatibility:

```bash
# Run all tests
cd tests
pnpm test

# Run tests for specific browser
pnpm test -- --project="Desktop Chrome"
pnpm test -- --project="Desktop Firefox"
pnpm test -- --project="Desktop Safari"

# Run tests for mobile devices
pnpm test -- --project="iPhone 14"
pnpm test -- --project="Pixel 7"
```

## Project Structure

```
fingerprint-wasm/
├── wasm/                   # Core Rust/WASM source code
│   ├── src/
│   │   ├── sources/        # Entropy source implementations
│   │   ├── utils/          # Utility functions
│   │   ├── agent.rs        # Main fingerprint agent
│   │   ├── confidence.rs   # Confidence calculation
│   │   └── lib.rs          # WASM bindings
│   └── Cargo.toml
├── playground/             # Vue.js demo application
├── tests/                  # Playwright browser compatibility tests
├── dist/                   # Compiled WASM output
└── package.json
```

All core fingerprinting logic resides in the `wasm/` directory and is compiled to WebAssembly for enhanced security.

## Limitations

### Accuracy

Since Fingerprint WASM processes and generates fingerprints from within the browser itself, the accuracy is limited compared to commercial solutions that use server-side processing.

### Security Considerations

While WASM provides better code protection than JavaScript, no client-side solution is completely secure. Determined attackers may still find ways to analyze or bypass fingerprinting. For mission-critical applications, consider using a commercial solution with server-side processing.

## Browser Support

The library supports all modern browsers with WebAssembly support:

| Browser | Minimum Version |
|---------|----------------|
| Chrome | 57+ |
| Firefox | 52+ |
| Safari | 11+ |
| Edge | 16+ |
| Opera | 44+ |
| iOS Safari | 11+ |
| Chrome Android | 57+ |

## Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

This project is a Rust + WebAssembly port of [FingerprintJS](https://github.com/fingerprintjs/fingerprintjs). Special thanks to the FingerprintJS team for their excellent work on browser fingerprinting.

## Related Projects

- [FingerprintJS](https://github.com/fingerprintjs/fingerprintjs) - Original JavaScript library
- [Fingerprint Pro](https://fingerprint.com) - Commercial version with higher accuracy and server-side processing

