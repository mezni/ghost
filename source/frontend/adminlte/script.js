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
  })
  .catch(error => {
    console.error('Error fetching overview data:', error);
    document.getElementById('last-load-date').textContent = 'Unavailable';
  });

document.addEventListener('DOMContentLoaded', function () {
  // Fetch Roamers OUT
  fetch(`${API_BASE}/api/v1/roamout-by-date`)
    .then(response => response.json())
    .then(data => {
      const dates = data.data.map(item => item.date);
      const counts = data.data.map(item => item.count);

      // Update chart
      renderLineChart('visitorsChart', 'Roamers Out', dates, counts, '#00c0ef');
    })
    .catch(error => {
      console.error('Error fetching roamout data:', error);
    });

  // Fetch Roamers IN
  fetch(`${API_BASE}/api/v1/roamin-by-date`)
    .then(response => response.json())
    .then(data => {
      const dates = data.data.map(item => item.date);
      const counts = data.data.map(item => item.count);

      // Update chart
      renderLineChart('roamersInChart', 'Roamers In', dates, counts, '#28a745');
    })
    .catch(error => {
      console.error('Error fetching roamin data:', error);
    });
});

// Reusable chart rendering function
function renderLineChart(canvasId, label, labels, dataPoints, color) {
  const ctx = document.getElementById(canvasId).getContext('2d');

  new Chart(ctx, {
    type: 'line',
    data: {
      labels: labels,
      datasets: [{
        label: label,
        data: dataPoints,
        backgroundColor: color + '33',
        borderColor: color,
        borderWidth: 2,
        tension: 0.4,
        pointBackgroundColor: color
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
