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

function testDrawCommand(exportsObj, handDrawGeometryUtils) {

  // Sample drawing points data (enhanced structure matching expected format)
  const drawPoints = [
    {
      ps: [
        [-170.45, 170.45, 0, 1],
        [-102.27, 170.45, 7, 0],
        [170.45, 170.45, 7, 1],
        [170.45, -170.45, 4, 1],
        [-170.45, -170.45, 5, 1],
        [-170.45, 170.45, 6, 1]
      ],
      x0: 169.17,
      y0: 156.62,
      z: 0.474,
      drawMode: 2,
      ang: 0,
      lineColor: 9,
      // Add potentially missing properties
      scale: 1.0,
      opacity: 1.0,
      width: 340.91,
      height: 340.91
    }
  ];

  const drawConfig = {
    txPointTime: 55,
    playTime: 0, // Add missing playTime property
    cnfValus: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3]
  };

  const features = {
    textStopTime: true,
    textDecimalTime: true,
    displayType: "00",
    showOutDoorTips: false,
    xyCnf: false,
    arbPlay: false,
    ilda: false,
    ttlAn: false,
    picsPlay: false,
    textUpDown: false,
    animationFix: false
  };



  const pointTime = "00"; // Fourth parameter for point timing

  // Use handDrawGeometryUtils.drawPs to properly flatten points like the real application
  let flatPoints;


 
  // Create a mock canvas context
  const mockCanvasContext = {
    clearRect: function () { },
    setLineWidth: function () { },
    setStrokeStyle: function () { },
    setLineDash: function () { },
    beginPath: function () { },
    moveTo: function () { },
    lineTo: function () { },
    closePath: function () { },
    stroke: function () { },
    fill: function () { },
    draw: function () { },
    rect: function () { },
    fillRect: function () { },
    setFillStyle: function () { },
    arc: function () { }
  };

  // Create canvas draw config matching the real application structure
  const canvasDrawConfig = {
    ctx: mockCanvasContext, // Mock canvas context
    w: 340.91,
    h: 340.91,
    draw_line_type: [20, 20], // Default line type from app
    colorSeg: [
      {
        color: [1, 2, 3, 4, 5, 6, 7],
        name: "Default Color Sequence"
      },
      {
        color: [1, 1, 1, 1, 1, 4, 4, 4, 4, 4],
        name: "Test Pattern"
      }
    ]
  };

  
  // Create global colors array that the drawing functions expect
  global.colors = ['black', 'red', 'green', 'blue', 'yellow', '#00FFFF', 'purple', 'white'];

  // Create selectLines entries for each drawing object
  const selectLines = drawPoints.map((drawObj, index) => ({
    sel: false,      // Selection flag
    mx0: 0,          // Movement offset X
    my0: 0,          // Movement offset Y  
    color: null      // Override color
  }));

  const selectionState = {
    selectRect: {
      x0: 0, y0: 0, z: 1, ang: 0,
      mx: 0, my: 0,
      width: 0, height: 0,
      left: 0, top: 0,
      lastAng: 0, startAng: 0
    },
    selectLines: selectLines,
    selectMode: false
  };


  flatPoints = handDrawGeometryUtils.drawPs(drawPoints, canvasDrawConfig, selectionState);
  const drawCommand = exportsObj.getDrawCmdStr(flatPoints, drawConfig, features, pointTime);
  console.log('Result of getDrawCmdStr:', drawCommand);
  const pointString = exportsObj.getDrawPointStr(drawPoints, drawConfig, features, -1, drawConfig.txPointTime);
  const cmdResult = exportsObj.drawPointStrToCmd(pointString, features);
  console.log('Result of drawPointStrToCmd:', cmdResult);
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
  if (handDrawGeometryUtilsModule && typeof handDrawGeometryUtilsModule === 'function') {
    const handDrawFakeExports = {};
    const handDrawFakeModule = { exports: handDrawFakeExports };
    try {
      handDrawGeometryUtilsModule(handDrawFakeExports, handDrawFakeModule, dependencyResolver, dependencyResolver);
      handDrawGeometryUtils = handDrawFakeModule.exports.exports || handDrawFakeModule.exports;
      console.log('HandDrawGeometryUtils loaded, keys:', Object.keys(handDrawGeometryUtils));
    } catch (err) {
      console.error('Error loading handDrawGeometryUtils:', err);
    }
  }

  try {
    // Pass arguments in correct order: exports, module, dependencyResolver, dependencyResolver
    targetModule(fakeExports, fakeModule, dependencyResolver, dependencyResolver);
    // Exported functions may be on fakeModule.exports.exports or fakeModule.exports
    const exported = fakeModule.exports.exports || fakeModule.exports;
    console.log('Module keys:', Object.keys(exported));

    testGetQueryCmd(exported);
    testShowCmd(exported);
    testDrawCommand(exported, handDrawGeometryUtils);

  } catch (err) {
    console.error('Error calling module function:', err);
  }
} else {
  console.error('Module not found or not a function:', moduleName);
}