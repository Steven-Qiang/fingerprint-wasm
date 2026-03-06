import init, { get_fingerprint } from '../../..';

export async function initWasm(): Promise<void> {
  try {
    await init();
  } catch (error) {
    console.error('WASM module initialization failed:', error);
    throw error;
  }
}

export const getFingerprint = get_fingerprint;
