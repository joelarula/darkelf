const express = require('express');
const path = require('path');

const app = express();
const PORT = process.env.PORT || 3000;

// Serve static files from the directory containing __uniappview.html
const staticDir = path.join(__dirname, 'lightelf', 'app', 'src', 'main', 'assets', 'apps', '__UNI__2C82991', 'www');
app.use(express.static(staticDir));

// Fallback to __uniappview.html for SPA routing
app.get('*', (req, res) => {
  res.sendFile(path.join(staticDir, '__uniappview.html'));
});

app.listen(PORT, () => {
  console.log(`Server running at http://localhost:${PORT}`);
});
