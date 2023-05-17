import * as dom from './dom.js';

// Convert a value into HTML.
//
// FIXME: somehow this modifies the Json object passed to it.
export function valueToHTML(value) {
  let valueChildren;
  switch (value.tag) {
    case 'Bool':
    case 'U8':
    case 'U16':
    case 'U32':
      valueChildren = `${value.data}`;
      break;
    case 'Record':
      valueChildren = [recordToHTML(value.data)];
      break;
    case 'Variant':
      valueChildren = [recordToHTML([value.data])];
      break;
    case 'Seq':
    case 'Tuple':
      valueChildren = [seqToHTML(value.data)];
      break;
    default:
      throw `unknown tag ${value.tag}`;
  }

  return dom.dl({}, [
    dom.dt({ className: typeof value.data }, value.tag),
    dom.dd({ className: typeof value.data }, valueChildren)
  ]);
}

function seqToHTML(items) {
  if (isRecordSeq(items)) {
    const fields = items[0].data.map(([name, value]) => [name, value.tag]);
    return renderSeqTable(items, fields);
  } else {
    return dom.ul({}, items.map(item =>
      dom.li({}, [valueToHTML(item)])
    ));
  }
}

function recordToHTML(fields) {
  if (isFlatRecord(fields)) {
    return renderRecordTable(fields);
  } else {
    return dom.ul({}, fields.map(([name, value]) =>
      dom.li({}, [fieldToHTML(name, value)])
    ));
  }
}

function isRecordSeq(items) {
  return items.length > 0 && items[0].tag === 'Record' && isFlatRecord(items[0].data);
}

function isFlatRecord(fields) {
  return fields.every(([name, value]) =>
    isAtomicValue(value) || getFieldASCII(name, value) !== null
  );
}

function isAtomicValue(value) {
  return ['Bool', 'U8', 'U16', 'U32'].includes(value.tag);
}

function getFieldASCII(name, value) {
  if (name === 'identifier' && value.tag === 'Seq') {
    // JPEG APP1 identifier
    return value.data;
  } else if ((name === 'signature' || name === 'tag') && value.tag === 'Tuple') {
    // PNG signature and tags
    return value.data;
  } else if (name === 'tag' && value.tag === 'Variant' && value.data[1].tag === 'Tuple') {
    // more PNG tags
    return value.data[1].data;
  } else if (name === 'version' && value.tag === 'Seq') {
    // GIF 89a version
    return value.data;
  } else {
    return null;
  }
}

function fieldToHTML(name, value) {
  return dom.ul({}, [
    dom.li({ className: typeof name }, name),
    dom.li({ className: typeof value }, [fieldValueToHTML(name, value)])
  ]);
}

function fieldValueToHTML(name, value) {
  const ascii = getFieldASCII(name, value);
  if (ascii === null) {
    return valueToHTML(value);
  } else {
    return renderASCII(value.data);
  }
}

function renderRecordTable(fields) {
  return dom.table({}, fields.map(([name, value]) =>
    dom.tr({}, [
      dom.th({}, name),
      dom.td({}, [fieldValueToHTML(name, value)])
    ])
  ));
}

function renderSeqTable(items, fields) {
  return dom.table({},
    [
      dom.tr({}, fields.map(([name, type]) =>
        dom.th({}, `${name} : ${type}`)
      ))
    ].concat(items.map(item => {
      if (item.tag === "Record") {
        return dom.tr({}, item.data.map(([_, value]) =>
          dom.td({}, `${value.data}`)
        ));
      }
    }))
  );
}

function renderASCII(items) {
  const escapes = {
    0x00: '0',
    0x09: 't',
    0x0A: 'n',
    0x0D: 'r',
  };
  const span = dom.span({ className: 'text' }, []);
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
        text = `\\${escapes[b]}`;
      } else {
        type = 'control';
        text = `\\x${b.toString(16).padStart(2, '0')}`;
      }
      if (!run || run.className !== type) {
        run = dom.span({ className: type }, []);
        span.appendChild(run);
      }
      run.textContent += text;
    }
  }
  return span;
}
