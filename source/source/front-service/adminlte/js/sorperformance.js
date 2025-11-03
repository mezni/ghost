console.log("🎯 sorperformance.js loaded");

window.sorperformanceInit = async function() {
  console.log("💠 SoR Performance page initialized");

  // Initialize state
  window.sorData = [];
  window.filteredSorData = [];
  window.currentPage = 1;
  window.itemsPerPage = 10;
  window.currentSubscriberData = null;
  window.goalCompletionData = [];

  await loadSorPerformanceData();
  await loadChartData(); // Load chart data
  await loadGoalCompletionData(); // Load goal completion data
  initPaginationButtons();
  initRefreshButton();
  initSearch();
  initExportButton();
};

// -------------------------
// Load Chart Data
// -------------------------
async function loadChartData() {
  try {
    console.log("📊 Loading chart data...");
    
    const res = await fetch(`${API_URL}/analytics`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        "dimension": "sor_performance",
        "aggregation": "history",
        "size": 30,
        "filter": [
          {"key": "is_barring", "value": "true"}
        ]
      })
    });

    if (!res.ok) throw new Error(`HTTP ${res.status}`);
    
    const data = await res.json();
    console.log("📈 Chart data received:", data);
    
    // Process data: sum operator_count by date
    const processedData = processChartData(data.data || []);
    renderLineChart(processedData);
    
    // Update footer metrics safely
    await updateFooterMetricsWithWait(data.data || []);
    
  } catch (err) {
    console.error("❌ Failed to load chart data:", err);
    // Show error in chart container
    const chartContainer = document.getElementById('sales-chart');
    if (chartContainer) {
      chartContainer.innerHTML = `
        <div class="d-flex align-items-center justify-content-center h-100 bg-light rounded">
          <p class="text-danger mb-0">Error loading chart: ${err.message}</p>
        </div>
      `;
    }
    resetFooterMetrics();
  }
}

// -------------------------
// Wait for footer elements and then update metrics
// -------------------------
async function updateFooterMetricsWithWait(rawData) {
  try {
    // Wait for footer elements to be available (using the correct IDs from your HTML)
    await waitForElement('#totalOperators');
    await waitForElement('#blacklistedOperators');
    await waitForElement('#affectedCountries');
    await waitForElement('#totalRoamers');
    
    updateFooterMetrics(rawData);
  } catch (err) {
    console.warn("⚠️ Could not find footer elements after waiting:", err);
  }
}

// -------------------------
// Update Footer Metrics
// -------------------------
function updateFooterMetrics(rawData) {
  // Check if footer metric elements exist
  const totalOperatorsEl = document.getElementById('totalOperators');
  const blacklistedOperatorsEl = document.getElementById('blacklistedOperators');
  const affectedCountriesEl = document.getElementById('affectedCountries');
  const totalRoamersEl = document.getElementById('totalRoamers');
  
  if (!totalOperatorsEl || !blacklistedOperatorsEl || !affectedCountriesEl || !totalRoamersEl) {
    console.warn("⚠️ Footer metric elements not found, skipping update");
    return;
  }

  if (!rawData || rawData.length === 0) {
    resetFooterMetrics();
    return;
  }

  // Calculate metrics
  const metrics = calculateMetrics(rawData);
  
  // Update DOM elements
  totalOperatorsEl.textContent = metrics.totalOperators;
  blacklistedOperatorsEl.textContent = metrics.blacklistedOperators;
  affectedCountriesEl.textContent = metrics.affectedCountries;
  totalRoamersEl.textContent = metrics.totalRoamers.toLocaleString();
}

