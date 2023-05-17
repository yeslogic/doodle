import { Empty, evaluate, matches, bindPatternNames } from './semantics.js';

export class Context {
  constructor(module) {
    this.module = module;
    this.localNames = new Empty;
    this.localValues = new Empty;
  }

  lookupItemName(level) {
    return this.module.names[level];
  }

  lookupItemFormat(level) {
    return this.module.formats[level];
  }

  bindLocalDef(name, value) {
    let context = new Context(this.module);
    context.localNames = this.localNames.extend(name);
    context.localValues = this.localValues.extend(value);
    return context;
  }

  bindLocalParam(name) {
    return this.bindLocalDef(name, null /* FIXME: bind variable */);
  }

  bindPattern(pattern, value) {
    const localValues = matches(this.localValues, pattern, value);
    if (localValues) {
      let context = new Context(this.module);
      context.localNames = bindPatternNames(this.localNames, pattern);
      context.localValues = localValues;
      return context;
    } else {
      return null;
    }
  }

  decodedValueToHTML(value, format) {
    switch (format.tag) {
      case 'ItemVar': {
        const itemFormat = this.lookupItemFormat(format.data);
        return this.decodedValueToHTML(value, itemFormat);
      }
      case 'Fail': throw 'uninhabited format';
      case 'EndOfInput': return valueToHTML(value);
      case 'Byte': return valueToHTML(value);
      case 'Union': switch (value.tag) {
        case 'Variant': {
          const [label, variantValue] = value.data;
          for (const [branchLabel, branchFormat] of format.data) {
            return valueToHTML(value); // TODO: use format
          }
          throw `missing branch ${label} in union format`;
        }
        default:
          throw `expected variant, found ${value.tag}`;
      }
      case 'Tuple': switch (value.tag) {
        case 'Tuple': {
          const formats = format.data;
          const values = value.data;
          return valueToHTML(value); // TODO: use format
        }
        default:
          throw `expected tuple, found ${value.tag}`;
      }
      case 'Record': switch (value.tag) {
        case 'Record': {
          const formatFields = format.data;
          const valueFields = value.data;
          return valueToHTML(value); // TODO: use format
        }
        default:
          throw `expected record, found ${value.tag}`;
      }
      case 'Repeat': return valueToHTML(value); // TODO: use format
      case 'Repeat1': return valueToHTML(value); // TODO: use format
      case 'RepeatCount': return valueToHTML(value); // TODO: use format
      case 'RepeatUntil': return valueToHTML(value); // TODO: use format
      case 'Peek':
        return this.decodedValueToHTML(value, format.data);
      case 'Slice':
      case 'WithRelativeOffset': {
        const [_, innerFormat] = format.data;
        return this.decodedValueToHTML(value, innerFormat);
      }
      case 'Map': {
        const [expr, innerFormat] = format.data;
        const valueFormat = mappedFormat(expr, innerFormat);
        return valueFormat
          ? this.decodedValueToHTML(value, valueFormat)
          : valueToHTML(value);
      }
      case 'Match': {
        const [headExpr, branches] = format.data;
        const headValue = evaluate(this.localValues, headExpr);
        for (const [pattern, branchFormat] of branches) {
          const context = this.bindPattern(pattern, headValue);
          if (context) {
            return context.decodedValueToHTML(value, branchFormat);
          }
        }
        throw 'non-exhaustive patterns';
      }
      default:
        throw `unknown tag ${format.tag}`;
    }
  }
}

// Convert a value into HTML.
//
// FIXME: somehow this modifies the Json object passed to it.
export function valueToHTML(value) {
  const result = document.createElement('dl');

  const dt = document.createElement('dt');
  const dd = document.createElement('dd');

  dt.classList.add(typeof value.data);
  dd.classList.add(typeof value.data);

  dt.appendChild(document.createTextNode(value.tag));
  result.appendChild(dt);

  switch (value.tag) {
    case 'Bool':
    case 'U8':
    case 'U16':
    case 'U32':
      dd.appendChild(document.createTextNode(value.data));
      break;
    case 'Record':
      dd.appendChild(recordToHTML(value.data));
      break;
    case 'Variant':
      dd.appendChild(recordToHTML([value.data]));
      break;
    case 'Seq':
    case 'Tuple':
      dd.appendChild(seqToHTML(value.data));
      break;
    default:
      throw `unknown tag ${format.tag}`;
  }

  result.appendChild(dd);

  return result;
}

function seqToHTML(items) {
  if (isRecordSeq(items)) {
    const fields = items[0].data.map(([name, value]) => {
      return [name, value.tag];
    });
    return renderSeqTable(items, fields);
  } else {
    const ul = document.createElement('ul');
    for (const item of items) {
      const li = document.createElement('li');
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
    for (const [name, value] of fields) {
      const li = document.createElement('li');
      ul.appendChild(li);
      const content = fieldToHTML(name, value);
      li.appendChild(content);
    }
    return ul;
  }
}

function isRecordSeq(items) {
  return items.length > 0 && items[0].tag === 'Record' && isFlatRecord(items[0].data);
}

function isFlatRecord(fields) {
  return fields.every(([name, value]) => {
    return isAtomicValue(value) || getFieldASCII(name, value) !== null;
  });
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
  const ul = document.createElement('ul');
  const liName = document.createElement('li');
  const liValue = document.createElement('li');
  liName.classList.add(typeof name);
  liValue.classList.add(typeof value);

  const nameContent = document.createTextNode(name);
  liName.appendChild(nameContent);
  const valueContent = fieldValueToHTML(name, value);
  liValue.appendChild(valueContent);

  ul.appendChild(liName);
  ul.appendChild(liValue);
  return ul;
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
  const table = document.createElement('table');
  for (const [name, value] of fields) {
    const tr = document.createElement('tr');
    table.appendChild(tr);
    const th = document.createElement('th');
    tr.appendChild(th);
    th.textContent = name;
    const td = document.createElement('td');
    tr.appendChild(td);
    const valueContent = fieldValueToHTML(name, value);
    td.appendChild(valueContent);
  }
  return table;
}

function renderSeqTable(items, fields) {
  const table = document.createElement('table');
  const tr = document.createElement('tr');
  table.appendChild(tr);
  for (const [name, type] of fields) {
    const th = document.createElement('th');
    tr.appendChild(th);
    th.textContent = name + ' : ' + type;
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

function mappedFormat(expr, format) {
  switch (expr.tag) {
    case 'Var':
      return expr.data === 0 ? format : null;
    case 'RecordProj': {
      const [head, label] = expr.data;
      const headFormat = mappedFormat(head, format);
      if (headFormat && headFormat.tag === 'Record') {
        const [_, fieldFormat] = headFormat.data.find(([fieldLabel, _]) => fieldLabel == label);
        return fieldFormat;
      } else {
        return null;
      }
    }
    case 'TupleProj': {
      const [head, index] = expr.data;
      const headFormat = mappedFormat(head, format);
      if (headFormat && headFormat.tag === 'Tuple') {
        return headFormat.data[index];
      } else {
        return null;
      }
    }
    default:
      return null;
  }
}
