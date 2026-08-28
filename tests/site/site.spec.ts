import AxeBuilder from "@axe-core/playwright";
import { expect, test } from "@playwright/test";

test("home is semantic, responsive, and free of serious accessibility issues", async ({ page }) => {
  const errors: string[] = [];
  page.on("console", (message) => { if (message.type() === "error") errors.push(message.text()); });
  await page.goto("/");
  await expect(page).toHaveTitle(/Restore Drill/);
  await expect(page.locator("html")).toHaveAttribute("lang", "en");
  await expect(page.locator("main")).toHaveCount(1);
  await expect(page.locator("h1")).toHaveCount(1);
  await expect(page.locator("img[alt]")).toHaveCount(1);
  const overflow = await page.evaluate(() => document.documentElement.scrollWidth - document.documentElement.clientWidth);
  expect(overflow).toBeLessThanOrEqual(1);
  const results = await new AxeBuilder({ page }).analyze();
  expect(results.violations.filter((item) => ["serious", "critical"].includes(item.impact ?? ""))).toEqual([]);
  expect(errors).toEqual([]);
});

test("demo exposes both pass and actionable failure states", async ({ page }) => {
  await page.goto("/#demo");
  await page.getByRole("button", { name: "Run healthy sample" }).click();
  await expect(page.getByText("Recovery proven", { exact: true })).toBeVisible();
  await expect(page.locator("#stamp")).toHaveText("Verified");
  await expect(page.locator("#terminal")).toContainText("exit 0");

  await page.getByRole("button", { name: "Inject missing file" }).click();
  await expect(page.getByText("Failure caught loudly", { exact: true })).toBeVisible();
  await expect(page.locator("#stamp")).toHaveText("Failed");
  await expect(page.locator("#demo-hint")).toContainText("confirm the backup includes this path");
  await expect(page.locator("#terminal")).toContainText("exit 1");
});

test("offline state tells the reader what remains available", async ({ page, context }) => {
  await page.goto("/");
  await page.evaluate(() => navigator.serviceWorker.ready);
  await page.reload();
  await context.setOffline(true);
  await page.reload();
  await expect(page.locator("#offline")).toBeVisible();
  await expect(page.locator("#offline")).toContainText("Reconnect");
  await expect(page.locator("h1")).toContainText("Recovery proven");
});

for (const path of ["/privacy/", "/terms/"]) {
  test(`${path} has a complete legal document`, async ({ page }) => {
    await page.goto(path);
    await expect(page.locator("main h1")).toHaveCount(1);
    await expect(page.locator("main section")).toHaveCount(3);
    const results = await new AxeBuilder({ page }).analyze();
    expect(results.violations.filter((item) => ["serious", "critical"].includes(item.impact ?? ""))).toEqual([]);
  });
}
