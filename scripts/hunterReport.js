// var iconv = require('iconv-lite');
var $ = require('jquery');

window.combineDescription = combineDescription
var desciptionPairs = new Map();

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

function combineDescription() {
  loadAndParseDescriptionFile()
  combineText()
}

function combineText(){
  var boxNodes = $('.propBox')
  for (i = 0; i < boxNodes.size; i++){
    if (boxNodes[i].checked) {
      prop = boxNodes[i].parentNode.innerHTML.split('>')[1]
      val = desciptionPairs.get(prop)
      gluedText = $('#combinedText')
      gluedText += prop + " - " + val
      $('#combinedText').html(gluedText)
    }
  }
}

async function loadAndParseDescriptionFile(){
  // var txt = loadDescriptionFileHelper()
  var fileName =  "HunterWyniki.txt";
  const response = await fetch(fileName)
  txt = await response.text()
  txt = parseDescriptionFile(txt)
}

function parseDescriptionFile(text) {
  text = text.split('>>')
  text.splice(0,1)
  // console.log(text.length);
  

  for (i =0; i<text.length; i++){
    text1 = text[i].split('^')
    text1.splice(0,1)
    // console.log(">>>"+text1);
    idName = text[i].split('^')[0]
    divToAdd = '<div id=type'+ i +' class=boxes>'+idName
    $('#descriptionsTable').append(divToAdd)
    for (j =0; j<text1.length; j++){
      keyToAdd = text1[j].split('$')[0]
      valueToAdd = text1[j].split('$')[1]
      desciptionPairs.set(keyToAdd, valueToAdd)
      labelToAdd = '<label for=box'+j+'><input type=checkbox class=propBox name=radioGroup>'+keyToAdd+'</label>'
      // console.log(labelToAdd);
      
      $('#type'+i).append(labelToAdd)
    }
    $('#descriptionsTable').append('</div><br>')
    
  }
  console.log(">> Pairs size:"+desciptionPairs.size);
  // console.log(">>>" + desciptionPairs.get("Lamblia Intestinalis"));
  console.log("box: " + $('.propBox')[2].parentNode.innerHTML.split('>')[1]);
  console.log('checked: '+ $('.propBox')[2].checked);
  
}

