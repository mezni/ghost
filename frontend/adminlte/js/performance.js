$(document).ready(function () {
  const API_BASE = 'http://localhost:3000/api/v1';
  const NOTIFICATIONS_ENDPOINT = '/notifications';

  function loadPerformanceData() {
    fetch(`${API_BASE}${NOTIFICATIONS_ENDPOINT}/details`)
      .then(response => response.json())
      .then(data => {
        const tableBody = document.querySelector('#performance-table tbody');
        tableBody.innerHTML = '';

        data.data.forEach(item => {
          const configure = Number(item.perct_configure);
          const reel = Number(item.perct_reel);
          const variance = reel - configure;
        
          let varianceDisplay = '';
        
          if (variance > 0) {
            varianceDisplay = `<span class="text-success"><i class="fas fa-arrow-up"></i> ${variance.toFixed(2)}%</span>`;
          } else if (variance < 0) {
            varianceDisplay = `<span class="text-danger"><i class="fas fa-arrow-down"></i> ${Math.abs(variance).toFixed(2)}%</span>`;
          } else {
            varianceDisplay = `<span class="text-muted">0%</span>`;
          }
        
          const row = document.createElement('tr');
          row.innerHTML = `
            <td>${item.operator}</td>
            <td>${configure.toFixed(2)}%</td>
                      <td>${reel.toFixed(2)}%</td>
            <td class="text-center">${varianceDisplay}</td>

          `;
          tableBody.appendChild(row);
        });
        
      })
      .catch(error => {
        console.error('Error loading data:', error);
      });
  }

  loadPerformanceData();
  setInterval(loadPerformanceData, 300000); // refresh every 5 minutes
});
