console.log("🎯 sorperformance.js loaded");

window.sorperformanceInit = async function() {
  console.log("💠 SoR Performance page initialized");

  // Initialize state
  window.sorData = [];
  window.filteredSorData = [];
  window.currentPage = 1;
  window.itemsPerPage = 10;
  window.currentSubscriberData = null;

  await loadSorPerformanceData();
  initPaginationButtons();
  initRefreshButton();
  initSearch();
  initExportButton();
};

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