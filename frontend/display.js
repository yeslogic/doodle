function main() {
  const structureSection = document.getElementById('structure');
  Promise.all([
    fetch('./test.json').then(r => r.json()),
    fetch('./format.json').then(r => r.json()),
  ]).then(([value, format]) => {
    // TODO: format-directed printing
    structureSection.appendChild(valueToHTML(value));
  });
}

// Convert a value into HTML.
//
// FIXME: somehow this modifies the Json object passed to it.
function valueToHTML(value) {
  switch (value.tag) {
    case 'Bool':
    case 'U8':
    case 'U16':
    case 'U32':
      return document.createTextNode(value.data);
    case 'Record':
      return recordToHTML(value.data);
    case 'Variant':
      return recordToHTML([value.data]);
    case 'Seq':
    case 'Tuple':
      return seqToHTML(value.data);
    default:
      // NOTE: Should never happen!
      return document.createTextNode(value.data);
  }
}

function seqToHTML(items) {
  if (isRecordSeq(items)) {
    const fields = items[0].data.map(([label, value]) => {
      return [label, value.tag];
    });
    return renderSeqTable(items, fields);
  } else {
    const ul = document.createElement('ul');
    for (const item of items) {
      const li = document.createElement('li');
      li.classList.add(item.tag);
      ul.appendChild(li);
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
    const ul = document.createElement('ul');
    for (const [label, value] of fields) {
      const li = document.createElement('li');
      ul.appendChild(li);
      const content = fieldToHTML(label, value);
      li.appendChild(content);
    }
    return ul;
  }
}

function isRecordSeq(items) {
  return items.length > 0 && items[0].tag === 'Record' && isFlatRecord(items[0].data);
}

function isFlatRecord(fields) {
  return fields.every(([label, value]) => {
    return isAtomicValue(value) || getFieldASCII(label, value) !== null;
  });
}

function isAtomicValue(value) {
  return ['Bool', 'U8', 'U16', 'U32'].includes(value.tag);
}

function getFieldASCII(label, value) {
  if (label === 'identifier' && value.tag === 'Seq') {
    // JPEG APP1 identifier
    return value.data;
  } else if ((label === 'signature' || label === 'tag') && value.tag === 'Tuple') {
    // PNG signature and tags
    return value.data;
  } else if (label === 'tag' && value.tag === 'Variant' && value.data[1].tag === 'Tuple') {
    // more PNG tags
    return value.data[1].data;
  } else if (label === 'version' && value.tag === 'Seq') {
    // GIF 89a version
    return value.data;
  } else {
    return null;
  }
}

function fieldToHTML(label, value) {
  const ul = document.createElement('ul');
  const liLabel = document.createElement('li');
  const liValue = document.createElement('li');
  liLabel.classList.add('label');
  liValue.classList.add(value.tag);

  const nameContent = document.createTextNode(label);
  liLabel.appendChild(nameContent);
  const valueContent = fieldValueToHTML(label, value);
  liValue.appendChild(valueContent);

  ul.appendChild(liLabel);
  ul.appendChild(liValue);
  return ul;
}

function fieldValueToHTML(label, value) {
  const ascii = getFieldASCII(label, value);
  if (ascii === null) {
    return valueToHTML(value);
  } else {
    return renderASCII(value.data);
  }
}

function renderRecordTable(fields) {
  const table = document.createElement('table');
  for (const [label, value] of fields) {
    const tr = document.createElement('tr');
    table.appendChild(tr);
    const th = document.createElement('th');
    tr.appendChild(th);
    th.textContent = label;
    const td = document.createElement('td');
    tr.appendChild(td);
    const valueContent = fieldValueToHTML(label, value);
    td.appendChild(valueContent);
  }
  return table;
}

function renderSeqTable(items, fields) {
  const table = document.createElement('table');
  const tr = document.createElement('tr');
  table.appendChild(tr);
  for (const [label, type] of fields) {
    const th = document.createElement('th');
    tr.appendChild(th);
    th.textContent = label + ' : ' + type;
  }
  for (const item of items) {
    if (item.tag === 'Record') {
      const tr = document.createElement('tr');
      table.appendChild(tr);
      for (const [_, value] of item.data) {
        const td = document.createElement('td');
        tr.appendChild(td);
        const content = document.createTextNode(value.data);
        td.appendChild(content);
      }
    }
  }
  return table;
}

function renderASCII(items) {
  const escapes = {
    0x00: '0',
    0x09: 't',
    0x0A: 'n',
    0x0D: 'r',
  };
  const span = document.createElement('span');
  span.className = 'text';
  let run = null;
  for (const item of items) {
    if (item.tag === 'U8') {
      const b = item.data;
      let type, text;
      if (b >= 0x20 && b < 0x7F) {
        type = 'printable';
        text = String.fromCharCode(b);
      } else if (b in escapes) {
        type = 'escape';
        text = '\\' + escapes[b];
      } else {
        type = 'control';
        text = '\\x' + b.toString(16).padStart(2, '0');
      }
      if (!run || run.className !== type) {
        run = document.createElement('span');
        run.className = type;
        span.appendChild(run);
      }
      run.textContent += text;
    }
  }
  return span;
}
