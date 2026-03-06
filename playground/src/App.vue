<template>
  <div class="content">
    <header>
      <a href="https://github.com/Steven-Qiang/fingerprint-wasm" target="_blank">
        <svg height="5vh" viewBox="0 0 400 40" xmlns="http://www.w3.org/2000/svg">
          <text x="0" y="30" font-family="sans-serif" font-size="28" font-weight="bold" fill="#f04405">FingerprintJS WASM</text>
        </svg>
      </a>
    </header>

    <div class="buttons">
      <button :disabled="isLoading || hasError" @click="copyDebugData">
        Copy Debug Data
      </button>
      <button :disabled="isLoading || hasError" @click="shareDebugData">
        Share Debug Data
      </button>
    </div>

    <div class="outputHolder">
      <section class="output">
        <template v-if="isLoading">
          <div class="heading">
            Getting the visitor identifier...
          </div>
        </template>

        <template v-else-if="hasError">
          <div class="heading">
            Unexpected error:
          </div>
          <pre>{{ errorMessage }}</pre>
          <div class="heading">
            Time passed before the error:
          </div>
          <pre class="big">{{ totalTime }}ms</pre>
          <div class="heading">
            User agent:
          </div>
          <pre>{{ userAgent }}</pre>
        </template>

        <template v-else>
          <div class="heading">
            Visitor identifier:
          </div>
          <pre class="giant">{{ visitorId }}</pre>

          <div class="heading">
            Time took to get the identifier:
          </div>
          <pre class="big">{{ totalTime }}ms</pre>

          <div class="heading">
            Confidence score:
          </div>
          <pre class="big">{{ confidence.score }}</pre>

          <div class="heading">
            User agent:
          </div>
          <pre>{{ userAgent }}</pre>

          <div class="heading">
            Entropy components:
          </div>
          <pre>{{ componentsToDebugString(components) }}</pre>
        </template>
      </section>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { AgentResult } from '../..';
import { computed, onMounted, ref } from 'vue';
import { getFingerprint, initWasm } from './utils/wasmLoader';

const isLoading = ref(true);
const hasError = ref(false);
const errorMessage = ref('');
const result = ref<AgentResult | null>(null);
const totalTime = ref(0);
const userAgent = ref(navigator.userAgent);

const visitorId = computed(() => result.value?.visitor_id || '');
const confidence = computed(() => result.value?.confidence || { score: 0 });
const components = computed(() => result.value?.components_json || '{}');

function componentsToDebugString(components_json: string): string {
  try {
    const componentsObj = JSON.parse(components_json);
    return JSON.stringify(componentsObj, null, 2);
  } catch {
    return 'Failed to parse components';
  }
}

const debugText = computed(() => {
  if (!result.value) return '';
  return `Visitor identifier: \`${visitorId.value}\`
Time took to get the identifier: ${totalTime.value}ms
Confidence: ${JSON.stringify(confidence.value)}
User agent: \`${userAgent.value}\`
Entropy components:
\`\`\`
${componentsToDebugString(components.value)}
\`\`\``;
});

async function copyDebugData() {
  try {
    await navigator.clipboard.writeText(debugText.value);
    alert('Debug data copied to clipboard!');
  } catch {
    const textarea = document.createElement('textarea');
    textarea.value = debugText.value;
    document.body.appendChild(textarea);
    textarea.focus();
    textarea.select();
    try {
      document.execCommand('copy');
      alert('Debug data copied to clipboard!');
    } catch {
      alert('Failed to copy debug data');
    }
    document.body.removeChild(textarea);
  }
}

async function shareDebugData() {
  if (!navigator.share) {
    alert(`Sharing is unavailable.

Sharing is available in mobile browsers and only on HTTPS websites. ${
  location.protocol === 'https:'
    ? 'Use a mobile device or the Copy button instead.'
    : `Open https://${location.host}${location.pathname}${location.search} instead.`
}`);
    return;
  }
  try {
    await navigator.share({ text: debugText.value });
  } catch {
    // Do nothing in case of a share abort
  }
}

onMounted(async () => {
  const startTime = Date.now();

  try {
    await initWasm();
    result.value = await getFingerprint();
    console.log(result.value);
    totalTime.value = Date.now() - startTime;
  } catch (error) {
    hasError.value = true;
    errorMessage.value = error instanceof Error ? error.message : 'Unknown error';
    totalTime.value = Date.now() - startTime;
  } finally {
    isLoading.value = false;
  }
});
</script>
