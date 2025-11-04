// ✅ Expose the init function on window
window.networksInit = async function() {
  console.log("💠 Networks page initialized");

  window.networks = [];
  window.filteredNetworks = [];
  window.currentPage = 1;
  window.itemsPerPage = 10;

  await loadNetworks();
  initPaginationButtons();
  initAddNetworkButton();
  initSaveNetworkButton();
  initSearch();
};

/**
 * Load networks from API
 */
async function loadNetworks() {
  try {
    const res = await fetch(`${BASE_API}/settings/networks`);
    if (!res.ok) throw new Error(`HTTP ${res.status}`);
    window.networks = await res.json();
    window.currentPage = 1;
    await renderTable();
  } catch (err) {
    console.error("❌ Failed to load networks:", err);
    const tbody = document.querySelector("#networksTable tbody");
    tbody.innerHTML = `<tr><td colspan="10" class="text-center text-danger">
      Error loading networks: ${err.message}</td></tr>`;
  }
}

/**
 * Render table
 */
async function renderTable() {
  const tbody = document.querySelector("#networksTable tbody");
  tbody.innerHTML = "";

  const list = (window.filteredNetworks && window.filteredNetworks.length > 0)
    ? window.filteredNetworks
    : window.networks;

  const start = (window.currentPage - 1) * window.itemsPerPage;
  const end = start + window.itemsPerPage;
  const pageItems = list.slice(start, end);

  if (pageItems.length === 0) {
    tbody.innerHTML = `<tr><td colspan="10" class="text-center">No networks found</td></tr>`;
    document.getElementById("paginationInfoNet").textContent = "";
    return;
  }

  pageItems.forEach((net, i) => {
    const row = document.createElement("tr");
    row.innerHTML = `
      <td>${start + i + 1}</td>
      <td>${net.country_name || 'N/A'}</td>
      <td>${net.operator_name || 'N/A'}</td>
      <td>${net.plmn_code || 'N/A'}</td>
      <td>${net.mcc || 'N/A'}</td>
      <td>${net.mnc || 'N/A'}</td>
      <td>${net.tech_2g ? "✅" : "❌"}</td>
      <td>${net.tech_3g ? "✅" : "❌"}</td>
      <td>${net.tech_lte ? "✅" : "❌"}</td>
      <td>
        <button class="btn btn-sm btn-primary me-1" onclick="editNetwork(${net.network_id})">
          <i class="fas fa-edit"></i> Edit
        </button>
        <button class="btn btn-sm btn-danger" onclick="deleteNetwork(${net.network_id})">
          <i class="fas fa-trash"></i> Delete
        </button>
      </td>
    `;
    tbody.appendChild(row);
  });

  const totalPages = Math.ceil(list.length / window.itemsPerPage);
  document.getElementById("paginationInfoNet").textContent = `Page ${window.currentPage} of ${totalPages}`;
}

/**
 * Pagination
 */
function initPaginationButtons() {
  document.getElementById("prevPageBtnNet").addEventListener("click", async () => {
    if (window.currentPage > 1) { window.currentPage--; await renderTable(); }
  });
  document.getElementById("nextPageBtnNet").addEventListener("click", async () => {
    const list = window.filteredNetworks.length > 0 ? window.filteredNetworks : window.networks;
    const totalPages = Math.ceil(list.length / window.itemsPerPage);
    if (window.currentPage < totalPages) { window.currentPage++; await renderTable(); }
  });
}

/**
 * Search
 */
function initSearch() {
  document.getElementById("networkSearch").addEventListener("input", (e) => {
    const query = e.target.value.toLowerCase();
    window.filteredNetworks = window.networks.filter(n =>
      (n.country_name && n.country_name.toLowerCase().includes(query)) ||
      (n.operator_name && n.operator_name.toLowerCase().includes(query)) ||
      (n.plmn_code && n.plmn_code.toLowerCase().includes(query))
    );
    window.currentPage = 1;
    renderTable();
  });
}

/**
 * Edit Network
 */
