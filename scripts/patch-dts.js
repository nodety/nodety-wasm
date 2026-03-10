#!/usr/bin/env node
/**
 * Patches the generated nodety_js.d.ts to fix missing type references.
 * - Adds Unscoped type (Rust empty enum, no runtime representation)
 * - Fixes Nodety* type aliases to reference the actual exported classes
 */

const fs = require("fs");
const path = require("path");

const dtsPath = path.join(__dirname, "..", "pkg", "nodety_js.d.ts");
let content = fs.readFileSync(dtsPath, "utf8");

// Add Unscoped type after JsType (before first use)
content = content.replace(
  /(export type JsType = "Integer" \| "String" \| "Array";)/,
  '$1\n\n/** Type marker for unscoped expressions (no runtime representation). */\nexport type Unscoped = never;'
);

// Fix Nodety* aliases - these reference Rust type aliases that aren't exported
const aliasFixes = [
  ["NodetyTypeExpr", "TypeExpr"],
  ["NodetyTypeParameters", "TypeParameters"],
  ["NodetyTypeParameter", "TypeParameter"],
  ["NodetyNode", "Node"],
  ["NodetyNodeSignature", "NodeSignature"],
  ["NodetyPortTypes", "PortTypes"],
];

for (const [from, to] of aliasFixes) {
  content = content.replace(new RegExp(`\\b${from}\\b`, "g"), to);
}

fs.writeFileSync(dtsPath, content);
console.log("Patched nodety_js.d.ts");
