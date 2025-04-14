// Determine API base URL based on environment
const API_BASE =
  window.location.hostname === "localhost"
    ? "http://localhost:3000"
    : "http://api-service:3000"; // 'api-service' is the backend container name in Docker

// Fetch summary overview data
fetch(`${API_BASE}/api/v1/overview`)
  .then(response => response.json())
  .then(data => {
    const stats = data.data;

    // Update Last Load Date
    const lastDate = new Date(stats.last_date);
    const formattedDate = lastDate.toLocaleDateString(undefined, {
      year: 'numeric',
      month: 'long',
      day: 'numeric'
    });
    document.getElementById('last-load-date').textContent = formattedDate;

    // Update Stats Cards
    document.getElementById('roam-out').textContent = stats.count_roam_out;
    document.getElementById('roam-in').textContent = stats.count_roam_in;
    document.getElementById('anomalies').textContent = stats.count_anomalies;
    document.getElementById('notifications').textContent = stats.count_notifications;

    // Optionally populate chart with dummy or processed data
    updateChart([120, 190, 300, 500, 200, 300, 400]); // Static data (can be removed if dynamic below is used)
  })
  .catch(error => {
    console.error('Error fetching data:', error);
    document.getElementById('last-load-date').textContent = 'Unavailable';
  });

// Fetch time-series data for chart
document.addEventListener('DOMContentLoaded', function () {
  fetch(`${API_BASE}/api/v1/roamout-by-date`)
    .then(response => response.json())
    .then(data => {
      const dates = data.data.map(item => item.date);
      const counts = data.data.map(item => item.count);

      // Update the Roamers Out card value
      document.getElementById('roam-out').textContent = counts[counts.length - 1];

      // Update chart
      updateChartWithLabels(dates, counts);
    })
    .catch(error => {
      console.error('Error fetching roamout data:', error);
    });
});

// Chart update (generic for dynamic data)
function updateChartWithLabels(labels, dataPoints) {
  const ctx = document.getElementById('visitorsChart').getContext('2d');

  new Chart(ctx, {
    type: 'line',
    data: {
      labels: labels,
      datasets: [{
        label: 'Roamers Out',
        data: dataPoints,
        backgroundColor: 'rgba(0, 192, 239, 0.2)',
        borderColor: '#00c0ef',
        borderWidth: 2,
        tension: 0.4,
        pointBackgroundColor: '#00c0ef'
      }]
    },
    options: {
      responsive: true,
      maintainAspectRatio: false,
      scales: {
        x: {
          title: {
            display: true,
            text: 'Date'
          }
        },
        y: {
          beginAtZero: true,
          title: {
            display: true,
            text: 'Count'
          }
        }
      }
    }
  });
}
