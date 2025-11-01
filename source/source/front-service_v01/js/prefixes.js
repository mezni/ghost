console.log("🎯 prefixes.js loaded");

window.prefixesInit = async function () {
  console.log("💠 Prefixes page initialized");

  window.prefixes = [];
  window.filteredPrefixes = [];
  window.currentPage = 1;
  window.itemsPerPage = 10;

  await loadPrefixes();
  initPaginationButtons();
  initAddPrefixButton();
  initSavePrefixButton();
  initSearch();
};

// -------------------------
// Load prefixes from API
// -------------------------
async function loadPrefixes() {
  const apiUrl = window.API_URL || "http://localhost:3000/api/v1/settings";
  try {
    const res = await fetch(`${apiUrl}/prefixes`);
    if (!res.ok) throw new Error(`HTTP ${res.status}`);
    window.prefixes = await res.json();
    window.currentPage = 1;
    await renderTable();
  } catch (err) {
    console.error("❌ Failed to load prefixes:", err);
    const tbody = document.querySelector("#prefixesTable tbody");
    tbody.innerHTML = `<tr><td colspan="6" class="text-center text-danger">
      Error loading prefixes: ${err.message}</td></tr>`;
  }
}

// -------------------------
// Render table
// -------------------------
async function renderTable() {
  const tbody = document.querySelector("#prefixesTable tbody");
  tbody.innerHTML = "";

  const list = (window.filteredPrefixes && window.filteredPrefixes.length > 0)
    ? window.filteredPrefixes
    : window.prefixes;

  const start = (window.currentPage - 1) * window.itemsPerPage;
  const end = start + window.itemsPerPage;
  const pageItems = list.slice(start, end);

  if (pageItems.length === 0) {
    tbody.innerHTML = `<tr><td colspan="6" class="text-center">No prefixes found</td></tr>`;
    document.getElementById("paginationInfoPrefix").textContent = "";
    return;
  }

  pageItems.forEach((pfx, i) => {
    const row = document.createElement("tr");
    row.innerHTML = `
      <td>${start + i + 1}</td>
      <td>${pfx.country_name || 'N/A'}</td>
      <td>${pfx.operator_name || 'N/A'}</td>
      <td>${pfx.prefix || 'N/A'}</td>
      <td>${pfx.is_valid ? "✅" : "❌"}</td>
      <td>
        <button class="btn btn-sm btn-primary me-1" onclick="editPrefix(${pfx.prefix_id})">
          <i class="fas fa-edit"></i> Edit
        </button>
        <button class="btn btn-sm btn-danger" onclick="deletePrefix(${pfx.prefix_id})">
          <i class="fas fa-trash"></i> Delete
        </button>
      </td>
    `;
    tbody.appendChild(row);
  });

  const totalPages = Math.ceil(list.length / window.itemsPerPage);
  document.getElementById("paginationInfoPrefix").textContent = `Page ${window.currentPage} of ${totalPages}`;
}

// -------------------------
// Pagination
// -------------------------
function initPaginationButtons() {
  document.getElementById("prevPageBtnPrefix").addEventListener("click", async () => {
    if (window.currentPage > 1) { window.currentPage--; await renderTable(); }
  });
  document.getElementById("nextPageBtnPrefix").addEventListener("click", async () => {
    const list = window.filteredPrefixes.length > 0 ? window.filteredPrefixes : window.prefixes;
    const totalPages = Math.ceil(list.length / window.itemsPerPage);
    if (window.currentPage < totalPages) { window.currentPage++; await renderTable(); }
  });
}

// -------------------------
// Search
// -------------------------
function initSearch() {
  document.getElementById("prefixSearch").addEventListener("input", (e) => {
    const query = e.target.value.toLowerCase();
    window.filteredPrefixes = window.prefixes.filter(p =>
      (p.country_name && p.country_name.toLowerCase().includes(query)) ||
      (p.operator_name && p.operator_name.toLowerCase().includes(query)) ||
      (p.prefix && p.prefix.toLowerCase().includes(query))
    );
    window.currentPage = 1;
    renderTable();
  });
}

