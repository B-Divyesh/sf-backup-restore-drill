import { readFile } from "node:fs/promises";
import { resolve } from "node:path";

const source = resolve("site/public/staticwebapp.config.json");
const artifact = resolve(process.argv[2] ?? "dist/site", "staticwebapp.config.json");
const expectedGlobalHeaders = {
  "Cache-Control": "public, max-age=0, must-revalidate",
  "Content-Security-Policy": "default-src 'self'; img-src 'self'; style-src 'self'; script-src 'self'; connect-src 'self'; object-src 'none'; base-uri 'none'; frame-ancestors 'none'",
  "Permissions-Policy": "camera=(), microphone=(), geolocation=()",
  "Referrer-Policy": "no-referrer",
  "X-Content-Type-Options": "nosniff"
};
const expectedRouteHeaders = new Map([
  ["/assets/*", "public, max-age=31536000, immutable"],
  ["/restore-path.webp", "public, max-age=31536000, immutable"],
  ["/sw.js", "no-cache"]
]);

async function parse(path) {
  try {
    return JSON.parse(await readFile(path, "utf8"));
  } catch (error) {
    throw new Error(`Could not read deploy response policy at ${path}: ${error.message}`);
  }
}

function assertPolicy(policy, label) {
  for (const [header, value] of Object.entries(expectedGlobalHeaders)) {
    if (policy.globalHeaders?.[header] !== value) {
      throw new Error(`${label} must set ${header} to ${JSON.stringify(value)}`);
    }
  }
  const routes = new Map(policy.routes?.map((entry) => [entry.route, entry.headers?.["Cache-Control"]]));
  for (const [route, value] of expectedRouteHeaders) {
    if (routes.get(route) !== value) {
      throw new Error(`${label} must set Cache-Control ${JSON.stringify(value)} for ${route}`);
    }
  }
}

const [sourcePolicy, artifactPolicy] = await Promise.all([parse(source), parse(artifact)]);
assertPolicy(sourcePolicy, "source staticwebapp.config.json");
assertPolicy(artifactPolicy, "built staticwebapp.config.json");
console.log("response-policy: source and deploy artifact carry the required Azure Static Web Apps headers");
