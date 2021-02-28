const MDCTextField = mdc.textField.MDCTextField;
const MDCRipple = mdc.ripple.MDCRipple;

// initialize the submit date button
const submitButtonNode = document.getElementById('date-submit-button');
const buttonRipple = new MDCRipple(submitButtonNode);
submitButtonNode.disabled = true;

// initializes the MDC handlers for the date selection text input
const dateNode = document.getElementById('date-text-entry');
const dateTextField = new MDCTextField(dateNode);

let savedDate = null;

dateNode.addEventListener('change', (e) => {
  // Format YYYY-MM-DD
  const enteredDate = e.target.value;

  if (enteredDate !== savedDate && enteredDate !== '') {
    submitButtonNode.disabled = false;
    savedDate = enteredDate;
  } else {
    submitButtonNode.disabled = true;
  }
});

submitButtonNode.addEventListener('click', () => {
  if (!savedDate) {
    return;
  }

  const [year, month, day] = savedDate.split('-');

  window.location.assign(`${window.location.origin}/${year}/${month}/${day}`);
});

// initializes the MDC handlers for the component
const searchNode = document.getElementById('fish-text-entry');
const searchTextField = new MDCTextField(searchNode);
searchTextField.leadingIconAriaLabel = 'search icon';

let textFieldInput = '';

searchNode.addEventListener('input', (e) => {
  textFieldInput = e.target.value;
  const cleansedTextInput = textFieldInput.toLowerCase().trim();
  filterFish(cleansedTextInput);
  filterSection('in-season');
  filterSection('out-of-season');
});

function filterSection(id) {
  const sectionNode = document.getElementById(id);
  const rowNodes = [
    ...document.querySelectorAll(
      `#${id} > .mdc-data-table > .mdc-data-table__table-container > .mdc-data-table__table > .mdc-data-table__content > .mdc-data-table__row`
    ),
  ];

  // if all cards are hidden
  if (rowNodes.every((n) => n.classList.contains('filter--hidden'))) {
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
  const fishNameNodes = document.querySelectorAll('.name-row');

  fishNameNodes.forEach((fishNameNode) => {
    const fishName = fishNameNode.innerHTML.toLowerCase();
    const rowNode = fishNameNode.parentElement;

    if (!fishName.includes(filterText)) {
      if (!rowNode.classList.contains('filter--hidden')) {
        rowNode.classList.add('filter--hidden');
      }
    } else {
      if (rowNode.classList.contains('filter--hidden')) {
        rowNode.classList.remove('filter--hidden');
      }
    }
  });
}
