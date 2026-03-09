import { test, expect, type Page } from '@playwright/test';

// Helper function to wait for fingerprint result
async function waitForFingerprintResult(page: Page, timeout: number = 60000) {
  // Wait for either success or error state
  await page.waitForFunction(() => {
    const loadingElement = document.querySelector('.output');
    if (!loadingElement) return false;
    const text = loadingElement.textContent || '';
    return !text.includes('Getting the visitor identifier');
  }, { timeout });
}

// Helper function to check if page has error
async function hasErrorState(page: Page): Promise<boolean> {
  return page.locator('.heading:has-text("Unexpected error:")').isVisible();
}

// Helper function to get visitor ID
async function getVisitorId(page: Page): Promise<string | null> {
  const visitorIdElement = page.locator('pre.giant');
  const count = await visitorIdElement.count();
  if (count === 0) return null;
  return visitorIdElement.textContent();
}

// Helper function to get confidence score
async function getConfidenceScore(page: Page): Promise<number | null> {
  const confidenceElement = page.locator('.heading:has-text("Confidence score:") + pre.big');
  const count = await confidenceElement.count();
  if (count === 0) return null;
  const text = await confidenceElement.textContent();
  if (!text) return null;
  const score = parseFloat(text);
  return isNaN(score) ? null : score;
}

