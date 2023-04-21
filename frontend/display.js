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

    const content = jsonToHTML(obj[key]);

    dd.appendChild(content);
    result.appendChild(dd);
  }
  return result;
}
