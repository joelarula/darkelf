// drawPicArrayShapes.js
// Node.js script to render each shape in picArrayShapes.js as a chart and save as images in ./pics

const fs = require('fs');
const path = require('path');
const { createCanvas } = require('canvas');

// Load picArrayShapes.js (assume it exports { picArray })
const picArrayShapes = require('./picArrayShapes.js');
const picArray = picArrayShapes.picArray;

const outputDir = path.join(__dirname, 'pics');
const width = 900;
const height = 900;
const margin = 40;

function getBounds(points) {
  let minX = Infinity, minY = Infinity, maxX = -Infinity, maxY = -Infinity;
  for (const pt of points) {
    minX = Math.min(minX, pt[0]);
    minY = Math.min(minY, pt[1]);
    maxX = Math.max(maxX, pt[0]);
    maxY = Math.max(maxY, pt[1]);
  }
  return { minX, minY, maxX, maxY };
}

function drawShape(points, outPath, idx) {
  const canvas = createCanvas(width, height);
  const ctx = canvas.getContext('2d');
  ctx.fillStyle = '#fff';
  ctx.fillRect(0, 0, width, height);
  ctx.strokeStyle = '#0074D9';
  ctx.lineWidth = 3;
  ctx.setLineDash([]);

  // Compute bounds and scaling
  const { minX, minY, maxX, maxY } = getBounds(points);
  const scale = Math.min(
    (width - 2 * margin) / (maxX - minX || 1),
    (height - 2 * margin) / (maxY - minY || 1)
  );
  const offsetX = margin - minX * scale + (width - (maxX - minX) * scale) / 2 - margin;
  const offsetY = margin - minY * scale + (height - (maxY - minY) * scale) / 2 - margin;

  ctx.beginPath();
  for (let i = 0; i < points.length; i++) {
    const [x, y] = points[i];
    const px = x * scale + offsetX;
    const py = height - (y * scale + offsetY); // y axis up
    if (i === 0) ctx.moveTo(px, py);
    else ctx.lineTo(px, py);
  }
  ctx.stroke();

  // Draw points
  ctx.fillStyle = '#FF4136';
  for (let i = 0; i < points.length; i++) {
    const [x, y] = points[i];
    const px = x * scale + offsetX;
    const py = height - (y * scale + offsetY);
    ctx.beginPath();
    ctx.arc(px, py, 7, 0, 2 * Math.PI);
    ctx.fill();
  }

  // Save image
  const outFile = path.join(outPath, `shape_${idx + 1}.png`);
  const outStream = fs.createWriteStream(outFile);
  const stream = canvas.createPNGStream();
  stream.pipe(outStream);
  outStream.on('finish', () => {
    console.log(`Saved: ${outFile}`);
  });
}

// Main
if (!fs.existsSync(outputDir)) fs.mkdirSync(outputDir);
picArray.forEach((shape, idx) => {
  drawShape(shape, outputDir, idx);
});