test.describe('Fingerprint WASM Browser Compatibility', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to the page before each test
    await page.goto('/');
  });

  test('should load WASM module and generate visitor identifier', async ({ page }, testInfo) => {
    // Wait for the fingerprint to be generated
    await waitForFingerprintResult(page);

    // Check for errors
    const hasError = await hasErrorState(page);
    if (hasError) {
      const errorMessage = await page.locator('pre').first().textContent();
      console.error(`[${testInfo.project.name}] Error: ${errorMessage}`);
      throw new Error(`Fingerprint generation failed: ${errorMessage}`);
    }

    // Get and validate visitor ID
    const visitorId = await getVisitorId(page);
    console.log(`[${testInfo.project.name}] Visitor ID: ${visitorId}`);

    expect(visitorId, 'Visitor ID should exist').toBeTruthy();
    expect(visitorId, 'Visitor ID should not be empty').not.toBe('');
    expect(visitorId, 'Visitor ID should not be undefined').not.toBe('undefined');
    expect(visitorId, 'Visitor ID should not be null').not.toBe('null');
    expect(visitorId?.length, 'Visitor ID should have length > 0').toBeGreaterThan(0);
  });

  test('should have valid confidence score', async ({ page }, testInfo) => {
    // Wait for the fingerprint to be generated
    await waitForFingerprintResult(page);

    // Skip if there's an error
    const hasError = await hasErrorState(page);
    if (hasError) {
      test.skip(true, 'Skipping due to error state');
    }

    // Get and validate confidence score
    const confidenceScore = await getConfidenceScore(page);
    console.log(`[${testInfo.project.name}] Confidence: ${confidenceScore}`);

    expect(confidenceScore, 'Confidence score should exist').not.toBeNull();
    expect(confidenceScore, 'Confidence score should be >= 0').toBeGreaterThanOrEqual(0);
    expect(confidenceScore, 'Confidence score should be <= 1').toBeLessThanOrEqual(1);
  });

  test('should not have console errors', async ({ page }, testInfo) => {
    const errors: string[] = [];
    const warnings: string[] = [];

    page.on('console', msg => {
      const text = msg.text();
      if (msg.type() === 'error') {
        errors.push(text);
      } else if (msg.type() === 'warning') {
        warnings.push(text);
      }
    });

    page.on('pageerror', error => {
      errors.push(error.message);
    });

    // Wait for the fingerprint to be generated
    await waitForFingerprintResult(page);

    // Filter out expected errors
    const filteredErrors = errors.filter(e =>
      !e.includes('favicon') &&
      !e.includes('manifest') &&
      !e.includes('[HMR]') &&
      !e.includes('WebSocket') &&
      !e.includes('hot-update') &&
      !e.includes('Source map')
    );

    if (filteredErrors.length > 0) {
      console.error(`[${testInfo.project.name}] Console errors:`, filteredErrors);
    }

    if (warnings.length > 0) {
      console.warn(`[${testInfo.project.name}] Console warnings:`, warnings);
    }

    expect(filteredErrors, 'Should not have console errors').toHaveLength(0);
  });

  test('should get entropy components', async ({ page }, testInfo) => {
    // Wait for the fingerprint to be generated
    await waitForFingerprintResult(page);

    // Skip if there's an error
    const hasError = await hasErrorState(page);
    if (hasError) {
      test.skip(true, 'Skipping due to error state');
    }

    // Find the entropy components section
    const componentsHeading = page.locator('.heading:has-text("Entropy components:")');
    await expect(componentsHeading, 'Entropy components heading should exist').toBeVisible();

    // Get the components text (next sibling pre element)
    const componentsElement = componentsHeading.locator('+ pre');
    const components = await componentsElement.textContent();

    console.log(`[${testInfo.project.name}] Components preview:`, components?.substring(0, 200));

    expect(components, 'Components should exist').toBeTruthy();
    expect(components, 'Components should not be empty').not.toBe('');

    // Validate JSON structure
    let parsed: Record<string, unknown>;
    try {
      parsed = JSON.parse(components || '{}');
    } catch (e) {
      throw new Error(`Failed to parse components JSON: ${e}`);
    }

    const componentKeys = Object.keys(parsed);
    console.log(`[${testInfo.project.name}] Component count: ${componentKeys.length}`);
    expect(componentKeys.length, 'Should have entropy components').toBeGreaterThan(0);

    // Check for expected components
    const expectedComponents = ['canvas', 'fonts', 'timezone', 'userAgent'];
    const hasExpectedComponents = expectedComponents.some(comp =>
      componentKeys.some(key => key.toLowerCase().includes(comp.toLowerCase()))
    );
    expect(hasExpectedComponents, 'Should have at least one expected component').toBe(true);
  });

  test('should complete within reasonable time', async ({ page }, testInfo) => {
    const startTime = Date.now();

    // Wait for the fingerprint to be generated
    await waitForFingerprintResult(page, 120000);

    const duration = Date.now() - startTime;
    console.log(`[${testInfo.project.name}] Duration: ${duration}ms`);

    // Should complete within 60 seconds (generous timeout for slow devices)
    expect(duration, 'Should complete within 60 seconds').toBeLessThan(60000);

    // Check for errors
    const hasError = await hasErrorState(page);
    if (!hasError) {
      // Get the reported time
      const timeElement = page.locator('.heading:has-text("Time took to get the identifier:") + pre.big');
      const reportedTime = await timeElement.textContent();
      console.log(`[${testInfo.project.name}] Reported time: ${reportedTime}`);
    }
  });

  test('should be consistent across multiple runs', async ({ page, context }, testInfo) => {
    // Skip this test for some browsers if needed
    test.skip(testInfo.project.name.includes('Mobile'), 'Skipping consistency test on mobile');

    const visitorIds: string[] = [];

    // Run fingerprinting twice
    for (let i = 0; i < 2; i++) {
      if (i > 0) {
        // Reload page for second run
        await page.reload();
      }

      await waitForFingerprintResult(page);

      const hasError = await hasErrorState(page);
      if (hasError) {
        test.skip(true, 'Skipping due to error state');
      }

      const visitorId = await getVisitorId(page);
      if (visitorId) {
        visitorIds.push(visitorId);
      }
    }

    // In the same browser session, visitor ID should be consistent
    if (visitorIds.length === 2) {
      console.log(`[${testInfo.project.name}] Visitor IDs:`, visitorIds);
      expect(visitorIds[0], 'Visitor ID should be consistent across runs').toBe(visitorIds[1]);
    }
  });

  test('should handle debug mode', async ({ page }, testInfo) => {
    // Wait for the fingerprint to be generated
    await waitForFingerprintResult(page);

    // Check if debug info is available (when debug: true is passed)
    const hasError = await hasErrorState(page);
    if (hasError) {
      test.skip(true, 'Skipping due to error state');
    }

    // Verify that we have all the expected sections
    const expectedSections = [
      'Visitor identifier:',
      'Time took to get the identifier:',
      'Confidence score:',
      'User agent:',
      'Entropy components:'
    ];

    for (const section of expectedSections) {
      const sectionElement = page.locator(`.heading:has-text("${section}")`);
      await expect(sectionElement, `Should have "${section}" section`).toBeVisible();
    }
  });
});
