$(document).ready(function () {
  const API_BASE = 'http://localhost:3000/api/v1';

  const METRICS_ENDPOINT = '/metrics';
  const NOTIFICATIONS_ENDPOINT = '/notifications/count';  // Notifications count endpoint
  const ALERTS_ENDPOINT = '/alerts/count';  // Alerts count endpoint
  const NOTIFICATIONS_SUMMARY_ENDPOINT = '/notifications/summary';  // Add this endpoint for summary

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

  // Fetch notifications count and update the notifications card
  async function fetchAndRenderNotifications() {
    try {
      const notificationsUrl = `${API_BASE}${NOTIFICATIONS_ENDPOINT}`;
      const data = await fetchData(notificationsUrl);
      if (data.length > 0) {
        const count = data[0].count;
        $('#notifications-count').text(formatNumber(count));  // Update the notifications count
        $('#notifications-card').removeClass('bg-danger').addClass('bg-success');  // Change card color
        if (count === 0) {
          $('.small-box-footer').text('All caught up!');
        } else {
          $('.small-box-footer').text('View notifications');
        }
      } else {
        $('#notifications-count').text('No data');
      }
    } catch (err) {
      console.error('Error fetching notifications:', err);
      $('#notifications-count').text('Error');
    }
  }

  // Fetch alerts count and update the alerts card
  async function fetchAndRenderAlerts() {
    try {
      const alertsUrl = `${API_BASE}${ALERTS_ENDPOINT}`;
      const data = await fetchData(alertsUrl);
      if (data.length > 0) {
        const count = data[0].count;
        $('#alerts-count').text(formatNumber(count));  // Update the alerts count
        $('#alerts-card').removeClass('bg-danger').addClass('bg-warning');  // Change card color
        if (count === 0) {
          $('.alerts-small-box-footer').text('No active alerts!');
        } else {
          $('.alerts-small-box-footer').text('View alerts');
        }
      } else {
        $('#alerts-count').text('No data');
      }
    } catch (err) {
      console.error('Error fetching alerts:', err);
      $('#alerts-count').text('Error');
    }
  }

  // Fetch and render summary for notifications and add a row to the table
  async function fetchAndRenderNotificationsSummary() {
    try {
      const summaryUrl = `${API_BASE}${NOTIFICATIONS_SUMMARY_ENDPOINT}`;
      const data = await fetchData(summaryUrl);
      const $tableBody = $('#notifications-summary-table tbody');  // Make sure this exists in your HTML

      if (data.length > 0) {
        // Clear the table before adding new data
        $tableBody.empty();

        // Add new row to the table for each entry
        data.forEach(item => {
          const date = formatDate(item.date);
          const rule = item.rule || 'Unknown Rule';
          const count = formatNumber(item.count);

          $tableBody.append(`
            <tr>
              <td>${rule}</td>
              <td>${count}</td>
              <td>${date}</td>
            </tr>
          `);
        });
      } else {
        $tableBody.html('<tr><td colspan="3">No data available</td></tr>');
      }
    } catch (err) {
      console.error('Error fetching notifications summary:', err);
      const $tableBody = $('#notifications-summary-table tbody');
      $tableBody.html('<tr><td colspan="3">Error loading data</td></tr>');
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

      // Fetch and render notifications data
      fetchAndRenderNotifications();
      
      // Fetch and render alerts data
      fetchAndRenderAlerts();

      // Fetch and render notifications summary table data
      fetchAndRenderNotificationsSummary();

    } catch (err) {
      console.error('Error loading dashboard data:', err);
    }
  }

  loadAllData();
  setInterval(loadAllData, 300000); // Refresh every 5 minutes
});
