// ✅ Expose the init function on window
window.prefixesInit = async function() {
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

/**
 * Load prefixes from API
 */
async function loadPrefixes() {
  try {
    const res = await fetch(`${BASE_API}/settings/prefixes`);
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

/**
 * Render table
 */
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

/**
 * Pagination
 */
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

/**
 * Search
 */
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

/**
 * Edit Prefix
 */
async function editPrefix(id) {
  const pfx = window.prefixes.find(p => p.prefix_id === id);
  if (!pfx) return;

  document.getElementById("prefixId").value = pfx.prefix_id;
  document.getElementById("prefix").value = pfx.prefix;
  document.getElementById("isValid").checked = pfx.is_valid;

  // Load countries and set the current country (read-only)
  await loadCountries();
  const countrySelect = document.getElementById("prefixCountry");
  
  // Find the country ID by matching country name
  const countriesRes = await fetch(`${BASE_API}/settings/countries`);
  const countries = await countriesRes.json();
  const country = countries.find(c => c.country_name === pfx.country_name);
  
  if (country) {
    countrySelect.value = country.country_id;
    countrySelect.disabled = true;
    countrySelect.classList.add('bg-light');

    // Load operators for the specific country (read-only)
    await loadOperatorsByCountry(country.country_id);
    
    const operatorSelect = document.getElementById("prefixOperator");
    // Find the operator by matching operator name in the loaded operators
    const operatorsRes = await fetch(`${BASE_API}/settings/operators/country/${country.country_id}`);
    const operators = await operatorsRes.json();
    const operator = operators.find(op => op.operator_name === pfx.operator_name);
    
    if (operator) {
      operatorSelect.value = operator.operator_id;
    } else {
      // Fallback: try to find by name in all operators
      const allOperatorsRes = await fetch(`${BASE_API}/settings/operators`);
      const allOperators = await allOperatorsRes.json();
      const fallbackOperator = allOperators.find(op => op.operator_name === pfx.operator_name);
      if (fallbackOperator) {
        operatorSelect.value = fallbackOperator.operator_id;
      }
    }
    
    operatorSelect.disabled = true;
    operatorSelect.classList.add('bg-light');
  }

  document.getElementById("prefixModalLabel").textContent = "Edit Prefix";
  new bootstrap.Modal(document.getElementById("prefixModal")).show();
}

/**
 * Delete Prefix
 */
async function deletePrefix(id) {
  if (!confirm("Are you sure you want to delete this prefix?")) return;
  
  try {
    const res = await fetch(`${BASE_API}/settings/prefixes/${id}`, { method: "DELETE" });
    if (!res.ok) throw new Error(`HTTP ${res.status}`);
    window.prefixes = window.prefixes.filter(p => p.prefix_id !== id);
    await renderTable();
    AppUtils.notify("Delete Prefix", `Prefix ID ${id} deleted`, "warning");
  } catch (err) {
    console.error("Failed to delete prefix:", err);
    AppUtils.notify("Delete Prefix", `Failed: ${err.message}`, "error");
  }
}

/**
 * Add Prefix Button
 */
function initAddPrefixButton() {
  document.getElementById("addPrefixBtn").addEventListener("click", async () => {
    document.getElementById("prefixForm").reset();
    document.getElementById("prefixId").value = "";
    document.getElementById("prefixModalLabel").textContent = "Add Prefix";

    // Enable country and operator fields for adding
    const countrySelect = document.getElementById("prefixCountry");
    const operatorSelect = document.getElementById("prefixOperator");
    
    if (countrySelect) {
      countrySelect.disabled = false;
      countrySelect.classList.remove('bg-light');
    }
    if (operatorSelect) {
      operatorSelect.disabled = false;
      operatorSelect.classList.remove('bg-light');
    }

    await loadCountries();
    await loadOperatorsByCountry(); // Reset operator dropdown

    new bootstrap.Modal(document.getElementById("prefixModal")).show();
  });
}

/**
 * Save Prefix Button
 */
function initSavePrefixButton() {
  document.getElementById("savePrefixBtn").addEventListener("click", async () => {
    const id = document.getElementById("prefixId").value;
    const prefix = document.getElementById("prefix").value;
    const countryId = document.getElementById("prefixCountry").value;
    const operatorId = document.getElementById("prefixOperator").value;
    const isValid = document.getElementById("isValid").checked;

    if (!prefix || !countryId || !operatorId) {
      AppUtils.notify("Error", "Prefix, Country, and Operator are required", "error");
      return;
    }

    const data = {
      prefix: prefix,
      country_id: parseInt(countryId),
      operator_id: parseInt(operatorId),
      is_valid: isValid,
      created_by: "dali",
      updated_by: "dali"
    };

    try {
      let res, json;
      if (id) {
        res = await fetch(`${BASE_API}/settings/prefixes/${id}`, {
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
        res = await fetch(`${BASE_API}/settings/prefixes`, {
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

/**
 * Load Countries for Dropdown (using country_id instead of country_name)
 */
async function loadCountries() {
  try {
    const res = await fetch(`${BASE_API}/settings/countries`);
    const countries = await res.json();
    const select = document.getElementById("prefixCountry");
    
    // Check if country dropdown exists, if not create it
    if (!select) {
      const operatorField = document.querySelector('label[for="prefixOperator"]').parentElement;
      const countryHtml = `
        <div class="mb-3">
          <label for="prefixCountry" class="form-label">Country *</label>
          <select id="prefixCountry" class="form-control" required>
            <option value="">Select a country</option>
          </select>
        </div>
      `;
      operatorField.insertAdjacentHTML('beforebegin', countryHtml);
    }

    const countrySelect = document.getElementById("prefixCountry");
    countrySelect.innerHTML = '<option value="">Select a country</option>';
    countries.forEach(c => {
      const opt = document.createElement("option");
      opt.value = c.country_id;
      opt.textContent = c.country_name;
      countrySelect.appendChild(opt);
    });

    // Add event listener for country change
    countrySelect.addEventListener("change", function() {
      const countryId = this.value;
      if (countryId) {
        loadOperatorsByCountry(countryId);
      } else {
        const operatorSelect = document.getElementById("prefixOperator");
        if (operatorSelect) {
          operatorSelect.innerHTML = '<option value="">Select a country first</option>';
        }
      }
    });
  } catch (err) {
    console.error("Failed to load countries:", err);
  }
}

/**
 * Load Operators by Country (using operator_id instead of operator_name)
 */
async function loadOperatorsByCountry(countryId = null) {
  try {
    let operators = [];
    
    if (countryId) {
      // Load operators for specific country
      const res = await fetch(`${BASE_API}/settings/operators/country/${countryId}`);
      if (!res.ok) throw new Error(`HTTP ${res.status}`);
      operators = await res.json();
    } else {
      // Reset operator dropdown when no country selected
      const select = document.getElementById("prefixOperator");
      if (select) {
        select.innerHTML = '<option value="">Select a country first</option>';
      }
      return;
    }

    const select = document.getElementById("prefixOperator");
    if (!select) return;
    
    select.innerHTML = '<option value="">Select an operator</option>';
    operators.forEach(op => {
      const opt = document.createElement("option");
      opt.value = op.operator_id;
      opt.textContent = op.operator_name;
      select.appendChild(opt);
    });
  } catch (err) {
    console.error("Failed to load operators by country:", err);
    const select = document.getElementById("prefixOperator");
    if (select) {
      select.innerHTML = '<option value="">Error loading operators</option>';
    }
  }
}

// ✅ Expose globally
window.editPrefix = editPrefix;
window.deletePrefix = deletePrefix;