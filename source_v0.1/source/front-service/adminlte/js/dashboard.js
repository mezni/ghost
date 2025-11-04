// ==========================
// Dashboard Page Script
// ==========================

// --------------------------
// Helpers
// --------------------------

// Format date from YYYY-MM-DD → DD/MM/YYYY
function formatDate(dateStr) {
  if (!dateStr) return "N/A";
  const [year, month, day] = dateStr.split("-");
  return `${day}/${month}/${year}`;
}

// Parse alert message to extract components
function parseAlertMessage(message) {
  if (!message) return { country: 'Unknown', operator: 'Unknown', type: 'Unknown', details: message };
  
  // Try to parse different alert formats
  let country = 'Unknown';
  let operator = 'Unknown';
  let type = 'Deviation';
  let details = message;
  
  // Format 1: "Country:Operator is blacklisted and X roamer(s) found"
  const blacklistedMatch = message.match(/^(.+?):(.+?) is blacklisted and (.+?) roamer\(s\) found$/);
  if (blacklistedMatch) {
    country = blacklistedMatch[1];
    operator = blacklistedMatch[2];
    type = 'Blacklisted';
    details = `${blacklistedMatch[3]} roamers detected on blacklisted operator`;
  }
  
  // Format 2: "Country:Operator is out of interval X/Y (roamers) %actual=Y.YY or %target=X.XX"
  const deviationMatch = message.match(/^(.+?):(.+?) is out of interval (.+?)\/(.+?) \(roamers\) %actual=(.+?) or %target=(.+)$/);
  if (deviationMatch) {
    country = deviationMatch[1];
    operator = deviationMatch[2];
    const operatorCount = deviationMatch[3];
    const countryCount = deviationMatch[4];
    const actualPercent = deviationMatch[5];
    const targetPercent = deviationMatch[6];
    type = 'Deviation';
    details = `Actual: ${actualPercent}% vs Target: ${targetPercent}%`;
  }
  
  return { country: country, operator: operator, type: type, details: details, original: message };
}

// Extract statistics from alert message
function extractStatistics(message) {
  const stats = { operatorCount: 0, countryCount: 0, actualPercent: 0, targetPercent: 0 };
  
  // Extract numbers from deviation alerts
  const deviationMatch = message.match(/is out of interval (\d+)\/(\d+) .*%actual=([\d.]+).*%target=([\d.]+)/);
  if (deviationMatch) {
    stats.operatorCount = parseInt(deviationMatch[1]);
    stats.countryCount = parseInt(deviationMatch[2]);
    stats.actualPercent = parseFloat(deviationMatch[3]);
    stats.targetPercent = parseFloat(deviationMatch[4]);
  }
  
  // Extract numbers from blacklisted alerts
  const blacklistedMatch = message.match(/and (\d+) roamer\(s\) found/);
  if (blacklistedMatch) {
    stats.operatorCount = parseInt(blacklistedMatch[1]);
  }
  
  return stats;
}

// Get badge color based on alert type
function getAlertBadge(type) {
  const badges = {
    'Blacklisted': 'bg-danger',
    'Deviation': 'bg-warning',
    'Unknown': 'bg-secondary'
  };
  return badges[type] || 'bg-secondary';
}

// Get status text based on deviation
function getStatusText(stats, type) {
  if (type === 'Blacklisted') return 'Critical';
  if (type === 'Deviation') {
    const variance = Math.abs(stats.actualPercent - stats.targetPercent);
    if (variance > 20) return 'High Risk';
    if (variance > 10) return 'Medium Risk';
    return 'Low Risk';
  }
  return 'Unknown';
}

// Get status badge color
function getStatusBadge(status) {
  const badges = {
    'Critical': 'bg-danger',
    'High Risk': 'bg-danger',
    'Medium Risk': 'bg-warning',
    'Low Risk': 'bg-info',
    'Unknown': 'bg-secondary'
  };
  return badges[status] || 'bg-secondary';
}

// Animate number update
function animateNumber(el) {
  if (!el) return;
  el.classList.add("updated");
  setTimeout(() => el.classList.remove("updated"), 500);
}

