var http = require('http');
var fs = require('fs');

http.createServer(function (req, res) {
        requestCss(req, res);
        requestJs(req, res);
        requestHome(req, res);
}).listen(8088);

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
  if (req.url.match("hunterReport.js")){
    fs.readFile('./hunterReport.js', function(err, data) {
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
