const fs = require('fs');
const vm = require('vm');
const path = require('path');

// Load the webpack bundle
const bundlePath = path.join(__dirname, '../','refactor target', 'app-service-minimal.js');
const code = fs.readFileSync(bundlePath, 'utf8');


// Prepare a sandboxed environment
const sandbox = {
  globalThis: {},
  console,
  require,
  exports: {},
  module: {},
};
sandbox.globalThis.webpackJsonp = [];
vm.createContext(sandbox);

// Execute the bundle code in the sandbox
vm.runInContext(code, sandbox);

const webpackJsonp = sandbox.globalThis.webpackJsonp;
if (!webpackJsonp || !Array.isArray(webpackJsonp) || webpackJsonp.length === 0) {
  console.error("webpackJsonp not found or empty after running bundle.");
  process.exit(1);
}
const modulesObj = webpackJsonp[webpackJsonp.length - 1][1];

// Example: Call a function from a module
const moduleName = "fontGeometryUtils "; // Note the space if present in the bundle
const targetModule = modulesObj[moduleName];

if (targetModule) {
  // If it's a function, call it with a fake exports object to get the exports
  let exportsObj = targetModule;
  if (typeof targetModule === 'function') {
    try {
      const fakeExports = {};
      targetModule(fakeExports);
      exportsObj = fakeExports;
    } catch (err) {
      console.error("Error calling module function:", err);
    }
  }
  if (exportsObj && exportsObj.exports && typeof exportsObj.exports.ifHasChinese === 'function') {
    const result = exportsObj.exports.ifHasChinese("汉字");
    console.log("Result of ifHasChinese:", result);
  } else {
    console.error("ifHasChinese function not found in module exports.");
    console.log("Module keys:", Object.keys(exportsObj));
    console.log("Module exports:", exportsObj.exports);
  }
} else {
  console.error("Module not found:", moduleName);
}