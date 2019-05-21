const ENTER_KEY_CODE = 13;

document.addEventListener('DOMContentLoaded', init, false);

function init() {

  let addr_nxt_month = document.getElementById('addr_nxt_month').value;
  let addr_prv_month = document.getElementById('addr_prv_month').value;
  let addr_nxt_day = document.getElementById('addr_nxt_day').value;
  let addr_prv_day = document.getElementById('addr_prv_day').value;

  let arrow_next_month = document.getElementById('arrow_next_month');

  arrow_next_month.addEventListener('click', function() {
      location.assign(addr_nxt_month);
  });

  let arrow_previous_month = document.getElementById('arrow_previous_month');

  arrow_previous_month.addEventListener('click', function() {
      location.assign(addr_prv_month);
  });

  let arrow_next_day = document.getElementById('arrow_next_day');

  arrow_next_day.addEventListener('click', function() {
      location.assign(addr_nxt_day);
  });

  let arrow_previous_day = document.getElementById('arrow_previous_day');

  arrow_previous_day.addEventListener('click', function() {
      location.assign(addr_prv_day);
  });


  let add_row_btn = document.getElementById('add_row_btn');

  let transactions_table = document.getElementById('transactions_table');
  
  let add_row_btn_row = document.getElementById('add_row_btn_row');


  add_row_btn.addEventListener('click', function() {
    let template = document.getElementById('new_row_template').innerHTML;
    console.log(template);
    let new_row = htmlToElement(template);    
    transactions_table.appendChild(new_row);
    transactions_table.appendChild(add_row_btn_row);

    initPost();
  });

  initPost();
}

// https://stackoverflow.com/questions/494143/creating-a-new-dom-element-from-an-html-string-using-built-in-dom-methods-or-pro
function htmlToElement(html) {
  var template = document.createElement('template');
  html = html.trim();
  template.innerHTML = html;
  return template.content.firstChild;
}

function enter_listener(e) {
  if (e.keyCode === ENTER_KEY_CODE) {
    post();
  }
}

// This function is called repeatedly for each new row added.
// To prevent multiple event handers per element, first they are removed, then added again.
// It could be written more elegantly, but it works.
function initPost() {
  let inputs = document.getElementsByTagName("input");
  Array.from(inputs).forEach(function(element) {
    element.removeEventListener('keydown', enter_listener)
    element.addEventListener('keydown', enter_listener);
  });
}

function post() {

  let table = document.getElementById('transactions_table');

  let rows = table.querySelectorAll('tr.input_row');

  let records = [];

  Array.from(rows).forEach(function(row) {

    let id = parseInt(row.id);

    let amount = parseFloat(row.querySelector('input[name=amount]').value);

    let description = row.querySelector('input[name=description]').value;

    let category = parseInt(row.querySelector('select[name=category]').value);

    let record = {
      id: id,
      amount: amount,
      description: description,
      category: category
    };

    records.push(record);
  });

  let year = parseInt(document.getElementById('year').value);
  let month = parseInt(document.getElementById('month').value);
  let day = parseInt(document.getElementById('day').value);

  let url = `/write-event/${year}/${month}/${day}`

  let xhr = new XMLHttpRequest();
  xhr.open('POST', url, true);

  xhr.send(JSON.stringify(records));

  xhr.onloadend = function () {
    let addr = `/${year}/${month}/${day}`;
    location.assign(addr);
  };
}