var http = require('http');
var fs = require('fs');
var path = require('path');

// Log the startup information
console.log('Starting Hunter Report Generator server...');
console.log('Current directory:', process.cwd());
console.log('Available files:', fs.readdirSync('.').join(', '));

try {
  console.log('Scripts directory contains:', fs.readdirSync('./scripts').join(', '));
} catch (err) {
  console.log('Warning: Could not read scripts directory:', err.message);
}

try {
  console.log('Data directory contains:', fs.readdirSync('./data').join(', '));
} catch (err) {
  console.log('Warning: Could not read data directory:', err.message);
}

http.createServer(function (req, res) {
        console.log('Received request for:', req.url);
        requestCss(req, res);
        requestJs(req, res);
        requestHome(req, res);
        requestDescription(req, res);
}).listen(8089, function() {
  console.log('Server running at http://localhost:8089/hunterReport');
});

function requestCss(req,res) {
  if (req.url.match("hunterReport.css")){
    var cssPath = path.join(process.cwd(), 'hunterReport.css');
    console.log('Reading CSS file from:', cssPath);
    fs.readFile(cssPath, function(err, data) {
      res.writeHead(200, {'Content-Type': 'text/css'});
      if (err) {
        console.error("Error reading CSS file:", err);
        res.end("/* CSS file not found */");
        return;
      }
      res.write(data);
      res.end();
    });
  }
}
function requestJs(req,res) {
  if (req.url.match("bundle.js")){
    var jsPath = path.join(process.cwd(), 'scripts', 'bundle.js');
    console.log('Reading JS file from:', jsPath);
    fs.readFile(jsPath, function(err, data) {
      res.writeHead(200, {'Content-Type': 'text/javascript'});
      if (err) {
        console.error("Error reading JS file:", err);
        res.end("console.error('Bundle file not found');");
        return;
      }
      res.write(data);
      res.end();
    });
  }
}
function requestHome(req,res) {
  if (req.url.match("hunterReport$")){
    var htmlPath = path.join(process.cwd(), 'hunterReport.html');
    console.log('Reading HTML file from:', htmlPath);
    fs.readFile(htmlPath, function(err, data) {
      res.writeHead(200, {'Content-Type': 'text/html'});
      if (err) {
        console.error("Error reading HTML file:", err);
        res.end("<html><body><h1>Error loading page</h1></body></html>");
        return;
      }
      res.write(data);
      res.end();
    });
  }
}
function requestDescription(req,res) {
  if (req.url.match("HunterWyniki.txt")){
    var dataPath = path.join(process.cwd(), 'data', 'HunterWyniki.txt');
    console.log('Reading data file from:', dataPath);
    fs.readFile(dataPath, function(err, data) {
      res.writeHead(200, {'Content-Type': 'text'});
      if (err) {
        console.error("Error reading data file:", err);
        res.end(""); // Send empty string instead of undefined
        return;
      }
      res.write(data);
      res.end();
    });
  }
}
