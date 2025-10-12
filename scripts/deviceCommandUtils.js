// deviceCommandUtils.js - refactored to match webpack module pattern

module.exports = function (exports, require, module, dependencyResolver) {
  // Dependency resolution
  var arrayConversionHelper = dependencyResolver('arrayConversionHelper');
  var enhancedConsoleLogger = dependencyResolver('enhancedConsoleLogger').default;

  // All original logic goes here, using arrayConversionHelper and enhancedConsoleLogger as needed
  // Example export:
  exports.test = function (e) {
    return "hello---" + e;
  };
  // ...add all other exports here, following the structure from your example...
};

// ...existing code...
// Utility function to test getCmdStr export
function testShowCmd(exportsObj) {

  const commandConfig = {
    curMode: 6, // Change mode (e.g., 2)
    textData: {
      txColor: 5,      // Change color (e.g., 5)
      txSize: 50,     // Font size (default 100)
      runSpeed: 50,    // Run speed (default 80)
      txDist: 50,      // Text distance (default 50)
      runDir: 1,       // Run direction (default 1)
      txPointTime: 10, // Point time (default 10)
    },
    prjData: {
      public: {
        rdMode: 1,     // Audio trigger mode (e.g., 1)
        soundVal: 77, // Sound sensitivity (e.g., 120)
      },
      prjItem: {
        2: {
          pyMode: 128,         // Change pyMode (e.g., 128)
          prjSelected: [21845, 21845, 21845, 1], // Selection bits (default)
        },
        3: {
          pyMode: 128,
          prjSelected: [1, 0, 0, 2],
        },
        5: {
          pyMode: 128,
          prjSelected: [0, 0, 0, 0],
        },
        6: {
          pyMode: 128,
          prjSelected: [65535, 65535, 65535, 3],
        }
      }
    }
  };

  if (typeof exportsObj.getCmdStr === 'function') {
    const result = exportsObj.getCmdStr(commandConfig, {});
    console.log('Result of getCmdStr:', result);
  } else {
    console.error('getCmdStr function not found in module exports.');
  }
}



function testGetQueryCmd(exportsObj) {

  var randomCheck = [];
  for (var e = 0; e < 4; e++) randomCheck[e] = Math.floor(256 * Math.random());
  console.log('Random check array:', randomCheck);

  if (typeof exportsObj.getQueryCmd === 'function') {
    const result = exportsObj.getQueryCmd(randomCheck);
    console.log('Result of getQueryCmd:', result);
  } else {
    console.error('getQueryCmd function not found in module exports.');
  }
}

function testPolylineCommand(exportsObj, handDrawGeometryUtils) {
  

  const data = JSON.parse(fs.readFileSync(path.join(__dirname, 'lill.json'), 'utf8'));
  testDrawCommandUtil(data,exportsObj, handDrawGeometryUtils);

}
function testDrawCommand(exportsObj, handDrawGeometryUtils) {

  const data = JSON.parse(fs.readFileSync(path.join(__dirname, 'ruut.json'), 'utf8'));
  testDrawCommandUtil(data,exportsObj, handDrawGeometryUtils);
}


function testDrawCommandUtil(data,exportsObj, handDrawGeometryUtils) {

  // Load ruut.json data (UTF-8 version)
  const ruutData = JSON.parse(fs.readFileSync(path.join(__dirname, 'ruut.json'), 'utf8'));

  // Load drawing points data from ruut.json
  const drawPoints = data.data.drawPoints;

  // Use drawConfig from ruut.json
  const drawConfig = data.data.pisObj;

  // Use features from ruut.json
  const features = data.data.features ;

  const pointTime = "00"; // Fourth parameter for point timing

  let flatPoints2 = handDrawGeometryUtils.drawPs2(drawPoints, 300);
  console.log('flatPoints2:', flatPoints2);

  const drawCommand = exportsObj.getDrawCmdStr(flatPoints2, drawConfig, features, pointTime);
  console.log('Result of getDrawCmdStr:', drawCommand);
  console.log('  drawCommand length:', drawCommand ? drawCommand.length : 'null');



}


const fs = require('fs');
const vm = require('vm');
const path = require('path');


// Load the webpack bundle
const bundlePath = path.join(__dirname, '../', 'refactor target', 'app-service-minimal.js');
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

// Debug: Check available modules
console.log('Available modules:', Object.keys(modulesObj).filter(k => k.includes('handDraw') || k.includes('deviceCommand')));

const moduleName = "deviceCommandUtils "; // Note the space if present in the bundle
const targetModule = modulesObj[moduleName];

// Also load handDrawGeometryUtils for drawPs function
const handDrawGeometryUtilsName = "handDrawGeometryUtils"; // No trailing space
const handDrawGeometryUtilsModule = modulesObj[handDrawGeometryUtilsName];

// Dependency resolver for webpack modules
function dependencyResolver(name) {
  if (name === "arrayConversionHelper") {
    try {
      return require('./arrayConversionHelper'); // Adjust path if needed
    } catch (e) {
      return function (arr) { return arr; }; // Fallback stub
    }
  }
  if (name === "enhancedConsoleLogger") {
    return { default: console };
  }
  if (name === "spreadToArrayHelper") {
    // Fallback implementation for spreadToArrayHelper
    return function (arr, count) {
      if (Array.isArray(arr)) {
        return count ? arr.slice(0, count) : arr;
      }
      return [];
    };
  }

  // Try to load from webpack modules
  if (modulesObj && modulesObj[name]) {
    const targetModule = modulesObj[name];
    if (typeof targetModule === 'function') {
      const fakeExports = {};
      const fakeModule = { exports: fakeExports };
      try {
        targetModule(fakeExports, fakeModule, dependencyResolver, dependencyResolver);
        return fakeModule.exports.exports || fakeModule.exports;
      } catch (e) {
        console.warn('Failed to load webpack module:', name, e.message);
        return {};
      }
    }
  }

  // Return stub for unknown dependencies
  console.warn('Unknown dependency, returning stub:', name);
  return {};
}

// For direct execution/testing:
if (targetModule && typeof targetModule === 'function') {
  const fakeExports = {};
  const fakeModule = { exports: fakeExports };

  // Setup handDrawGeometryUtils
  let handDrawGeometryUtils = null;
  const handDrawFakeExports = {};
  const handDrawFakeModule = { exports: handDrawFakeExports };
  
  handDrawGeometryUtilsModule(handDrawFakeExports, handDrawFakeModule, dependencyResolver, dependencyResolver);
  handDrawGeometryUtils = handDrawFakeModule.exports.exports || handDrawFakeModule.exports;
 

  // Pass arguments in correct order: exports, module, dependencyResolver, dependencyResolver
  targetModule(fakeExports, fakeModule, dependencyResolver, dependencyResolver);
  // Exported functions may be on fakeModule.exports.exports or fakeModule.exports
  const exported = fakeModule.exports.exports || fakeModule.exports;

  testGetQueryCmd(exported);
  testShowCmd(exported);
  //testDrawCommand(exported, handDrawGeometryUtils);
  testPolylineCommand(exported, handDrawGeometryUtils);

} else {
  console.error('Module not found or not a function:', moduleName);
}