async function editNetwork(id) {
  const net = window.networks.find(n => n.network_id === id);
  if (!net) return;

  document.getElementById("networkId").value = net.network_id;
  document.getElementById("plmnCode").value = net.plmn_code;
  document.getElementById("plmn").value = net.plmn;
  document.getElementById("mcc").value = net.mcc;
  document.getElementById("mnc").value = net.mnc;
  document.getElementById("tech2G").checked = net.tech_2g;
  document.getElementById("tech3G").checked = net.tech_3g;
  document.getElementById("techLTE").checked = net.tech_lte;

  await loadCountries();
  document.getElementById("networkCountry").value = net.country_name;

  await loadOperators(net.country_name);
  document.getElementById("networkOperator").value = net.operator_name;

  document.getElementById("networkModalLabel").textContent = "Edit Network";
  new bootstrap.Modal(document.getElementById("networkModal")).show();
}

/**
 * Delete Network
 */
async function deleteNetwork(id) {
  if (!confirm("Are you sure you want to delete this network?")) return;
  
  try {
    const res = await fetch(`${BASE_API}/settings/networks/${id}`, { method: "DELETE" });
    if (!res.ok) throw new Error(`HTTP ${res.status}`);
    window.networks = window.networks.filter(n => n.network_id !== id);
    await renderTable();
    AppUtils.notify("Delete Network", `Network ID ${id} deleted`, "warning");
  } catch (err) {
    console.error("Failed to delete network:", err);
    AppUtils.notify("Delete Network", `Failed: ${err.message}`, "error");
  }
}

/**
 * Add Network Button
 */
function initAddNetworkButton() {
  document.getElementById("addNetworkBtn").addEventListener("click", async () => {
    document.getElementById("networkForm").reset();
    document.getElementById("networkId").value = "";
    document.getElementById("networkModalLabel").textContent = "Add Network";
    await loadCountries();
    await loadOperators();
    new bootstrap.Modal(document.getElementById("networkModal")).show();
  });
}

/**
 * Save Network Button
 */
function initSaveNetworkButton() {
  document.getElementById("saveNetworkBtn").addEventListener("click", async () => {
    const id = document.getElementById("networkId").value;
    const data = {
      plmn_code: document.getElementById("plmnCode").value,
      plmn: document.getElementById("plmn").value,
      mcc: document.getElementById("mcc").value,
      mnc: document.getElementById("mnc").value,
      country_name: document.getElementById("networkCountry").value,
      operator_name: document.getElementById("networkOperator").value,
      tech_2g: document.getElementById("tech2G").checked,
      tech_3g: document.getElementById("tech3G").checked,
      tech_lte: document.getElementById("techLTE").checked,
      created_by: "dali",
      updated_by: "dali"
    };

    try {
      let res, json;
      if (id) {
        res = await fetch(`${BASE_API}/settings/networks/${id}`, {
          method: "PUT",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify(data)
        });
        if (!res.ok) throw new Error(`HTTP ${res.status}`);
        json = await res.json();
        const idx = window.networks.findIndex(n => n.network_id == id);
        window.networks[idx] = json;
        AppUtils.notify("Edit Network", "Network updated", "info");
      } else {
        res = await fetch(`${BASE_API}/settings/networks`, {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify(data)
        });
        if (!res.ok) throw new Error(`HTTP ${res.status}`);
        json = await res.json();
        window.networks.push(json);
        AppUtils.notify("Add Network", "Network added", "success");
      }
      await renderTable();
      bootstrap.Modal.getOrCreateInstance(document.getElementById("networkModal")).hide();
    } catch (err) {
      console.error("Failed to save network:", err);
      AppUtils.notify("Save Network", `Failed: ${err.message}`, "error");
    }
  });
}

/**
 * Load Countries for Dropdown
 */
async function loadCountries() {
  const res = await fetch(`${BASE_API}/settings/countries`);
  const countries = await res.json();
  const select = document.getElementById("networkCountry");
  select.innerHTML = '<option value="">Select a country</option>';
  countries.forEach(c => {
    const opt = document.createElement("option");
    opt.value = c.country_name;
    opt.textContent = c.country_name;
    select.appendChild(opt);
  });
}

/**
 * Load Operators for Dropdown
 */
async function loadOperators(countryName) {
  const res = await fetch(`${BASE_API}/settings/operators`);
  const ops = await res.json();
  const select = document.getElementById("networkOperator");
  select.innerHTML = '<option value="">Select an operator</option>';
  ops.filter(o => !countryName || o.country_name === countryName)
     .forEach(o => {
       const opt = document.createElement("option");
       opt.value = o.operator_name;
       opt.textContent = o.operator_name;
       select.appendChild(opt);
     });
}

// ✅ Expose globally
window.editNetwork = editNetwork;
window.deleteNetwork = deleteNetwork;