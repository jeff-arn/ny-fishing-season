const MDCTextField = mdc.textField.MDCTextField;

// initializes the MDC handlers for the component
const node = document.querySelector('.mdc-text-field');
const textField = new MDCTextField(node);
textField.leadingIconAriaLabel = 'search icon';

node.addEventListener('input', (e) => {
  textFieldInput = e.target.value;
  const cleansedTextInput = textFieldInput.toLowerCase().trim();
  filterFish(cleansedTextInput);
  filterSection('in-season-limited');
  filterSection('in-season-all-year');
  filterSection('upcoming-seasons');
});

function filterSection(id) {
  const sectionNode = document.getElementById(id);
  const cardNodes = [...sectionNode.lastElementChild.children];

  // if all cards are hidden
  if (cardNodes.every((n) => n.classList.contains('filter--hidden') || n.classList.contains('no-season-placeholder'))) {
    if (!sectionNode.classList.contains('filter--hidden')) {
      sectionNode.classList.add('filter--hidden');
    }
  } else {
    if (sectionNode.classList.contains('filter--hidden')) {
      sectionNode.classList.remove('filter--hidden');
    }
  }
}

function filterFish(filterText) {
  const cardTitleNodes = document.querySelectorAll('.card__title');

  cardTitleNodes.forEach((cardTitleNode) => {
    const cardTitle = cardTitleNode.innerHTML.toLowerCase();
    const cardNode = cardTitleNode.parentElement.parentElement;

    if (!cardTitle.includes(filterText)) {
      if (!cardNode.classList.contains('filter--hidden')) {
        cardNode.classList.add('filter--hidden');
      }
    } else {
      const cardNode = cardTitleNode.parentElement.parentElement;
      if (cardNode.classList.contains('filter--hidden')) {
        cardNode.classList.remove('filter--hidden');
      }
    }
  });
}