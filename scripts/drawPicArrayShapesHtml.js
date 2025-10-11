// drawPicArrayShapesHtml.js
// Node.js script to render each shape in picArrayShapes.js as an HTML chart
// and save each as a separate HTML file in ./pics

const fs = require('fs');
const path = require('path');

// Load picArrayShapes.js (assume it exports { picArray })
const picArray = require('./picArrayShapes.json');

const outputDir = path.join(__dirname, '../pics');
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

function shapeToSvg(points, idx) {
  const { minX, minY, maxX, maxY } = getBounds(points);
  const scale = Math.min(
    (width - 2 * margin) / (maxX - minX || 1),
    (height - 2 * margin) / (maxY - minY || 1)
  );
  const offsetX = margin - minX * scale + (width - (maxX - minX) * scale) / 2 - margin;
  const offsetY = margin - minY * scale + (height - (maxY - minY) * scale) / 2 - margin;

  let pathData = '';
  points.forEach(([x, y], i) => {
    const px = x * scale + offsetX;
    const py = height - (y * scale + offsetY);
    pathData += (i === 0 ? 'M' : 'L') + px.toFixed(2) + ',' + py.toFixed(2) + ' ';
  });

  // Draw points as circles
  let circles = points.map(([x, y], i) => {
    const px = x * scale + offsetX;
    const py = height - (y * scale + offsetY);
    return `<circle cx="${px.toFixed(2)}" cy="${py.toFixed(2)}" r="7" fill="#FF4136" />`;
  }).join('\n');

  return `<svg width="${width}" height="${height}" style="background:#fff">
    <path d="${pathData}" stroke="#0074D9" stroke-width="3" fill="none" />
    ${circles}
  </svg>`;
}

function shapeHtml(points, idx) {
  return `<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <title>Shape ${idx + 1}</title>
  <style>
    body { background: #f8f8f8; text-align: center; }
    .container { margin: 30px auto; width: ${width}px; }
    h1 { font-family: sans-serif; }
  </style>
</head>
<body>
  <div class="container">
    <h1>Shape ${idx + 1}</h1>
    ${shapeToSvg(points, idx)}
  </div>
</body>
</html>`;
}

if (!fs.existsSync(outputDir)) fs.mkdirSync(outputDir);
picArray.forEach((shape, idx) => {
  const html = shapeHtml(shape, idx);
  const outFile = path.join(outputDir, `shape_${idx + 1}.html`);
  fs.writeFileSync(outFile, html, 'utf8');
  console.log(`Saved: ${outFile}`);
});