// -------------------------
// Calculate Metrics from Data
// -------------------------
function calculateMetrics(rawData) {
  const metrics = {
    totalOperators: 0,
    blacklistedOperators: 0,
    affectedCountries: 0,
    totalRoamers: 0
  };

  // Use Sets to track unique values
  const uniqueOperators = new Set();
  const uniqueCountries = new Set();
  const blacklistedOperatorsSet = new Set();

  rawData.forEach(item => {
    // Count unique operators
    if (item.operator) {
      uniqueOperators.add(item.operator);
    }
    
    // Count unique countries
    if (item.country) {
      uniqueCountries.add(item.country);
    }
    
    // If is_barring is true, count as blacklisted
    if (item.is_barring === true || item.is_barring === 'true') {
      if (item.operator) {
        blacklistedOperatorsSet.add(item.operator);
      }
    }
    
    // Sum operator_count for total roamers
    const operatorCount = parseInt(item.operator_count) || 0;
    metrics.totalRoamers += operatorCount;
  });

  // Set the calculated values
  metrics.totalOperators = uniqueOperators.size;
  metrics.blacklistedOperators = blacklistedOperatorsSet.size;
  metrics.affectedCountries = uniqueCountries.size;

  console.log("📊 Calculated metrics:", metrics);
  return metrics;
}

// -------------------------
// Reset Footer Metrics to Zero
// -------------------------
function resetFooterMetrics() {
  const totalOperatorsEl = document.getElementById('totalOperators');
  const blacklistedOperatorsEl = document.getElementById('blacklistedOperators');
  const affectedCountriesEl = document.getElementById('affectedCountries');
  const totalRoamersEl = document.getElementById('totalRoamers');
  
  if (totalOperatorsEl) totalOperatorsEl.textContent = '0';
  if (blacklistedOperatorsEl) blacklistedOperatorsEl.textContent = '0';
  if (affectedCountriesEl) affectedCountriesEl.textContent = '0';
  if (totalRoamersEl) totalRoamersEl.textContent = '0';
}

// -------------------------
// Load Goal Completion Data
// -------------------------
async function loadGoalCompletionData() {
  try {
    console.log("📊 Loading goal completion data...");
    
    const res = await fetch(`${API_URL}/analytics`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        "dimension": "sor_performance",
        "aggregation": "history",
        "size": 30,
        "filter": [
          {"key": "is_barring", "value": "true"}
        ]
      })
    });

    if (!res.ok) throw new Error(`HTTP ${res.status}`);
    
    const data = await res.json();
    console.log("📊 Goal completion data received:", data);
    
    // Process data for goal completion
    const processedData = processGoalCompletionData(data.data || []);
    renderGoalCompletion(processedData);
    
  } catch (err) {
    console.error("❌ Failed to load goal completion data:", err);
    // Show error in goal completion section
    const goalSection = document.getElementById('goal-completion-container');
    if (goalSection) {
      const errorDiv = document.createElement('div');
      errorDiv.className = 'alert alert-danger mt-3';
      errorDiv.textContent = `Error loading goal completion data: ${err.message}`;
      goalSection.appendChild(errorDiv);
    }
  }
}

// -------------------------
// Process Goal Completion Data
// -------------------------
function processGoalCompletionData(rawData) {
  if (!rawData || rawData.length === 0) {
    return [];
  }

  // Calculate variance for each country/operator combination
  const varianceMap = {};
  
  rawData.forEach(item => {
    if (!item.country || !item.operator) return;
    
    const key = `${item.country} / ${item.operator}`;
    const operatorCount = parseInt(item.operator_count) || 0;
    const countryCount = parseInt(item.country_count) || 1; // Avoid division by zero
    
    // Calculate ratio
    const ratio = countryCount > 0 ? (operatorCount / countryCount) * 100 : 0;
    
    // Store or update variance data
    if (!varianceMap[key] || Math.abs(ratio) > Math.abs(varianceMap[key].variance)) {
      varianceMap[key] = {
        country: item.country,
        operator: item.operator,
        operatorCount: operatorCount,
        countryCount: countryCount,
        variance: ratio,
        ratio: ratio.toFixed(1)
      };
    }
  });

  // Convert to array and sort by absolute variance (descending)
  const sortedData = Object.values(varianceMap)
    .sort((a, b) => Math.abs(b.variance) - Math.abs(a.variance))
    .slice(0, 5); // Take top 5

  console.log("📊 Processed goal completion data:", sortedData);
  return sortedData;
}

