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
  for (x of list) {
    let el = document.createElement('li');
    const content = jsonToHTML(x);
    el.appendChild(content);
    result.appendChild(el);
  }
  return result;
}

// Turn a Javascript object into a definition list element.
function objToDL(obj) {
  let result = document.createElement('dd');
  const keys = Object.keys(obj);
  for (key of keys) {
    let dt = document.createElement('dt');
    let dd = document.createElement('dd');
    dt.appendChild(document.createTextNode(key));
    const content = jsonToHTML(obj[key]);
    dd.appendChild(content);
    result.appendChild(dt);
    result.appendChild(dd);
  }
  return result;
}
