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
  import init, { get_fingerprint } from 'https://unpkg.com/fingerprint-wasm/dist/fingerprint_wasm.js';
  
  async function main() {
    await init();
    const result = await get_fingerprint();
    console.log('Visitor ID:', result.visitor_id);
  }
  
  main();
</script>
```

## Usage

### Basic Usage

```javascript
import init, { get_fingerprint } from 'fingerprint-wasm';

async function getVisitorId() {
  // Initialize the WASM module
  await init();
  
  // Get the fingerprint
  const result = await get_fingerprint();
  
  console.log('Visitor ID:', result.visitor_id);
  console.log('Confidence:', result.confidence);
  console.log('Components:', result.components_json);
  
  return result.visitor_id;
}

getVisitorId();
```

### With Vue.js

```vue
<script setup>
import { ref, onMounted } from 'vue';
import init, { get_fingerprint } from 'fingerprint-wasm';

const visitorId = ref('');
const isLoading = ref(true);

onMounted(async () => {
  await init();
  const result = await get_fingerprint();
  visitorId.value = result.visitor_id;
  isLoading.value = false;
});
</script>

<template>
  <div v-if="isLoading">Loading...</div>
  <div v-else>Visitor ID: {{ visitorId }}</div>
</template>
```

### With React

```jsx
import { useEffect, useState } from 'react';
import init, { get_fingerprint } from 'fingerprint-wasm';

function App() {
  const [visitorId, setVisitorId] = useState('');
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    async function loadFingerprint() {
      await init();
      const result = await get_fingerprint();
      setVisitorId(result.visitor_id);
      setIsLoading(false);
    }
    loadFingerprint();
  }, []);

  if (isLoading) return <div>Loading...</div>;
  return <div>Visitor ID: {visitorId}</div>;
}
```

## API Reference

### `get_fingerprint(): Promise<AgentResult>`

Returns a promise that resolves to an `AgentResult` object containing:

| Property | Type | Description |
|----------|------|-------------|
| `visitor_id` | `string` | The unique visitor identifier (128-bit hash) |
| `confidence` | `ConfidenceResult` | Confidence score and comment |
| `components_json` | `string` | JSON string of all entropy components |
| `version` | `string` | Library version |

### `ConfidenceResult`

| Property | Type | Description |
|----------|------|-------------|
| `score` | `number` | Confidence score (0-1) |
| `comment` | `string \| null` | Optional comment about the confidence |

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

- Chrome 57+
- Firefox 52+
- Safari 11+
- Edge 16+

## Contributing

See the [Contributing Guide](CONTRIBUTING.md) to learn how to contribute to the project.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

This project is a Rust + WebAssembly port of [FingerprintJS](https://github.com/fingerprintjs/fingerprintjs). Special thanks to the FingerprintJS team for their excellent work on browser fingerprinting.

## Related Projects

- [FingerprintJS](https://github.com/fingerprintjs/fingerprintjs) - Original JavaScript library
- [Fingerprint Pro](https://fingerprint.com) - Commercial version with higher accuracy and server-side processing