// -------------------------
// Render Goal Completion
// -------------------------
function renderGoalCompletion(goalData) {
  // Add an ID to the goal completion container for easier targeting
  let goalContainer = document.getElementById('goal-completion-container');
  
  // If ID doesn't exist yet, try to find the container and add the ID
  if (!goalContainer) {
    goalContainer = document.querySelector('.col-md-4 .card-body');
    if (goalContainer) {
      goalContainer.id = 'goal-completion-container';
    }
  }
  
  // Alternative selectors if the above fails
  if (!goalContainer) {
    const rightColumn = document.querySelector('.row > .col-md-4');
    if (rightColumn) {
      goalContainer = rightColumn.querySelector('.card-body');
      if (goalContainer) {
        goalContainer.id = 'goal-completion-container';
      }
    }
  }

  if (!goalContainer) {
    console.error("❌ Goal completion container not found after multiple attempts");
    console.log("Available elements:", {
      allCardBodies: document.querySelectorAll('.card-body'),
      allColMd4: document.querySelectorAll('.col-md-4'),
      rows: document.querySelectorAll('.row')
    });
    return;
  }

  // Clear existing content except the title
  const title = goalContainer.querySelector('p.text-center');
  goalContainer.innerHTML = '';
  if (title) {
    goalContainer.appendChild(title);
  }

  if (!goalData || goalData.length === 0) {
    const noDataGroup = document.createElement('div');
    noDataGroup.className = 'progress-group';
    noDataGroup.innerHTML = `
      <span class="progress-text">No data available</span>
      <span class="float-end"><b>0</b>/0</span>
      <div class="progress progress-sm">
        <div class="progress-bar text-bg-secondary" style="width: 0%"></div>
      </div>
    `;
    goalContainer.appendChild(noDataGroup);
    return;
  }

  // Add new progress groups for each top variance item
  goalData.forEach((item, index) => {
    const progressGroup = document.createElement('div');
    progressGroup.className = 'progress-group';
    
    // Determine progress bar color based on variance
    let progressBarClass = 'text-bg-primary';
    if (item.variance > 50) progressBarClass = 'text-bg-success';
    else if (item.variance > 25) progressBarClass = 'text-bg-info';
    else if (item.variance > 10) progressBarClass = 'text-bg-warning';
    else if (item.variance <= 10) progressBarClass = 'text-bg-danger';

    // Calculate width (cap at 100%)
    const width = Math.min(Math.abs(item.variance), 100);
    
    progressGroup.innerHTML = `
      <span class="progress-text">${item.country} / ${item.operator}</span>
      <span class="float-end"><b>${item.operatorCount}</b>/${item.countryCount}</span>
      <div class="progress progress-sm">
        <div class="progress-bar ${progressBarClass}" style="width: ${width}%"></div>
      </div>
      <small class="text-muted">Ratio: ${item.ratio}%</small>
    `;
    
    goalContainer.appendChild(progressGroup);
  });

  console.log("✅ Goal completion rendered successfully");
}

// -------------------------
// Process Chart Data - Sum operator_count by date
// -------------------------
function processChartData(rawData) {
  if (!rawData || rawData.length === 0) {
    return { labels: [], datasets: [] };
  }

  // Group by date and sum operator_count
  const dateMap = {};
  
  rawData.forEach(item => {
    if (!item.date) return;
    
    const dateKey = item.date.split('T')[0]; // Use only date part
    const operatorCount = parseInt(item.operator_count) || 0;
    
    if (dateMap[dateKey]) {
      dateMap[dateKey] += operatorCount;
    } else {
      dateMap[dateKey] = operatorCount;
    }
  });

  // Convert to arrays for chart
  const labels = Object.keys(dateMap).sort(); // Sort dates chronologically
  const data = labels.map(date => dateMap[date]);

  console.log("📊 Processed chart data:", { labels, data });

  return {
    labels,
    datasets: [{
      label: 'Total Operators with Barring',
      data,
      borderColor: '#0d6efd',
      backgroundColor: 'rgba(13, 110, 253, 0.1)',
      tension: 0.4,
      fill: true
    }]
  };
}

