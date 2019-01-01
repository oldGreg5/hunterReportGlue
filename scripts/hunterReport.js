var iconv = require('iconv-lite');
var fs = require('fs');

function myFunction() {
  txtField = document.getElementById("combinedText");
  txtField.innerHTML = txtField.innerHTML + "grass";
}

function loadDescriptionFile() {
  console.log("fdfdfdfdf");
  var fileName =  "HunterWyniki.txt";
  txtField = document.getElementById("combinedText");

  fetch(fileName)
  .then(response => response.arrayBuffer())
  .then(arrayBuffer => iconv.decode(new Buffer(arrayBuffer), 'iso-8859-1').toString())
  // .then(converted => console.log(converted))

  // fetch(fileName)
  // .then(response => response.text())
  // .then(text => txtField.innerHTML = text)
  // .then(text => console.log(text))

}
