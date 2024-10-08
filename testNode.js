var http = require('http');
var fs = require('fs');

http.createServer(function (req, res) {
        requestCss(req, res);
        requestJs(req, res);
        requestHome(req, res);
        requestDescription(req, res);
}).listen(8089);

function requestCss(req,res) {
  if (req.url.match("hunterReport.css")){
    fs.readFile('./hunterReport.css', function(err, data) {
      res.writeHead(200, {'Content-Type': 'text/css'});
      res.write(data);
      res.end();
    });
  }
}
function requestJs(req,res) {
  if (req.url.match("bundle.js")){
    fs.readFile('./scripts/bundle.js', function(err, data) {
      res.writeHead(200, {'Content-Type': 'text/javascript'});
      res.write(data);
      res.end();
    });
  }
}
function requestHome(req,res) {
  if (req.url.match("hunterReport$")){
    fs.readFile('./hunterReport.html', function(err, data) {
      res.writeHead(200, {'Content-Type': 'text/html'});
      res.write(data);
      res.end();
    });
  }
}
function requestDescription(req,res) {
  if (req.url.match("HunterWyniki.txt")){
    fs.readFile('./data/HunterWyniki.txt', function(err, data) {
      res.writeHead(200, {'Content-Type': 'text'});
      res.write(data);
      res.end();
    });
  }
}