// -------------------------
// Render Line Chart
// -------------------------
function renderLineChart(chartData) {
  const chartContainer = document.getElementById('sales-chart');
  if (!chartContainer) {
    console.error("❌ Chart container not found");
    return;
  }

  // Clear existing content
  chartContainer.innerHTML = '';

  if (!chartData.labels || chartData.labels.length === 0) {
    chartContainer.innerHTML = `
      <div class="d-flex align-items-center justify-content-center h-100 bg-light rounded">
        <p class="text-muted mb-0">No chart data available</p>
      </div>
    `;
    return;
  }

  // Create canvas for chart
  const canvas = document.createElement('canvas');
  canvas.id = 'sorPerformanceChart';
  canvas.style.minHeight = '195px';
  chartContainer.appendChild(canvas);

  // Initialize Chart.js
  const ctx = canvas.getContext('2d');
  
  try {
    new Chart(ctx, {
      type: 'line',
      data: chartData,
      options: {
        responsive: true,
        maintainAspectRatio: false,
        plugins: {
          legend: {
            display: true,
            position: 'top',
          },
          tooltip: {
            mode: 'index',
            intersect: false,
          }
        },
        scales: {
          x: {
            display: true,
            title: {
              display: true,
              text: 'Date'
            },
            ticks: {
              maxTicksLimit: 10,
              callback: function(value, index, values) {
                // Format date to be more readable
                const dateStr = chartData.labels[index];
                if (!dateStr) return '';
                return new Date(dateStr).toLocaleDateString('en-US', { 
                  month: 'short', 
                  day: 'numeric' 
                });
              }
            }
          },
          y: {
            display: true,
            title: {
              display: true,
              text: 'Operator Count'
            },
            beginAtZero: true,
            ticks: {
              precision: 0
            }
          }
        },
        interaction: {
          mode: 'nearest',
          axis: 'x',
          intersect: false
        }
      }
    });
    
    console.log("✅ Chart rendered successfully");
    
  } catch (error) {
    console.error("❌ Error rendering chart:", error);
    chartContainer.innerHTML = `
      <div class="d-flex align-items-center justify-content-center h-100 bg-light rounded">
        <p class="text-danger mb-0">Error rendering chart</p>
      </div>
    `;
  }
}

// -------------------------
// Utility to wait for element
// -------------------------
function waitForElement(selector) {
  return new Promise(resolve => {
    const check = () => {
      const el = document.querySelector(selector);
      if (el) resolve(el);
      else requestAnimationFrame(check);
    };
    check();
  });
}

// -------------------------
// Load SoR performance data from API
// -------------------------
async function loadSorPerformanceData() {
  try {
    const res = await fetch(`${API_URL}/analytics`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        dimension: "sor_performance",
        aggregation: "latest"
      })
    });

    if (!res.ok) throw new Error(`HTTP ${res.status}`);
    
    const data = await res.json();
    window.sorData = data.data || [];
    window.filteredSorData = [];
    window.currentPage = 1;
    
    await renderSorTable();
    
  } catch (err) {
    console.error("❌ Failed to load SoR performance data:", err);
    const tbody = await waitForElement("#sorPerformanceTable tbody");
    tbody.innerHTML = `<tr><td colspan="8" class="text-center text-danger">
      Error loading data: ${err.message}</td></tr>`;
  }
}

