function main() {
  const structureSection = document.getElementById('structure');
  let testJson;
  fetch('./test.json')
    .then(r => r.json())
    .then(json => {
      structureSection.appendChild(jsonToHTML(json));
    });
}

// Convert some parsed Json into HTML.
//
// FIXME: somehow this modifies the Json object passed to it.
function jsonToHTML(json) {
  let node;
  if (Array.isArray(json)) {
    node = seqToHTML(json);
  } else if (typeof json === 'object') {
    node = objToDL(json);
  } else {
    node = document.createTextNode(json);
  }
  return node;
}

function seqToHTML(seq) {
    if (isRecordSeq(seq)) {
        let fields = [];
        for (let [name, value] of seq[0]["Record"]) {
            fields.push([name, getAtomicType(value)]);
        }
        return renderSeqTable(seq, fields);
    } else {
        let ul = document.createElement('ul');
        for (item of seq) {
            let li = document.createElement('li');
            ul.appendChild(li);
            li.classList.add(typeof item);
            const content = jsonToHTML(item);
            li.appendChild(content);
        }
        return ul;
    }
}

function recordToHTML(fields) {
    if (isFlatRecord(fields)) {
        return renderRecordTable(fields);
    } else {
        let ul = document.createElement('ul');
        for (field of fields) {
            let li = document.createElement('li');
            ul.appendChild(li);
            li.classList.add(typeof field);
            const content = fieldToHTML(field);
            li.appendChild(content);
        }
        return ul;
    }
}

function isRecordSeq(seq) {
    return seq.length > 0 && (typeof seq[0] === "object") && ("Record" in seq[0]) && isFlatRecord(seq[0]["Record"]);
}

function isFlatRecord(fields) {
    for (let [name, value] of fields) {
        if (!isAtomicValue(value) && getFieldASCII(name, value) === null) {
            return false;
        }
    }
    return true;
}

function isAtomicValue(value) {
    return getAtomicType(value) !== null;
}

function getAtomicType(value) {
    const atomicTypes = ["U8", "U16", "U32"];
    for (const type of atomicTypes) {
        if (type in value) return type;
    }
    return null;
}

function getFieldASCII(name, value) {
    if (name === "identifier" && ("Seq" in value)) {
        // JPEG APP1 identifier
        return value["Seq"];
    } else if ((name === "signature" || name === "tag") && ("Tuple" in value)) {
        // PNG signature and tags
        return value["Tuple"];
    } else if (name === "tag" && ("Variant" in value) && ("Tuple" in value["Variant"][1])) {
        // more PNG tags
        return value["Variant"][1]["Tuple"];
    } else if (name === "version" && ("Seq" in value)) {
        // GIF 89a version
        return value["Seq"];
    }
    else {
        return null;
    }
}

// Turn a Javascript object into a definition list element.
function objToDL(obj) {
  let result = document.createElement('dl');
  const keys = Object.keys(obj);
  for (key of keys) {
    let dt = document.createElement('dt');
    let dd = document.createElement('dd');

    dt.classList.add(typeof obj[key]);
    dd.classList.add(typeof obj[key]);

    dt.appendChild(document.createTextNode(key));
    result.appendChild(dt);

    if (key === "Record") {
        const content = recordToHTML(obj[key]);
        dd.appendChild(content);
    } else if (key === "Seq") {
        const content = seqToHTML(obj[key]);
        dd.appendChild(content);
    } else {
        const content = jsonToHTML(obj[key]);
        dd.appendChild(content);
    }

    result.appendChild(dd);
  }
  return result;
}

function fieldToHTML([name, value]) {
    let ul = document.createElement('ul');
    let liName = document.createElement('li');
    let liValue = document.createElement('li');
    liName.classList.add(typeof name);
    liValue.classList.add(typeof value);

    const nameContent = jsonToHTML(name);
    liName.appendChild(nameContent);

    let valueContent;
    let valueASCII = getFieldASCII(name, value);
    if (valueASCII !== null) {
        valueContent = renderASCII(valueASCII);
    } else {
        valueContent = jsonToHTML(value);
    }
    liValue.appendChild(valueContent);

    ul.appendChild(liName);
    ul.appendChild(liValue);
    return ul;
}

function renderRecordTable(record) {
    let table = document.createElement("table");
    table.border = 1;
    for (let [name, value] of record) {
        let tr = document.createElement("tr");
        table.appendChild(tr);
        let th = document.createElement("th");
        tr.appendChild(th);
        th.textContent = name;
        let td = document.createElement("td");
        tr.appendChild(td);
        let content;
        let valueASCII = getFieldASCII(name, value);
        if (valueASCII !== null) {
            content = renderASCII(valueASCII);
        } else {
            content = jsonToHTML(value);
        }
        td.appendChild(content);
    }
    return table;
}

function renderSeqTable(seq, fields) {
    let table = document.createElement("table");
    table.border = 1;
    let tr = document.createElement("tr");
    table.appendChild(tr);
    let map = {};
    for (let [name, type] of fields) {
        let th = document.createElement("th");
        tr.appendChild(th);
        th.textContent = name + " : " + type;
        map[name] = type;
    }
    for (let item of seq) {
        let record = item["Record"];
        let tr = document.createElement("tr");
        table.appendChild(tr);
        for (let [name, value] of record) {
            let type = map[name];
            let td = document.createElement("td");
            tr.appendChild(td);
            let content = jsonToHTML(value[type]);
            td.appendChild(content);
        }
    }
    return table;
}

function renderASCII(seq) {
    const escapes = {
        0x00: '0',
        0x09: 't',
        0x0A: 'n',
        0x0D: 'r',
    };
    let span = document.createElement("span");
    span.className = "text";
    let run = null;
    for (let item of seq) {
        let b = item["U8"];
        let type, text;
        if (b >= 0x20 && b < 0x7F) {
            type = "printable";
            text = String.fromCharCode(b);
        } else if (b in escapes) {
            type = "escape";
            text = "\\" + escapes[b];
        } else {
            type = "control";
            text = "\\x" + b.toString(16).padStart(2, '0');
        }
        if (!run || run.className !== type) {
            run = document.createElement("span");
            run.className = type;
            span.appendChild(run);
        }
        run.textContent += text;
    }
    return span;
}
