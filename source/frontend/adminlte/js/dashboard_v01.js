$(document).ready(function () {
  const API_BASE = 'http://localhost:3000/api/v1';
  const METRICS_ENDPOINT = '/metrics';
  const NOTIFICATIONS_ENDPOINT = '/notifications';  // Add this endpoint for notifications

  function formatNumber(num) {
    return num ? num.toLocaleString() : 'N/A';
  }

  function formatDate(dateString) {
    if (!dateString) return '';
    return new Date(dateString).toLocaleDateString(undefined, { year: 'numeric', month: 'short', day: 'numeric' });
  }

  function calculatePercentage(count, total) {
    return total > 0 ? Math.round((count / total) * 100) : 0;
  }

  function createProgressBars(containerId, data, total, direction) {
    const $container = $(`#${containerId}`);
    $container.empty();

    if (!data || data.length === 0) {
      $container.html('<div class="alert alert-info">No data available</div>');
      return;
    }

    data.sort((a, b) => b.count - a.count);

    data.forEach(item => {
      const percentage = calculatePercentage(item.count, total);
      const barClass = direction === 'totin' ? 'bg-primary' : 'bg-info';

      $container.append(`
        <div class="mb-4">
          <p class="mb-1">
            ${item.country || 'Unknown'}
            <span class="float-right">${formatNumber(item.count)} (${percentage}%)</span>
          </p>
          <div class="progress progress-xs">
            <div class="progress-bar ${barClass}" 
                 role="progressbar" 
                 style="width: ${percentage}%"
                 aria-valuenow="${percentage}" 
                 aria-valuemin="0" 
                 aria-valuemax="100">
            </div>
          </div>
        </div>
      `);
    });
  }

  async function fetchData(url) {
    const response = await fetch(url);
    if (!response.ok) throw new Error(`Failed to fetch from ${url}`);
    const result = await response.json();
    return result.data || [];
  }

  async function fetchAndRenderMetric(url, countId, dateId) {
    try {
      const data = await fetchData(url);
      if (data.length > 0) {
        const item = data[0];
        $(`#${countId}`).text(formatNumber(item.count));
        $(`#${dateId}`).text(`as of ${formatDate(item.date)}`);
        return item.count;
      } else {
        $(`#${countId}`).text('No data');
        $(`#${dateId}`).text('');
        return 0;
      }
    } catch (err) {
      console.error(`Error fetching ${url}:`, err);
      $(`#${countId}`).text('Error');
      $(`#${dateId}`).text('');
      return 0;
    }
  }

  // New function to fetch and render notifications
  async function fetchAndRenderNotifications() {
    try {
      const notificationsUrl = `${API_BASE}${NOTIFICATIONS_ENDPOINT}/details`;
      const notifications = await fetchData(notificationsUrl);
  
      const $tableBody = $('#notifications-table tbody');
      $tableBody.empty();
  
      if (notifications.length === 0) {
        $tableBody.html('<tr><td colspan="3">No notifications available</td></tr>');
        return;
      }
  
      notifications.forEach(notification => {
        const row = `
          <tr>
            <td>${notification.operator}</td>
            <td>${notification.perct_configure}%</td>
            <td>${notification.perct_reel}%</td>
          </tr>
        `;
        $tableBody.append(row);
      });
    } catch (err) {
      console.error('Error fetching notifications:', err);
      $('#notifications-table tbody').html('<tr><td colspan="3">Error loading notifications</td></tr>');
    }
  }
  

  async function loadAllData() {
    try {
      const roamInUrl = `${API_BASE}${METRICS_ENDPOINT}?direction=totin&dimensions=global`;
      const roamOutUrl = `${API_BASE}${METRICS_ENDPOINT}?direction=out&dimensions=global`;
      const topInUrl = `${API_BASE}${METRICS_ENDPOINT}?direction=totin&dimensions=country&limit=5`;
      const topOutUrl = `${API_BASE}${METRICS_ENDPOINT}?direction=out&dimensions=country&limit=5`;

      const [roamInTotal, roamOutTotal, roamInCountries, roamOutCountries] = await Promise.all([
        fetchAndRenderMetric(roamInUrl, 'roam-in-count', 'roam-in-date'),
        fetchAndRenderMetric(roamOutUrl, 'roam-out-count', 'roam-out-date'),
        fetchData(topInUrl),
        fetchData(topOutUrl)
      ]);

      createProgressBars('roam-in-progress-bars', roamInCountries, roamInTotal, 'totin');
      createProgressBars('roam-out-progress-bars', roamOutCountries, roamOutTotal, 'out');

      // Fetch and render notifications
      fetchAndRenderNotifications();
    } catch (err) {
      console.error('Error loading dashboard data:', err);
    }
  }

  loadAllData();
  setInterval(loadAllData, 300000); // Refresh every 5 minutes
});