// -------------------------
// Render table with pagination
// -------------------------
async function renderSorTable() {
  const tbody = await waitForElement("#sorPerformanceTable tbody");
  tbody.innerHTML = "";

  const list = (window.filteredSorData && window.filteredSorData.length > 0)
    ? window.filteredSorData
    : window.sorData;

  const start = (window.currentPage - 1) * window.itemsPerPage;
  const end = start + window.itemsPerPage;
  const pageItems = list.slice(start, end);

  if (pageItems.length === 0) {
    tbody.innerHTML = `<tr><td colspan="8" class="text-center text-muted">No performance data found</td></tr>`;
    const infoEl = document.getElementById("paginationInfoSor");
    if (infoEl) infoEl.textContent = "";
    return;
  }

  pageItems.forEach((item, i) => {
    const variance = item.actual_percentage - item.target_percentage;
    const varianceValue = Math.abs(variance).toFixed(1);
    
    // Determine color and arrow based on variance
    let varianceClass = '';
    let arrowIcon = '';
    
    if (variance > 5) {
      varianceClass = 'text-success';
      arrowIcon = '<i class="fas fa-arrow-up"></i>';
    } else if (variance < -5) {
      varianceClass = 'text-danger';
      arrowIcon = '<i class="fas fa-arrow-down"></i>';
    } else {
      varianceClass = 'text-warning';
      arrowIcon = '<i class="fas fa-minus"></i>';
    }

    const row = document.createElement("tr");
    row.innerHTML = `
      <td>${start + i + 1}</td>
      <td>${formatDateDDMMYYYY(item.date)}</td>
      <td>${item.country || 'N/A'}</td>
      <td>${item.operator || 'N/A'}</td>
      <td>${item.target_percentage}%</td>
      <td>${item.actual_percentage}%</td>
      <td class="${varianceClass} fw-bold">
        ${arrowIcon} ${variance > 0 ? '+' : ''}${variance.toFixed(1)}%
      </td>
      <td>
        <button class="btn btn-sm btn-outline-primary" 
                onclick="viewSorDetails(${item.perf_id}, '${item.country || ''}', '${item.operator || ''}')">
          <i class="fas fa-eye"></i> View
        </button>
      </td>
    `;
    tbody.appendChild(row);
  });

  const totalPages = Math.ceil(list.length / window.itemsPerPage);
  const infoEl = document.getElementById("paginationInfoSor");
  if (infoEl) infoEl.textContent = `Page ${window.currentPage} of ${totalPages}`;
}

// -------------------------
// View SoR Details - Fetch Subscriber Data
// -------------------------
async function viewSorDetails(perfId, country, operator) {
  console.log("Viewing details for performance ID:", perfId, "Country:", country, "Operator:", operator);
  
  // Validate country and operator
  if (!country || !operator || country === 'N/A' || operator === 'N/A') {
    AppUtils.notify("Error", "Invalid country or operator data", "error");
    return;
  }
  
  try {
    // Show loading state
    AppUtils.notify("Loading Subscribers", `Fetching subscribers for ${operator} in ${country}...`, "info");
    
    const response = await fetch(`${API_URL}/analytics`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        dimension: "subscriber",
        aggregation: "latest",
        filter: [
          {
            "key": "direction",
            "value": "OUT"
          },
          {
            "key": "operator",
            "value": operator
          },
          {
            "key": "country",
            "value": country
          }
        ]
      })
    });

    if (!response.ok) throw new Error(`HTTP ${response.status}`);
    
    const data = await response.json();
    const subscribers = data.data || [];
    
    console.log("📊 Subscriber data fetched:", subscribers);
    
    // Display the results in a modal
    displaySubscriberResults(subscribers, country, operator);
    
  } catch (err) {
    console.error("❌ Failed to fetch subscriber data:", err);
    AppUtils.notify("Error", `Failed to fetch subscriber data: ${err.message}`, "error");
  }
}