// Safe fetch and update DOM
async function fetchMetric(body, valueId, dateId) {
  try {
    const res = await fetch(`${BASE_API}/analytics`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(body)
    });
    const data = await res.json();

    const elValue = document.getElementById(valueId);
    const elDate = document.getElementById(dateId);

    if (!elValue || !elDate) return;

    if (data && data.status === "success" && data.data && data.data.length) {
      const latest = data.data[0];
      elValue.innerText = latest.value;
      elDate.innerText = `Updated at: ${formatDate(latest.date)}`;
      animateNumber(elValue);
    } else {
      elValue.innerText = "0";
      elDate.innerText = "Updated at: N/A";
    }
  } catch (err) {
    console.error(`Error fetching ${valueId}:`, err);
    const elValue = document.getElementById(valueId);
    const elDate = document.getElementById(dateId);
    if (elValue) elValue.innerText = "0";
    if (elDate) elDate.innerText = "Updated at: Error";
  }
}

// --------------------------
// Load Alerts Table
// --------------------------
async function loadAlertsTable() {
  const tableBody = document.getElementById("alertsTableBody");
  const countBadge = document.getElementById("alertsCountBadge");
  const lastUpdate = document.getElementById("alertsLastUpdate");

  if (!tableBody || !countBadge) return;

  try {
    const res = await fetch(`${BASE_API}/analytics`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        dimension: "alerts",
        aggregation: "detail"
      })
    });

    const data = await res.json();
    
    if (lastUpdate) {
      lastUpdate.textContent = new Date().toLocaleString();
    }

    if (data && data.status === "success" && data.data && data.data.length) {
      const alerts = data.data;
      countBadge.textContent = `${alerts.length} Alert${alerts.length !== 1 ? 's' : ''}`;
      
      tableBody.innerHTML = '';
      
      alerts.forEach((alert, index) => {
        const parsed = parseAlertMessage(alert.value);
        const stats = extractStatistics(alert.value);
        const status = getStatusText(stats, parsed.type);
        
        const row = document.createElement('tr');
        row.innerHTML = `
          <td>
            <small class="text-muted">${formatDate(alert.date)}</small>
          </td>
          <td>
            <strong>${parsed.country}</strong>
          </td>
          <td>${parsed.operator}</td>
          <td>
            <span class="badge ${getAlertBadge(parsed.type)}">${parsed.type}</span>
          </td>
          <td>
            <small>${parsed.details}</small>
          </td>
          <td>
            ${stats.operatorCount > 0 ? `
              <div class="small">
                <span class="text-muted">Operators:</span> ${stats.operatorCount}
                ${stats.countryCount > 0 ? `<br><span class="text-muted">Total:</span> ${stats.countryCount}` : ''}
                ${stats.actualPercent > 0 ? `<br><span class="text-muted">Actual:</span> ${stats.actualPercent}%` : ''}
                ${stats.targetPercent > 0 ? `<br><span class="text-muted">Target:</span> ${stats.targetPercent}%` : ''}
              </div>
            ` : '<small class="text-muted">No stats</small>'}
          </td>
          <td>
            <span class="badge ${getStatusBadge(status)}">${status}</span>
          </td>
        `;
        tableBody.appendChild(row);
      });
      
    } else {
      countBadge.textContent = '0 Alerts';
      tableBody.innerHTML = `
        <tr>
          <td colspan="7" class="text-center text-muted py-4">
            <i class="bi bi-check-circle text-success me-2"></i>
            No active alerts found
          </td>
        </tr>
      `;
    }
  } catch (err) {
    console.error("Error loading alerts table:", err);
    countBadge.textContent = 'Error';
    tableBody.innerHTML = `
      <tr>
        <td colspan="7" class="text-center text-danger py-4">
          <i class="bi bi-exclamation-triangle me-2"></i>
          Error loading alerts. Please try again.
        </td>
      </tr>
    `;
  }
}

