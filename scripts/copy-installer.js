import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const rootDir = path.resolve(__dirname, "..");
const bundleDir = path.join(rootDir, "src-tauri", "target", "release", "bundle");
const outDir = path.join(rootDir, "dist-installer");

function findInstallers(dir) {
  const results = [];
  if (!fs.existsSync(dir)) return results;

  const entries = fs.readdirSync(dir, { withFileTypes: true });
  for (const entry of entries) {
    const fullPath = path.join(dir, entry.name);
    if (entry.isDirectory()) {
      results.push(...findInstallers(fullPath));
    } else if (
      entry.isFile() &&
      (entry.name.endsWith(".exe") || entry.name.endsWith(".msi")) &&
      !entry.name.includes("uninst")
    ) {
      results.push(fullPath);
    }
  }
  return results;
}

function copyInstallers() {
  const installers = findInstallers(bundleDir);

  if (installers.length === 0) {
    console.log(
      `\x1b[33m[!] No installer found in: ${path.relative(rootDir, bundleDir)}\x1b[0m\n` +
      `    Run "npm run build:app" or "tauri build" first.`
    );
    return;
  }

  if (!fs.existsSync(outDir)) {
    fs.mkdirSync(outDir, { recursive: true });
  }

  console.log(`\x1b[32m[✓] Found ${installers.length} installer file(s). Copying to ./dist-installer/ ...\x1b[0m`);

  for (const file of installers) {
    const filename = path.basename(file);
    const dest = path.join(outDir, filename);
    fs.copyFileSync(file, dest);
    console.log(`    -> Copied: dist-installer/${filename}`);

    if (filename.endsWith("-setup.exe") || filename.includes("setup")) {
      const easySetup = path.join(outDir, "setup.exe");
      fs.copyFileSync(file, easySetup);
      console.log(`    -> Created easy alias: dist-installer/setup.exe`);
    }
  }

  console.log(`\n\x1b[36m[✓] All installer files are ready in:\x1b[0m ${outDir}`);
}

copyInstallers();

