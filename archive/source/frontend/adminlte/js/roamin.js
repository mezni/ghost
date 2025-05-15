$(document).ready(function () {
  const API_BASE = 'http://localhost:3000/api/v1';
  const METRICS_ENDPOINT = '/metrics';

  const charts = {
    historical: null,
    pieRoamIn: null,
    pieRoamOut: null
  };

  function formatChartDate(dateString) {
    if (!dateString) return '';
    const date = new Date(dateString);
    return date.toLocaleDateString('en-US', { month: 'short', day: 'numeric' });
  }

  async function fetchData(url) {
    const response = await fetch(url);
    if (!response.ok) throw new Error(`Failed to fetch from ${url}`);
    const result = await response.json();
    return result.data || [];
  }

  async function fetchHistoryRoamInTotal() {
    return fetchData(`${API_BASE}${METRICS_ENDPOINT}?direction=totin&dimensions=global&kind=history`);
  }

  async function fetchHistoryRoamInActive() {
    return fetchData(`${API_BASE}${METRICS_ENDPOINT}?direction=actin&dimensions=global&kind=history`);
  }

  async function fetchHistoryRoamInByCountry() {
    return fetchData(`${API_BASE}${METRICS_ENDPOINT}?direction=totin&dimensions=country`);
  }

  async function fetchHistoryRoamInByOperator() {
    return fetchData(`${API_BASE}${METRICS_ENDPOINT}?direction=totin&dimensions=operator`);
  }

  async function initHistoricalChart() {
    try {
      const [total, active] = await Promise.all([
        fetchHistoryRoamInTotal(),
        fetchHistoryRoamInActive()
      ]);

      const labels = total.map(item => formatChartDate(item.date));
      const totalCounts = total.map(item => item.count);
      const activeCounts = active.map(item => item.count);

      const ctx = document.getElementById('historicalChart').getContext('2d');
      if (charts.historical) charts.historical.destroy();

      charts.historical = new Chart(ctx, {
        type: 'line',
        data: {
          labels: labels,
          datasets: [
            {
              label: 'Total Roam In',
              data: totalCounts,
              borderColor: 'rgba(78, 115, 223, 1)',
              backgroundColor: 'rgba(78, 115, 223, 0.1)',
              pointBackgroundColor: '#4e73df',
              fill: true
            },
            {
              label: 'Active Roam In',
              data: activeCounts,
              borderColor: 'rgba(54, 185, 204, 1)',
              backgroundColor: 'rgba(54, 185, 204, 0.1)',
              pointBackgroundColor: '#36b9cc',
              fill: true
            }
          ]
        },
        options: {
          responsive: true,
          maintainAspectRatio: false,
          scales: {
            y: {
              beginAtZero: false,
              ticks: {
                callback: value => value.toLocaleString()
              }
            },
            x: {
              grid: { display: false }
            }
          },
          plugins: {
            tooltip: {
              callbacks: {
                label: context => `${context.dataset.label}: ${context.raw.toLocaleString()}`
              }
            }
          }
        }
      });
    } catch (err) {
      console.error('Error initializing historical chart:', err);
    }
  }

  function aggregateByField(historyData, keyFn) {
    const aggregated = {};
    historyData.forEach(item => {
      const key = keyFn(item) || 'Unknown';
      aggregated[key] = (aggregated[key] || 0) + item.count;
    });

    return Object.entries(aggregated)
      .map(([label, count]) => ({ label, count }))
      .sort((a, b) => b.count - a.count)
      .slice(0, 10);
  }

  function renderPieChart(data, chartId, labelKey) {
    const ctx = document.getElementById(chartId).getContext('2d');
    if (!ctx) return;
  
    if (charts[chartId]) charts[chartId].destroy();
  
    charts[chartId] = new Chart(ctx, {
      type: 'doughnut',
      data: {
        labels: data.map(item => item[labelKey] || 'Unknown'),
        datasets: [{
          data: data.map(item => item.count),
          backgroundColor: [
            '#FF6384', '#36A2EB', '#FFCE56', '#FF9F40', '#4BC0C0',
            '#9966FF', '#FF9F80', '#B2FF59', '#FF4081', '#3F51B5'
          ]
        }]
      },
      options: {
        responsive: true,
        maintainAspectRatio: false,
        cutoutPercentage: 50, 
        legend: {
          display: true,
          position: 'right', // ✅ place it right
          fullWidth: false,  // ✅ prevent full top width
          labels: {
            boxWidth: 15,
            padding: 20
          }
        },
        layout: {
          padding: {
            left: 0,
            right: 50,  // ✅ space for legend
            top: 0,
            bottom: 0
          }
        },
        tooltips: {
          callbacks: {
            label: function(tooltipItem, data) {
              const label = data.labels[tooltipItem.index] || '';
              const value = data.datasets[0].data[tooltipItem.index] || 0;
              return `${label}: ${value.toLocaleString()}`;
            }
          }
        }
      }
    });
  }

  
  async function renderDoughnutCharts() {
    try {
      const [historyByCountry, historyByOperator] = await Promise.all([
        fetchHistoryRoamInByCountry(),
        fetchHistoryRoamInByOperator()
      ]);

      const topCountries = aggregateByField(historyByCountry, item => item.country);
      const topOperators = aggregateByField(historyByOperator, item => {
        if (item.operator && item.country) {
          return `${item.operator} (${item.country})`;
        } else if (item.operator) {
          return item.operator;
        } else {
          return 'Unknown';
        }
      });

      renderPieChart(topCountries, 'pieChartRoamIn', 'label');
      renderPieChart(topOperators, 'pieChartRoamOut', 'label');
    } catch (err) {
      console.error('Error rendering doughnut charts:', err);
    }
  }

  async function loadDashboard() {
    try {
      await initHistoricalChart();
      await renderDoughnutCharts();
    } catch (err) {
      console.error('Error loading dashboard:', err);
    }
  }

  loadDashboard();
  setInterval(loadDashboard, 300000); // Refresh every 5 minutes
});