// --------------------------
// Load Notifications / Messages
// --------------------------
async function loadMessages() {
  const container = document.getElementById("messagesContainer");
  const badge = document.getElementById("messagesBadge");

  if (!container || !badge) return;

  try {
    const res = await fetch(`${BASE_API}/analytics`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        dimension: "notification",
        aggregation: "detail"
      })
    });

    const data = await res.json();
    container.innerHTML = "";

    if (data && data.status === "success" && data.data && data.data.length) {
      const total = data.data.length;
      badge.textContent = total;

      data.data.forEach((item, idx) => {
        const msg = document.createElement("div");
        msg.className = "direct-chat-msg";
        msg.innerHTML = `
          <div class="direct-chat-infos clearfix">
            <span class="direct-chat-name float-start">System</span>
            <span class="direct-chat-timestamp float-end">${formatDate(item.date || new Date().toISOString().split("T")[0])}</span>
          </div>
          <img class="direct-chat-img" src="https://ui-avatars.com/api/?name=SY" alt="user image">
          <div class="direct-chat-text">
            ${item.message || `#${idx + 1}: ${item.value || "No message text"}`}
          </div>
        `;
        container.appendChild(msg);
      });
    } else {
      badge.textContent = "0";
      container.innerHTML = '<p class="text-muted text-center">No notifications</p>';
    }
  } catch (err) {
    console.error("Error loading messages:", err);
    badge.textContent = "0";
    container.innerHTML = '<p class="text-danger text-center">Error loading notifications</p>';
  }
}

// --------------------------
// Global Trend Chart
// --------------------------
async function loadGlobalTrendChart() {
  const chartEl = document.getElementById("global-chart");
  if (!chartEl) return;

  try {
    // Fetch Roam IN history
    const resIn = await fetch(`${BASE_API}/analytics`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        dimension: "global",
        aggregation: "history",
        filter: [{ key: "direction", value: "in" }]
      })
    });
    const dataIn = await resIn.json();

    // Fetch Roam OUT history
    const resOut = await fetch(`${BASE_API}/analytics`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        dimension: "global",
        aggregation: "history",
        filter: [{ key: "direction", value: "out" }]
      })
    });
    const dataOut = await resOut.json();

    // Prepare chart data
    const labels = [];
    const inValues = [];
    const outValues = [];

    if (dataIn && dataIn.status === "success" && dataOut && dataOut.status === "success") {
      dataIn.data.forEach((item, idx) => {
        labels.push(formatDate(item.date));
        inValues.push(item.value);
        outValues.push(dataOut.data[idx] ? dataOut.data[idx].value : 0);
      });
    }

    new Chart(chartEl.getContext("2d"), {
      type: "line",
      data: {
        labels: labels,
        datasets: [
          {
            label: "Roam IN",
            data: inValues,
            borderColor: "blue",
            backgroundColor: "rgba(0,0,255,0.1)",
            tension: 0.2,
          },
          {
            label: "Roam OUT",
            data: outValues,
            borderColor: "green",
            backgroundColor: "rgba(0,255,0,0.1)",
            tension: 0.2,
          }
        ]
      },
      options: {
        responsive: true,
        maintainAspectRatio: false,
        plugins: {
          legend: { position: "top" }
        },
        scales: {
          x: {
            title: { display: false }
          },
          y: {
            title: { display: true, text: "Value" },
            beginAtZero: true
          }
        }
      }
    });
  } catch (err) {
    console.error("Error loading Global Trend chart:", err);
  }
}

// --------------------------
// Dashboard Initialization
// --------------------------
async function dashboardInit() {
  console.log("💠 Dashboard initialized");

  // Metrics for info boxes
  const metrics = [
    { body: { dimension: "global", aggregation: "latest", filter: [{ key: "direction", value: "in" }] }, valueId: "roamInValue", dateId: "roamInDate" },
    { body: { dimension: "global", aggregation: "latest", filter: [{ key: "direction", value: "out" }] }, valueId: "roamOutValue", dateId: "roamOutDate" },
    { body: { dimension: "alerts", aggregation: "summary" }, valueId: "alertsValue", dateId: "alertsDate" },
    { body: { dimension: "notification", aggregation: "summary" }, valueId: "notificationsValue", dateId: "notificationsDate" }
  ];

  // Update metric cards
  for (const m of metrics) {
    await fetchMetric(m.body, m.valueId, m.dateId);
  }

  // Load Global Trend chart
  await loadGlobalTrendChart();

  // Load messages (notifications)
  await loadMessages();

  // Load alerts table
  await loadAlertsTable();
}

// Note: app.js will call dashboardInit(), do not call it here