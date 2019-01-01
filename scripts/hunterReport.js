var iconv = require('iconv-lite');
// var fs = require('fs');

function myFunction() {
  txtField = document.getElementById("combinedText");
  txtField.innerHTML = txtField.innerHTML + "grass";
}

function loadDescriptionFileHelper() {
  // console.log("fdfdfdfdf");
  var fileName =  "HunterWyniki.txt";
  txtField = document.getElementById("combinedText");

var txt;
  fetch(fileName)
  .then(response => response.text())
  // .then((text) => return text)
     // {

    // console.log(txt)
    // txt = text
    // txtField.innerHTML = txt
  // })
  // .then(response => response.text())
  // .then(converted => console.log(converted))
  // .then(arrayBuffer => iconv.decode(new Buffer(arrayBuffer), 'iso-8859-1').toString())
  // .then(converted => console.log(converted))
// txtField.innerHTML = txt
 // console.log(txt)
  // fetch(fileName)
  // .then(response => response.text())
  // .then(text => txtField.innerHTML = text)
  // .then(text => console.log(text))

}

async function loadDescriptionFile(){
  // var txt = loadDescriptionFileHelper()
  var fileName =  "HunterWyniki.txt";
  const response = await fetch(fileName)
  txt = await response.text()
  txt = txt.split('>>')
  txt.splice(0,1)


  console.log(txt)
  txtField = document.getElementById("combinedText");
  txtField.innerHTML = txt;
}
