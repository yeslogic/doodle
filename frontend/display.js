function main() {
  const structureSection = document.getElementById('structure');
  fetch('./test.json')
    .then(r => r.json())
    .then(json => {
      structureSection.appendChild(valueToHTML(json));
    });
}

// Convert a value into HTML.
//
// FIXME: somehow this modifies the Json object passed to it.
function valueToHTML(value) {
  let result = document.createElement('dl');

  let dt = document.createElement('dt');
  let dd = document.createElement('dd');

  dt.classList.add(typeof value.data);
  dd.classList.add(typeof value.data);

  dt.appendChild(document.createTextNode(value.tag));
  result.appendChild(dt);

  switch (value.tag) {
    case "Bool":
    case "U8":
    case "U16":
    case "U32":
      dd.appendChild(document.createTextNode(value.data));
      break;
    case "Record":
      dd.appendChild(recordToHTML(value.data));
      break;
    case "Variant":
      dd.appendChild(recordToHTML([value.data]));
      break;
    case "Seq":
    case "Tuple":
      dd.appendChild(seqToHTML(value.data));
      break;
    default:
      // NOTE: Should never happen!
      dd.appendChild(document.createTextNode(value.data));
      break;
  }

  result.appendChild(dd);

  return result;
}

function seqToHTML(seq) {
  if (isRecordSeq(seq)) {
    let fields = [];
    for (let [name, value] of seq[0].data) {
      fields.push([name, getAtomicType(value)]);
    }
    return renderSeqTable(seq, fields);
  } else {
    let ul = document.createElement('ul');
    for (item of seq) {
      let li = document.createElement('li');
      ul.appendChild(li);
      li.classList.add(typeof item);
      const content = valueToHTML(item);
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
  return seq.length > 0 && seq[0].tag === "Record" && isFlatRecord(seq[0].data);
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
    if (value.tag === type) return type;
  }
  return null;
}

function getFieldASCII(name, value) {
  if (name === "identifier" && value.tag === "Seq") {
    // JPEG APP1 identifier
    return value.data;
  } else if ((name === "signature" || name === "tag") && value.tag === "Tuple") {
    // PNG signature and tags
    return value.data;
  } else if (name === "tag" && value.tag === "Variant" && value.data[1].tag === "Tuple") {
    // more PNG tags
    return value.data[1].data;
  } else if (name === "version" && value.tag === "Seq") {
    // GIF 89a version
    return value.data;
  } else {
    return null;
  }
}

function fieldToHTML([name, value]) {
  let ul = document.createElement('ul');
  let liName = document.createElement('li');
  let liValue = document.createElement('li');
  liName.classList.add(typeof name);
  liValue.classList.add(typeof value);

  const nameContent = document.createTextNode(name);
  liName.appendChild(nameContent);

  let valueContent;
  let valueASCII = getFieldASCII(name, value);
  if (valueASCII !== null) {
    valueContent = renderASCII(valueASCII);
  } else {
    valueContent = valueToHTML(value);
  }
  liValue.appendChild(valueContent);

  ul.appendChild(liName);
  ul.appendChild(liValue);
  return ul;
}

function renderRecordTable(record) {
  let table = document.createElement("table");
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
      content = valueToHTML(value);
    }
    td.appendChild(content);
  }
  return table;
}

function renderSeqTable(seq, fields) {
  let table = document.createElement("table");
  let tr = document.createElement("tr");
  table.appendChild(tr);
  for (let [name, type] of fields) {
    let th = document.createElement("th");
    tr.appendChild(th);
    th.textContent = name + " : " + type;
  }
  for (let item of seq) {
    if (item.tag === "Record") {
      let tr = document.createElement("tr");
      table.appendChild(tr);
      for (let [_, value] of item.data) {
        let td = document.createElement("td");
        tr.appendChild(td);
        let content = document.createTextNode(value.data);
        td.appendChild(content);
      }
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
    if (item.tag === "U8") {
      let b = item.data;
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
  }
  return span;
}
