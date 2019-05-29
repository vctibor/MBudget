document.addEventListener('DOMContentLoaded', init, false);

function init() {

  let url = '/analytics/data';

  fetch(url)
    .then(res => res.json())
    .then((data) => {
      plot(data);
    })
    .catch(err => { throw err });
}

function plot(data) {

  let trace = [{
    x: data.daily_expenses[0],
    y: data.daily_expenses[1],
    type: 'bar'
  }];
  
  Plotly.newPlot('daily_expenses_plot', trace, {paper_bgcolor: 'rgba(0,0,0,0)'});
}