const ENTER_KEY_CODE = 13;

const NEW_ROW = '<tr class="expanses_row_odd"><td></td><td></td><td></td></tr>';

/*
window.onload = function(e) {
    alert("Hello, World!");
    let categorySelects = document.getElementsByClassName("category_select");
    // alert(categorySelects.length);

    for (categorySelect in categorySelects) {
        let option = document.createElement("option");
        option.text = "Kiwi";        
        categorySelect.add(option);
    }

    for (i = 0; i < categorySelects.length; i++) {
        categorySelects[i].style.backgroundColor = "red";
    }
}
*/

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
    let new_row = htmlToElement(NEW_ROW);
    transactions_table.appendChild(new_row);
    transactions_table.appendChild(add_row_btn_row);
  });
}

// https://stackoverflow.com/questions/494143/creating-a-new-dom-element-from-an-html-string-using-built-in-dom-methods-or-pro
function htmlToElement(html) {
    var template = document.createElement('template');
    html = html.trim(); // Never return a text node of whitespace as the result
    template.innerHTML = html;
    return template.content.firstChild;
}
/*


function init() {

  let year = parseInt(document.getElementById('year').value);
  let month = parseInt(document.getElementById('month').value);

  initNext(year, month);
  initPrevious(year, month);
  initWriteEvent(year, month);
}

function initNext(year, month) {

  let next_month = (month === 12) ? 1 : (month + 1);
  
  // Next year changes only if current month is December,
  //  otherwise 'next_year' is actually current year.
  let next_year = (month === 12) ? (year + 1) : year;
  let  addr_next = `/${next_year}/${next_month}`;

}

function initPrevious(year, month) {

  let previous_month = (month === 1) ? 12 : (month - 1);
  let previous_year = (month === 1) ? (year - 1) : year;
  let  addr_prev = `/${previous_year}/${previous_month}`;
  let arrow_previous = document.getElementById('arrow_previous');

  arrow_previous.addEventListener('click', function() {
    location.assign(addr_prev);
  });
}

function initWriteEvent(year, month) {

  let inputs = document.getElementsByTagName('input');
  
  Array.from(inputs).forEach(function(elem) {
    elem.addEventListener('keydown', function(e) {
      if (e.keyCode === ENTER_KEY_CODE) {

        let day = this.dataset.day;

        let event_value = this.value;

        let url = `/write-event/${year}/${month}/${day}`

        var xhr = new XMLHttpRequest();
        xhr.open('POST', url, true);

        xhr.send(event_value);

        xhr.onloadend = function () {
          let addr = `/${year}/${month}`;
          location.assign(addr);
        };

      }
    });
  });
}

*/