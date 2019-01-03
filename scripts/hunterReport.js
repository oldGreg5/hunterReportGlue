var iconv = require('iconv-lite');
var $ = require('jquery');

function myFunction() {
  txtField = document.getElementById("combinedText");
  txtField.innerHTML = txtField.innerHTML + "grass";
}

function loadDescriptionFileHelper() {
  var fileName =  "HunterWyniki.txt";
  txtField = document.getElementById("combinedText");

var txt;
  fetch(fileName)
  .then(response => response.text())

  // .then(response => response.text()
  // .then(converted => console.log(converted))
  // .then(arrayBuffer => iconv.decode(new Buffer(arrayBuffer), 'iso-8859-1').toString())
  // .then(converted => console.log(converted))

}

async function loadDescriptionFile(){
  // var txt = loadDescriptionFileHelper()
  var fileName =  "HunterWyniki.txt";
  const response = await fetch(fileName)
  txt = await response.text()
  txt = parseDescriptionFile(txt)


  console.log(txt)
  txtField = document.getElementById("combinedText");
  txtField.innerHTML = txt;
}

function parseDescriptionFile(text) {
  text = text.split('>>')
  text.splice(0,1)
  console.log(text.length);
   

}