// Originally inspired by https://github.com/abuseofnotation/vanilla-fp

const createElement = (tagName) => ({ className }, children) => {
  const element = document.createElement(tagName);

  if (className) {
    element.classList.add(className);
  }

  if (children) {
    if (typeof children === 'string') {
      const textNode = document.createTextNode(children);
      element.appendChild(textNode);
    } else {
      element.replaceChildren(...children);
    }
  }

  return element;
};

export const dd = createElement('dd');
export const div = createElement('div');
export const dl = createElement('dl');
export const dt = createElement('dt');
export const li = createElement('li');
export const span = createElement('span');
export const table = createElement('table');
export const td = createElement('td');
export const th = createElement('th');
export const tr = createElement('tr');
export const ul = createElement('ul');
