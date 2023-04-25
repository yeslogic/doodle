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
    node = listToUL(json);
  } else if (typeof json === 'object') {
    node = objToDL(json);
  } else {
    node = document.createTextNode(json);
  }
  return node;
}

// Turn a Javascript list into an unordered list element.
function listToUL(list) {
  let result = document.createElement('ul');
  for (el of list) {
    let li = document.createElement('li');
    li.classList.add(typeof el);
    const content = jsonToHTML(el);
    li.appendChild(content);
    result.appendChild(li);
  }
  return result;
}

function recordToHTML(list) {
  let result = document.createElement('ul');
  for (el of list) {
    let li = document.createElement('li');
    li.classList.add(typeof el);
    const content = fieldToHTML(el);
    li.appendChild(content);
    result.appendChild(li);
  }
  return result;
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
    if (name === "identifier" && ("Seq" in value)) {
        // FIXME JPEG APP1 identifier
        valueContent = renderASCII(value["Seq"]);
    } else if ((name === "signature" || name === "tag") && ("Tuple" in value)) {
        // FIXME PNG signature and tags
        valueContent = renderASCII(value["Tuple"]);
    } else {
        valueContent = jsonToHTML(value);
    }
    liValue.appendChild(valueContent);

    ul.appendChild(liName);
    ul.appendChild(liValue);
    return ul;
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
