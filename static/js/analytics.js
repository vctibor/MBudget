document.addEventListener('DOMContentLoaded', init, false);

function init() {

  var trace1 = {
    x: [1, 2, 3, 4],
    y: [10, 15, 13, 17],
    type: 'scatter',
  };

  var trace2 = {
    x: [1, 2, 3, 4],
    y: [16, 5, 11, 9],
    type: 'scatter'
  };

  var data = [trace1, trace2];

  Plotly.newPlot('my_div', data, {
    paper_bgcolor: 'rgba(0,0,0,0)',
    plot_bgcolor: 'rgba(0,0,0,0)'
  });

}