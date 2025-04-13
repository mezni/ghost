// Fetch data from API and populate dashboard
fetch('http://localhost:3000/api/v1/overview')
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
    updateChart([120, 190, 300, 500, 200, 300, 400]); // Static data for now
  })
  .catch(error => {
    console.error('Error fetching data:', error);
    document.getElementById('last-load-date').textContent = 'Unavailable';
  });


// Initialize chart
function updateChart(dataPoints) {
  const ctx = document.getElementById('visitorsChart').getContext('2d');

  new Chart(ctx, {
    type: 'line',
    data: {
      labels: ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'],
      datasets: [{
        label: 'Visitors',
        data: dataPoints,
        backgroundColor: 'rgba(60,141,188,0.2)',
        borderColor: 'rgba(60,141,188,1)',
        borderWidth: 2,
        pointBackgroundColor: '#3b8bba'
      }]
    },
    options: {
      responsive: true,
      maintainAspectRatio: false,
      scales: {
        y: {
          beginAtZero: true
        }
      }
    }
  });
}


document.addEventListener('DOMContentLoaded', function () {
  // Fetch the data from the API
  fetch('http://localhost:3000/api/v1/roamout-by-date')
      .then(response => response.json())
      .then(data => {
          // Extract the date and count arrays
          const dates = data.data.map(item => item.date);
          const counts = data.data.map(item => item.count);

          // Update the Roamers Out card value
          document.getElementById('roam-out').textContent = counts[counts.length - 1];

          // Update the chart title
          const chartTitle = document.querySelector('.card-title');
          chartTitle.textContent = 'Roamers Out par date';

          // Create the chart with the fetched data
          const ctx = document.getElementById('visitorsChart').getContext('2d');

          const visitorsChart = new Chart(ctx, {
              type: 'line',
              data: {
                  labels: dates, // X-axis labels (dates)
                  datasets: [{
                      label: 'Roamers Out',
                      data: counts, // Y-axis data (counts)
                      borderColor: '#00c0ef',
                      backgroundColor: 'rgba(0, 192, 239, 0.2)',
                      borderWidth: 2,
                      tension: 0.4
                  }]
              },
              options: {
                  responsive: true,
                  maintainAspectRatio: false,
                  scales: {
                      x: {
                          beginAtZero: true,
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
      })
      .catch(error => {
          console.error('Error fetching roamout data:', error);
      });
});
