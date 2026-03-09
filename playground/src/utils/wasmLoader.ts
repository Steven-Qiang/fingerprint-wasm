import init from '../../..';

export async function initWasm(): Promise<void> {
  try {
    await init();
  } catch (error) {
    console.error('WASM module initialization failed:', error);
    throw error;
  }
}
