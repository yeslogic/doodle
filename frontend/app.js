import { valueToHTML } from './display.js';

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

main()
