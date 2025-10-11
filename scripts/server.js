const express = require('express');
const path = require('path');
const app = express();

// Serve static files from the scripts directory (where the JS files are)
app.use(express.static(__dirname));
// Also serve files from parent directory for any additional resources
app.use(express.static(path.join(__dirname, '..')));

// Serve the main HTML page
app.get('/', (req, res) => {
    res.sendFile(path.join(__dirname, 'index.html'));
});

// Handle routes for the drawing app
app.get('/draw', (req, res) => {
    res.sendFile(path.join(__dirname, 'index.html'));
});

// Handle routes for the pattern editor
app.get('/pattern-editor', (req, res) => {
    res.sendFile(path.join(__dirname, 'index.html'));
});

// API endpoint to get app info
app.get('/api/info', (req, res) => {
    res.json({
        name: 'DarkElf App Service',
        version: '1.0.0',
        components: [
            'handDrawPageComponent',
            'scenePatternEditorPageComponent',
            'deviceCommandUtils',
            'bleDeviceControlUtils'
        ]
    });
});

const PORT = process.env.PORT || 3000;
app.listen(PORT, () => {
    console.log(`🚀 DarkElf App Service running on http://localhost:${PORT}`);
    console.log(`📁 Serving files from: ${__dirname} and ${path.join(__dirname, '..')}`);
    console.log(`🎨 Drawing app available at: http://localhost:${PORT}/draw`);
    console.log(`⚙️  Pattern editor at: http://localhost:${PORT}/pattern-editor`);
});