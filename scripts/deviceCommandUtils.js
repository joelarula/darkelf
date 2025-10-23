// deviceCommandUtils.js - refactored to match webpack module pattern

module.exports = function (exports, require, module, dependencyResolver) {
  // Dependency resolution
  var arrayConversionHelper = dependencyResolver('arrayConversionHelper');
  var enhancedConsoleLogger = dependencyResolver('enhancedConsoleLogger').default;
  var textLineVectorizer = dependencyResolver('textLineVectorizer ');
  var fontGeometryUtils = dependencyResolver('fontGeometryUtils ');
  var codePointAt = dependencyResolver('codePointAt');

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

  // Use pisObj from ruut.json
  const pisObj = data.data.pisObj;

  // Use features from ruut.json
  const features = data.data.features ;

  const pointTime = "00"; // Fourth parameter for point timing

  // Create mock canvas context for drawPs function
  const mockContext = {
    clearRect: function() {},
    setLineWidth: function() {},
    beginPath: function() {},
    moveTo: function() {},
    lineTo: function() {},
    stroke: function() {},
    arc: function() {},
    fill: function() {},
    setStrokeStyle: function() {},
    setFillStyle: function() {},
    setLineDash: function() {},
    closePath: function() {},
    rect: function() {},
    save: function() {},
    restore: function() {},
    translate: function() {},
    rotate: function() {},
    scale: function() {},
    clip: function() {},
    createLinearGradient: function() { return { addColorStop: function() {} }; },
    createRadialGradient: function() { return { addColorStop: function() {} }; }
  };

  // Create drawConfig for drawPs function
  const drawConfig = {
    ctx: mockContext,
    w: 300,
    h: 300,
    draw_line_type: 0,
    colorSeg: [
      { color: [1, 2, 3, 4, 5, 6, 7] } // Mock color segment
    ]
  };

  // Create selectionState for drawPs function  
  const selectionState = {
    selectRect: null,
    selectLines: [],
    selectMode: false
  };

  let flatPoints1 = handDrawGeometryUtils.drawPs(drawPoints, drawConfig, selectionState);
  let flatPoints2 = handDrawGeometryUtils.drawPs2(drawPoints, 300);
  console.log('Number of points:', flatPoints2.length);
  
  // Show first few points for analysis
  console.log('First 5 points:');
  flatPoints2.slice(0, 5).forEach((point, idx) => {
    console.log(`  Point ${idx}: [${point[0].toFixed(2)}, ${point[1].toFixed(2)}, ${point[2]}, ${point[3]}]`);
  });

  const drawCommand = exportsObj.getDrawCmdStr(flatPoints2, pisObj, features, pointTime);
  console.log('\nGenerated draw command:');
  console.log('  Full command:', drawCommand);
  console.log('  Command length:', drawCommand ? drawCommand.length : 'null');
  
  // Analyze command structure
  if (drawCommand && drawCommand.length > 16) {
    console.log('\nCommand structure analysis:');
    console.log('  Header:', drawCommand.substring(0, 8));
    console.log('  Config section (first 32 chars):', drawCommand.substring(8, 40));
    console.log('  Point count (next 4 chars):', drawCommand.substring(40, 44));
    console.log('  Point data starts at char:', 44);
    console.log('  Footer:', drawCommand.substring(drawCommand.length - 8));
    
    // Calculate bytes per point
    const headerFooterLength = 16; // F0F1F2F3...F4F5F6F7 = 8+8 chars
    const configLength = 32; // Config section length
    const pointCountLength = 4; // Point count length
    const pointDataLength = drawCommand.length - headerFooterLength - configLength - pointCountLength;
    const bytesPerPoint = pointDataLength / flatPoints2.length / 2; // /2 because 2 hex chars = 1 byte
    
    console.log(`\nPoint encoding analysis:`);
    console.log(`  Total command length: ${drawCommand.length} hex characters`);
    console.log(`  Header + Footer: ${headerFooterLength} chars`);
    console.log(`  Config section: ${configLength} chars`);
    console.log(`  Point count: ${pointCountLength} chars`);
    console.log(`  Point data: ${pointDataLength} chars`);
    console.log(`  Number of points: ${flatPoints2.length}`);
    console.log(`  Bytes per point: ${bytesPerPoint.toFixed(1)}`);
    console.log(`  Hex chars per point: ${(pointDataLength / flatPoints2.length).toFixed(1)}`);
    
    // Show first few point encodings
    console.log('\nFirst 3 point encodings:');
    const pointDataStart = 44;
    for (let i = 0; i < Math.min(3, flatPoints2.length); i++) {
      const pointHexStart = pointDataStart + (i * 10); // 10 hex chars per point
      const pointHex = drawCommand.substring(pointHexStart, pointHexStart + 10);
      const point = flatPoints2[i];
      
      console.log(`  Point ${i}:`);
      console.log(`    Coordinates: [${point[0].toFixed(2)}, ${point[1].toFixed(2)}, ${point[2]}, ${point[3]}]`);
      console.log(`    Hex encoding: ${pointHex}`);
      
      if (pointHex.length >= 10) {
        const xHex = pointHex.substring(0, 4);
        const yHex = pointHex.substring(4, 8);
        const colorPenHex = pointHex.substring(8, 10);
        
        // Decode values
        const xDecoded = parseInt(xHex, 16);
        const yDecoded = parseInt(yHex, 16);
        const colorPenDecoded = parseInt(colorPenHex, 16);
        const colorDecoded = (colorPenDecoded >> 4) & 0xF;
        const penDecoded = colorPenDecoded & 0xF;
        
        console.log(`    X: ${xHex} = ${xDecoded} (original: ${Math.round(point[0])})`);
        console.log(`    Y: ${yHex} = ${yDecoded} (original: ${Math.round(point[1])})`);
        console.log(`    Color/Pen: ${colorPenHex} = ${colorPenDecoded} (color: ${colorDecoded}, pen: ${penDecoded})`);
      }
    }
  }



}