// -------------------------
// Edit / Delete / Add
// -------------------------
async function editPrefix(id) {
  const pfx = window.prefixes.find(p => p.prefix_id === id);
  if (!pfx) return;

  document.getElementById("prefixId").value = pfx.prefix_id;
  document.getElementById("prefix").value = pfx.prefix;
  document.getElementById("isValid").checked = pfx.is_valid;

  await loadCountries();
  document.getElementById("prefixCountry").value = pfx.country_name;

  await loadOperators(pfx.country_name);
  document.getElementById("prefixOperator").value = pfx.operator_name;

  document.getElementById("prefixModalLabel").textContent = "Edit Prefix";
  new bootstrap.Modal(document.getElementById("prefixModal")).show();
}

async function deletePrefix(id) {
  if (!confirm("Are you sure you want to delete this prefix?")) return;
  const apiUrl = window.API_URL || "http://localhost:3000/api/v1/settings";
  try {
    const res = await fetch(`${apiUrl}/prefixes/${id}`, { method: "DELETE" });
    if (!res.ok) throw new Error(`HTTP ${res.status}`);
    window.prefixes = window.prefixes.filter(p => p.prefix_id !== id);
    await renderTable();
    AppUtils.notify("Delete Prefix", `Prefix ID ${id} deleted`, "warning");
  } catch (err) {
    console.error("Failed to delete prefix:", err);
    AppUtils.notify("Delete Prefix", `Failed: ${err.message}`, "error");
  }
}

// -------------------------
// Add / Save Button
// -------------------------
function initAddPrefixButton() {
  document.getElementById("addPrefixBtn").addEventListener("click", async () => {
    document.getElementById("prefixForm").reset();
    document.getElementById("prefixId").value = "";
    document.getElementById("prefixModalLabel").textContent = "Add Prefix";
    await loadCountries();
    await loadOperators();
    new bootstrap.Modal(document.getElementById("prefixModal")).show();
  });
}

function initSavePrefixButton() {
  document.getElementById("savePrefixBtn").addEventListener("click", async () => {
    const id = document.getElementById("prefixId").value;
    const data = {
      prefix: document.getElementById("prefix").value,
      country_name: document.getElementById("prefixCountry").value,
      operator_name: document.getElementById("prefixOperator").value,
      is_valid: document.getElementById("isValid").checked,
      created_by: "dali",
      updated_by: "dali"
    };

    const apiUrl = window.API_URL || "http://localhost:3000/api/v1/settings";

    try {
      let res, json;
      if (id) {
        res = await fetch(`${apiUrl}/prefixes/${id}`, {
          method: "PUT",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify(data)
        });
        if (!res.ok) throw new Error(`HTTP ${res.status}`);
        json = await res.json();
        const idx = window.prefixes.findIndex(p => p.prefix_id == id);
        window.prefixes[idx] = json;
        AppUtils.notify("Edit Prefix", "Prefix updated", "info");
      } else {
        res = await fetch(`${apiUrl}/prefixes`, {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify(data)
        });
        if (!res.ok) throw new Error(`HTTP ${res.status}`);
        json = await res.json();
        window.prefixes.push(json);
        AppUtils.notify("Add Prefix", "Prefix added", "success");
      }
      await renderTable();
      bootstrap.Modal.getOrCreateInstance(document.getElementById("prefixModal")).hide();
    } catch (err) {
      console.error("Failed to save prefix:", err);
      AppUtils.notify("Save Prefix", `Failed: ${err.message}`, "error");
    }
  });
}

// -------------------------
// Load Countries / Operators
// -------------------------
async function loadCountries() {
  const apiUrl = window.API_URL || "http://localhost:3000/api/v1/settings";
  const res = await fetch(`${apiUrl}/countries`);
  const countries = await res.json();
  const select = document.getElementById("prefixCountry");
  select.innerHTML = '<option value="">Select a country</option>';
  countries.forEach(c => {
    const opt = document.createElement("option");
    opt.value = c.country_name;
    opt.textContent = c.country_name;
    select.appendChild(opt);
  });
}

async function loadOperators(countryName) {
  const apiUrl = window.API_URL || "http://localhost:3000/api/v1/settings";
  const res = await fetch(`${apiUrl}/operators`);
  const ops = await res.json();
  const select = document.getElementById("prefixOperator");
  select.innerHTML = '<option value="">Select an operator</option>';
  ops.filter(o => !countryName || o.country_name === countryName)
     .forEach(o => {
       const opt = document.createElement("option");
       opt.value = o.operator_name;
       opt.textContent = o.operator_name;
       select.appendChild(opt);
     });
}

// -------------------------
// Expose globally
// -------------------------
window.editPrefix = editPrefix;
window.deletePrefix = deletePrefix;
