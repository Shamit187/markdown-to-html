const http = require('http');
const fs = require('fs');
const path = require('path');

const server = http.createServer((req, res) => {
    // Set the content type to HTML
    res.writeHead(200, {'Content-Type': 'text/html'});

    // Read the index.html file and serve it
    fs.readFile(path.join(__dirname, 'index.html'), (err, data) => {
        if (err) {
            // If there's an error reading the file, send a 500 Internal Server Error response
            res.writeHead(500);
            return res.end('Error loading index.html');
        }
        // Send the contents of index.html to the client
        res.end(data);
    });
});

const PORT = process.env.PORT || 3000; // Use the provided port or default to 3000

server.listen(PORT, () => {
    console.log(`Server is running on port ${PORT}`);
});