function testTextCommand(exports,textLineVectorizer,fontGeometryUtils,codePointAt) {

const fs = require('fs');
const path = require('path');
const opentype = require('opentype.js');

  const latinWoffPath = path.join(__dirname, 'Roboto-Bold.ttf');
  opentype.load(latinWoffPath, function(err, loadedFontOpentype) {
    if (err) {
      console.log(err);
    } else {
  
    var fontData = {
      data: loadedFontOpentype, // or just fontBuffer if that's expected
      mode: 1,
      sn: 1002
    };

    var text = "ABC123";   
    console.log("Testing text:", text);

    var textLines = textLineVectorizer.getTextLines(loadedFontOpentype, text);
    console.log(textLines);
    const textLinesPath = path.join(__dirname, 'textLines.json');
    fs.writeFileSync(textLinesPath, JSON.stringify(textLines, null, 2), 'utf8');
    console.log('textLines written to', textLinesPath);


    var textCoordinates = textLineVectorizer.getXXYY(opentype, fontData, text, true);
    console.log(textCoordinates.xxyy);

    const outputPath = path.join(__dirname, 'textCoordinates.json');
    fs.writeFileSync(outputPath, JSON.stringify(textCoordinates, null, 2), 'utf8');
    console.log('textCoordinates written to', outputPath);
  
    var testTextData = {
    verTag: 0,
    runDir: 0,
    arrColor: ["red", "green", "blue", "yellow", "#00FFFF", "purple", "white"],
    txPointTime: 50,
    txColor: 9,
    txSize: 50,
    txDist: 50,
    runSpeed: 50,
    groupIdex: 0,
    groupList: [
      {
        text: text,
        update: 0,
        color: 9,
        fontIdex: 0,
        time: 5,
        xys: textCoordinates.xxyy,
        XysRight: textCoordinates.xxyyRight || [],
        XysUp: textCoordinates.xxyyUp || [],
        XysDown: textCoordinates.xxyyDown || []
      }
    ]
  };

  if (typeof exports.getXysCmdArr === 'function') {
    const result = exports.getXysCmdArr(
      testTextData.groupList,
      {},
      testTextData.runDir,
      testTextData.verTag
    );
    console.log('Result of getXysCmdArr:', result);

      const getXysCmdArrPath = path.join(__dirname, 'getXysCmdArr.txt');
    fs.writeFileSync(getXysCmdArrPath, JSON.stringify(result, null, 2), 'utf8');
    console.log('getXysCmdArr result written to', getXysCmdArrPath);


  } else {
    console.error('getXysCmdArr function not found in module exports.');
  }


    }
  });

}
  


const fs = require('fs');
const vm = require('vm');
const path = require('path');



// Load the webpack bundle
const bundlePath = path.join(__dirname,  'app-service-minimal.js');
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


const fontGeometryUtilsModule = modulesObj['fontGeometryUtils'];
const fontGeometryUtilsExports = {};
if (typeof fontGeometryUtilsModule === 'function') {
  fontGeometryUtilsModule(fontGeometryUtilsExports, {}, dependencyResolver);
}
const fontGeometryUtils = fontGeometryUtilsExports;
console.log('fontGeometryUtils keys:', Object.keys(fontGeometryUtils.exports));

// Import required modules from modulesObj
const textLineVectorizerModule = modulesObj['textLineVectorizer'];
const textLineVectorizerExports = {};
if (typeof textLineVectorizerModule === 'function') {
  textLineVectorizerModule(textLineVectorizerExports, {}, dependencyResolver);
}
const textLineVectorizer = textLineVectorizerExports;
console.log('textLineVectorizer keys:', Object.keys(textLineVectorizer.exports));




const codePointAt = modulesObj['codePointAt'];

// Debug: Check available modules
console.log('Available modules:', Object.keys(modulesObj));

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
  if (name === "fontGeometryUtils") {
    const fontGeometryUtilsModule = modulesObj['fontGeometryUtils'];
    if (typeof fontGeometryUtilsModule === 'function') {
      const fontGeometryUtilsExports = {};
      fontGeometryUtilsModule(fontGeometryUtilsExports, {}, dependencyResolver);
      return fontGeometryUtilsExports;
    }
    return fontGeometryUtilsModule;
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
    return targetModule;
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

  //testGetQueryCmd(exported);
  //testShowCmd(exported);
  //testDrawCommand(exported, handDrawGeometryUtils);
  //testPolylineCommand(exported, handDrawGeometryUtils);




  testTextCommand(exported,textLineVectorizer.exports,fontGeometryUtils.exports,codePointAt);

} else {
  console.error('Module not found or not a function:', moduleName);
}