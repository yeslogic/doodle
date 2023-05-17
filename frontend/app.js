import { Context, valueToHTML } from './display.js';

function main() {
  const structureSection = document.getElementById('structure');
  Promise.all([
    fetch('./test.json').then(r => r.json()),
    fetch('./format.json').then(r => r.json()),
  ]).then(([value, module]) => {
    const context = new Context(module);

    // FIXME: extract to method
    const formatLevel = context.module.names.lastIndexOf('main');
    const format = context.module.formats[formatLevel];

    const valueContent = context.decodedValueToHTML(value, format);
    structureSection.appendChild(valueContent);
  });
}

main()
