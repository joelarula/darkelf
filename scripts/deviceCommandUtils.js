// deviceCommandUtils.js - refactored to match webpack module pattern

module.exports = function(exports, require, module, dependencyResolver) {
  // Dependency resolution
  var arrayConversionHelper = dependencyResolver('arrayConversionHelper');
  var enhancedConsoleLogger = dependencyResolver('enhancedConsoleLogger').default;

  // All original logic goes here, using arrayConversionHelper and enhancedConsoleLogger as needed
  // Example export:
  exports.test = function(e) {
    return "hello---" + e;
  };
  // ...add all other exports here, following the structure from your example...
};

// ...existing code...
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

const moduleName = "deviceCommandUtils "; // Note the space if present in the bundle
const targetModule = modulesObj[moduleName];

// Dependency resolver for webpack modules
function dependencyResolver(name) {
  if (name === "arrayConversionHelper") {
    try {
      return require('./arrayConversionHelper'); // Adjust path if needed
    } catch (e) {
      return function(arr) { return arr; }; // Fallback stub
    }
  }
  if (name === "enhancedConsoleLogger") {
    return { default: console };
  }
  throw new Error("Unknown dependency: " + name);
}

// For direct execution/testing:
if (targetModule && typeof targetModule === 'function') {
  const fakeExports = {};
  const fakeModule = { exports: fakeExports };
  try {
    // Pass arguments in correct order: exports, module, dependencyResolver, dependencyResolver
    targetModule(fakeExports, fakeModule, dependencyResolver, dependencyResolver);
    // Exported functions may be on fakeModule.exports.exports or fakeModule.exports
    const exported = fakeModule.exports.exports || fakeModule.exports;
    console.log('Module keys:', Object.keys(exported));
    if (typeof exported.getQueryCmd === 'function') {
      const result = exported.getQueryCmd([1,2,3,4]);
      console.log('Result of getQueryCmd:', result);
    } else {
      console.error('getQueryCmd function not found in module exports.');
    }
  } catch (err) {
    console.error('Error calling module function:', err);
  }
} else {
  console.error('Module not found or not a function:', moduleName);
}