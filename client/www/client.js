var json = {};

var txtRaw = document.getElementById("txtRaw");

function renderAlert(ts, headline, description, instruction) {
    let alert_div = document.getElementById("alerts");
    var alert_html = `
    <div class="alert alert-warning alert-dismissible fade show" role="alert">
        <i><b>[ TIME ] </b></i> <strong>HEADLINE</strong> DESCRIPTION
        <i><p>INSTRUCTION</p></i>
        <button type="button" class="close" data-dismiss="alert" aria-label="Close">
            <span aria-hidden="true">&times;</span>
        </button>
    </div>
    `;
    var s = new Date(ts * 1000 ).toLocaleString("de-DE");
    console.log(ts);
    console.log(s);
    alert_html = alert_html.replace("HEADLINE", headline).replace("DESCRIPTION", description).replace("INSTRUCTION", instruction).replace("TIME", s);
    alert_div.innerHTML = alert_html + alert_div.innerHTML;
    alert_count.innerHTML = warn_history.size;
}


var warn_history = new Set();
var alert_count = document.getElementById("alertCount");
var last = document.getElementById("last");

function handleBundle(warnings_json) {
    console.log("new bundle");
    console.log(warnings_json);
    let warns_all = JSON.parse(warnings_json);
    let warns = warns_all["warnings"];
    let last_ts = warns_all.time;
    var s = new Date(last_ts).toLocaleString("de-DE");
    last.innerHTML = s;
    Object.keys(warns).forEach((key, index) => {
        warns[key].forEach(function(item, index) {
            console.log(item);
            let warn_key = key + item.regionName + item.start + item.headline;
            if (!warn_history.has(warn_key)) {
                txtRaw.innerHTML += JSON.stringify(item, null, 2) + "\n";
                txtRaw.scrollTop = txtRaw.scrollHeight;
                console.log(warn_key);
                warn_history.add(warn_key);
                renderAlert(item.start, item.headline, item.description, item.instruction);
            }
        });
    });
}
function btnTestClicked() {
    console.log("test");
    var ts = Math. round((new Date()). getTime() / 1000);
    renderAlert(ts, "TIMER", "all good here", "just stay home");
}

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
        .catch(err => {
            throw err
        });
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
    external.invoke(JSON.stringify(pub_data));
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

    if (!('999999999' in json.warnings)) {
        json.warnings['999999999'] = [];
    }
    json.warnings['999999999'].push(warn);

    //  txtRaw.value = JSON.stringify(json, null, 2);  
    btnUpdateTimeClicked();
    description.value = '';
    instruction.value = '';
    headline.value = '';
}

function modalCloseClicked() {
    modal.style.display = "none";
}
// When the user clicks anywhere outside of the modal, close it
window.onclick = function(event) {
    if (event.target == modal) {
        modal.style.display = "none";
    }
}
