// var iconv = require('iconv-lite');
var $ = require('jquery');

window.loadDescriptionFile = loadDescriptionFile

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

}

function parseDescriptionFile(text) {
  text = text.split('>>')
  text.splice(0,1)
  console.log(text.length);
  
  for (i =0; i<text.length; i++){
    text1 = text[i].split('^')
    text1.splice(0,1)
    console.log(">>>"+text1);
    idName = text[i].split('^')[0]
    divToAdd = '<div id=type'+ i +' class=boxes>'+idName
    $('#descriptionsTable').append(divToAdd)
    for (j =0; j<text1.length; j++){
      itemToAdd = text1[j].split('$')[0]
      labelToAdd = '<label for=box'+j+'><input type=checkbox name=radioGroup>'+itemToAdd+'</label>'
      console.log(labelToAdd);
      
      $('#type'+i).append(labelToAdd)
    }
    $('#descriptionsTable').append('</div><br>')
    

  }
  // console.log(">>>"+text);
}