// -------------------------
// Display Subscriber Results
// -------------------------
function displaySubscriberResults(subscribers, country, operator) {
  if (subscribers.length === 0) {
    AppUtils.notify("No Subscribers", `No subscribers found for ${operator} in ${country}`, "warning");
    return;
  }

  // Create a modal to display the results
  const modalHtml = `
    <div class="modal fade" id="subscriberModal" tabindex="-1">
      <div class="modal-dialog modal-lg">
        <div class="modal-content">
          <div class="modal-header">
            <h5 class="modal-title">
              <i class="fas fa-users"></i> Subscribers for ${operator} (${country})
            </h5>
            <button type="button" class="btn-close" data-bs-dismiss="modal"></button>
          </div>
          <div class="modal-body">
            <div class="mb-3">
              <strong>Total Subscribers:</strong> ${subscribers.length}
            </div>
            <div class="table-responsive" style="max-height: 400px; overflow-y: auto;">
              <table class="table table-sm table-striped">
                <thead>
                  <tr>
                    <th>#</th>
                    <th>Date</th>
                    <th>IMSI</th>
                    <th>MSISDN</th>
                    <th>Value</th>
                  </tr>
                </thead>
                <tbody>
                  ${subscribers.map((sub, index) => `
                    <tr>
                      <td>${index + 1}</td>
                      <td>${formatDateDDMMYYYY(sub.date)}</td>
                      <td>${sub.imsi || 'N/A'}</td>
                      <td>${sub.msisdn || 'N/A'}</td>
                      <td>${sub.value || 'N/A'}</td>
                    </tr>
                  `).join('')}
                </tbody>
              </table>
            </div>
          </div>
          <div class="modal-footer">
            <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">Close</button>
            <button type="button" class="btn btn-primary" onclick="exportSubscriberData()">
              <i class="fas fa-download"></i> Export
            </button>
          </div>
        </div>
      </div>
    </div>
  `;

  // Remove existing modal if any
  const existingModal = document.getElementById('subscriberModal');
  if (existingModal) {
    existingModal.remove();
  }

  // Add modal to DOM
  document.body.insertAdjacentHTML('beforeend', modalHtml);
  
  // Show modal
  const modal = new bootstrap.Modal(document.getElementById('subscriberModal'));
  modal.show();

  // Store current subscriber data for export
  window.currentSubscriberData = {
    subscribers,
    country,
    operator
  };
}

// -------------------------
// Export Subscriber Data
// -------------------------
function exportSubscriberData() {
  if (!window.currentSubscriberData || !window.currentSubscriberData.subscribers) {
    AppUtils.notify("Export Error", "No subscriber data to export", "error");
    return;
  }

  const { subscribers, country, operator } = window.currentSubscriberData;

  const headers = ["Date", "IMSI", "MSISDN", "Value"];
  const csvContent = [
    headers.join(","),
    ...subscribers.map(sub => [
      formatDateDDMMYYYY(sub.date),
      sub.imsi || '',
      sub.msisdn || '',
      sub.value || ''
    ].join(","))
  ].join("\n");

  const blob = new Blob([csvContent], { type: "text/csv" });
  const url = window.URL.createObjectURL(blob);
  const a = document.createElement("a");
  a.href = url;
  a.download = `subscribers-${operator}-${country}-${new Date().toISOString().split('T')[0]}.csv`;
  document.body.appendChild(a);
  a.click();
  document.body.removeChild(a);
  window.URL.revokeObjectURL(url);

  AppUtils.notify("Export Successful", "Subscriber data exported successfully", "success");
}

// -------------------------
// Pagination Buttons
// -------------------------
async function initPaginationButtons() {
  const prevBtn = await waitForElement("#prevPageBtnSor");
  const nextBtn = await waitForElement("#nextPageBtnSor");

  prevBtn.addEventListener("click", async () => {
    if (window.currentPage > 1) {
      window.currentPage--;
      await renderSorTable();
    }
  });

  nextBtn.addEventListener("click", async () => {
    const list = (window.filteredSorData && window.filteredSorData.length > 0)
      ? window.filteredSorData
      : window.sorData;

    const totalPages = Math.ceil(list.length / window.itemsPerPage);
    if (window.currentPage < totalPages) {
      window.currentPage++;
      await renderSorTable();
    }
  });
}

