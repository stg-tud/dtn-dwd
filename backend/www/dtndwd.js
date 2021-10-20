function sendMessageToServer(cmd) {
  if (window.external !== undefined) {
    return window.external.invoke(cmd);
  } else if (window.webkit.messageHandlers.external !== undefined) {
    return window.webkit.messageHandlers.external.postMessage(cmd);
  }
  throw new Error('Failed to locate webkit external handler')
}

var json = {};

var txtRaw = document.getElementById("txtRaw");
function btnDownloadClicked() {
  console.log("test");
  let url = 'https://www.dwd.de/DWD/warnungen/warnapp/json/warnings.json';

  fetch(url)
    .then(res => res.text())
    .then(res => {
      let jsonp = res
      let rawjson = jsonp.substring(24, jsonp.length - 2);
      json = JSON.parse(rawjson);
      txtRaw.value = JSON.stringify(json, null, 2);
    })
    .catch(err => { throw err });
}

function btnUpdateTimeClicked() {
  var ts = Math.round((new Date()).getTime() / 1000) * 1000;
  console.log(ts);
  console.log(json.time);
  json.time = ts;

  txtRaw.value = JSON.stringify(json, null, 2);
}


var modal = document.getElementById("myModal");
function btnNewWarningClicked() {
  //  alert("unimplemented!");
  modal.style.display = "block";
}
function btnPublishClicked() {
  //  alert("unimplemented!");
  var pub_data = {
    "cmd": "publish",
    "data": JSON.stringify(json)
  };
  sendMessageToServer(JSON.stringify(pub_data));
}

function btnAddNewWarningClicked() {
  modal.style.display = "none";

  var description = document.getElementById("txtDescription");
  var headline = document.getElementById("txtHeadline");
  var instruction = document.getElementById("txtInstruction");

  var ts = Math.round((new Date()).getTime() / 1000) * 1000;
  var warn = {
    "regionName": "TESTLAND",
    "start": ts,
    "end": null,
    "state": "Test",
    "type": 1,
    "level": 2,
    "description": description.value,
    "event": "TEST-WARNUNG",
    "headline": headline.value,
    "instruction": instruction.value,
    "stateShort": "TS",
    "altitudeStart": null,
    "altitudeEnd": null
  };
  console.log(warn);

  if (json.warnings == null) {
    json.warnings = {};
  }
  if (!('999999999' in json.warnings)) {
    json.warnings['999999999'] = [];
  }
  json.warnings['999999999'].push(warn);

  //  txtRaw.value = JSON.stringify(json, null, 2);  
  btnUpdateTimeClicked();
  description.value = '';
  instruction.value = '';
  headline.value = '';

  // Scroll text area to bottom to show new entry
  txtRaw.scrollTop = txtRaw.scrollHeight;
}
function modalCloseClicked() {
  modal.style.display = "none";
}
// When the user clicks anywhere outside of the modal, close it
window.onclick = function (event) {
  if (event.target == modal) {
    modal.style.display = "none";
  }
}