// -------------------------
// Refresh Button
// -------------------------
async function initRefreshButton() {
  const refreshBtn = await waitForElement("#refreshSorDataBtn");
  
  refreshBtn.addEventListener("click", async () => {
    // Show loading state
    const originalHtml = refreshBtn.innerHTML;
    refreshBtn.innerHTML = '<i class="fas fa-spinner fa-spin"></i> Refreshing...';
    refreshBtn.disabled = true;

    try {
      await loadSorPerformanceData();
      await loadChartData(); // Refresh chart data too
      await loadGoalCompletionData(); // Refresh goal completion data
      AppUtils.notify("Refresh Data", "Data refreshed successfully", "success");
    } catch (err) {
      AppUtils.notify("Refresh Data", "Failed to refresh data", "error");
    } finally {
      // Restore original state
      refreshBtn.innerHTML = originalHtml;
      refreshBtn.disabled = false;
    }
  });
}

// -------------------------
// Search Functionality
// -------------------------
function initSearch() {
  waitForElement("#sorSearch").then(input => {
    input.addEventListener("input", () => {
      const query = input.value.toLowerCase();
      window.filteredSorData = window.sorData.filter(item => 
        (item.country && item.country.toLowerCase().includes(query)) ||
        (item.operator && item.operator.toLowerCase().includes(query)) ||
        (item.date && item.date.toLowerCase().includes(query)) ||
        (item.target_percentage && item.target_percentage.toString().includes(query)) ||
        (item.actual_percentage && item.actual_percentage.toString().includes(query))
      );
      window.currentPage = 1;
      renderSorTable();
    });
  });
}

// -------------------------
// Export SoR Data Button
// -------------------------
async function initExportButton() {
  const exportBtn = await waitForElement("#exportSorDataBtn");
  
  exportBtn.addEventListener("click", () => {
    // Simple CSV export implementation
    const list = (window.filteredSorData && window.filteredSorData.length > 0)
      ? window.filteredSorData
      : window.sorData;

    if (list.length === 0) {
      AppUtils.notify("Export Data", "No data to export", "warning");
      return;
    }

    const headers = ["Date", "Country", "Operator", "Target %", "Actual %", "Variance"];
    const csvContent = [
      headers.join(","),
      ...list.map(item => [
        formatDateDDMMYYYY(item.date),
        item.country || 'N/A',
        item.operator || 'N/A',
        item.target_percentage,
        item.actual_percentage,
        (item.actual_percentage - item.target_percentage).toFixed(1)
      ].join(","))
    ].join("\n");

    const blob = new Blob([csvContent], { type: "text/csv" });
    const url = window.URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = `sor-performance-${new Date().toISOString().split('T')[0]}.csv`;
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    window.URL.revokeObjectURL(url);

    AppUtils.notify("Export Data", "Data exported successfully", "success");
  });
}

// -------------------------
// Utility Functions
// -------------------------
function formatDateDDMMYYYY(dateStr) {
  if (!dateStr) return 'N/A';
  
  try {
    const d = new Date(dateStr);
    if (isNaN(d.getTime())) return 'Invalid Date';
    
    const dd = String(d.getDate()).padStart(2, "0");
    const mm = String(d.getMonth() + 1).padStart(2, "0");
    const yyyy = d.getFullYear();
    return `${dd}/${mm}/${yyyy}`;
  } catch (error) {
    console.error("Error formatting date:", dateStr, error);
    return 'N/A';
  }
}

// -------------------------
// Initialize export button if it exists
// -------------------------
waitForElement("#exportSorDataBtn").then(() => {
  initExportButton();
});

// -------------------------
// Expose functions globally
// -------------------------
window.viewSorDetails = viewSorDetails;
window.exportSubscriberData = exportSubscriberData;
window.formatDateDDMMYYYY = formatDateDDMMYYYY;