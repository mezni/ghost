// ✅ Expose the init function on window
window.sorplansInit = async function() {
  console.log("💠 SOR Plan page initialized");

  window.sorPlans = [];
  window.filteredSorPlans = [];
  window.currentPageSP = 1;
  window.itemsPerPageSP = 10;

  try {
    await loadSorPlans();
    await waitForElement("#prevPageBtnSP"); // ensure DOM is ready
    initPaginationButtonsSP();
    initAddSorPlanButton();
    initSaveSorPlanButton();
    initSearchSP();
  } catch (err) {
    console.error("❌ Failed to init SOR plans page:", err);
  }
};

/**
 * Utility to wait for an element to exist
 */
function waitForElement(selector, timeout = 2000) {
  return new Promise((resolve, reject) => {
    const el = document.querySelector(selector);
    if (el) return resolve(el);

    const observer = new MutationObserver(() => {
      const el = document.querySelector(selector);
      if (el) {
        observer.disconnect();
        resolve(el);
      }
    });

    observer.observe(document.body, { childList: true, subtree: true });

    setTimeout(() => {
      observer.disconnect();
      reject(new Error(`Element ${selector} not found in DOM`));
    }, timeout);
  });
}

/**
 * Load SOR Plans from API
 */
async function loadSorPlans() {
  try {
    const res = await fetch(`${BASE_API}/settings/sor_plan`);
    if (!res.ok) throw new Error(`HTTP ${res.status}`);
    window.sorPlans = await res.json() || [];
    window.currentPageSP = 1;
    renderSorPlanTable();
  } catch (err) {
    console.error("❌ Failed to load SOR plans:", err);
    window.sorPlans = [];
    const tbody = document.querySelector("#sorPlanTable tbody");
    if (tbody) {
      tbody.innerHTML = `<tr><td colspan="7" class="text-center text-danger">
        Error loading SOR plans: ${err.message}</td></tr>`;
    }
  }
}

/**
 * Render table with pagination
 */
function renderSorPlanTable() {
  const tbody = document.querySelector("#sorPlanTable tbody");
  if (!tbody) {
    console.warn("⚠️ sorPlanTable tbody not found in DOM");
    return;
  }

  tbody.innerHTML = "";

  const list = (window.filteredSorPlans && window.filteredSorPlans.length > 0)
    ? window.filteredSorPlans
    : (window.sorPlans || []);

  const start = (window.currentPageSP - 1) * window.itemsPerPageSP;
  const end = start + window.itemsPerPageSP;
  const pageItems = list.slice(start, end);

  if (pageItems.length === 0) {
    tbody.innerHTML = `<tr><td colspan="7" class="text-center">No SOR plans found</td></tr>`;
    const infoEl = document.getElementById("paginationInfoSP");
    if (infoEl) infoEl.textContent = "";
    return;
  }

  pageItems.forEach((sp, i) => {
    const row = document.createElement("tr");
    row.innerHTML = `
      <td>${start + i + 1}</td>
      <td>${sp.country_name || 'N/A'}</td>
      <td>${sp.operator_name || 'N/A'}</td>
      <td>${sp.routage_type_name || 'N/A'}</td>
      <td>${sp.barring ? "✅" : "❌"}</td>
      <td>${sp.rate || 'N/A'}</td>
      <td>
        <button class="btn btn-sm btn-primary me-1" onclick="editSorPlan(${sp.sor_plan_id})">
          <i class="fas fa-edit"></i> Edit
        </button>
        <button class="btn btn-sm btn-danger" onclick="deleteSorPlan(${sp.sor_plan_id})">
          <i class="fas fa-trash"></i> Delete
        </button>
      </td>
    `;
    tbody.appendChild(row);
  });

  const totalPages = Math.ceil(list.length / window.itemsPerPageSP);
  const infoEl = document.getElementById("paginationInfoSP");
  if (infoEl) infoEl.textContent = `Page ${window.currentPageSP} of ${totalPages}`;
}

/**
 * Pagination Buttons
 */
function initPaginationButtonsSP() {
  const prevBtn = document.getElementById("prevPageBtnSP");
  const nextBtn = document.getElementById("nextPageBtnSP");

  if (!prevBtn || !nextBtn) return;

  prevBtn.addEventListener("click", () => {
    if (window.currentPageSP > 1) {
      window.currentPageSP--;
      renderSorPlanTable();
    }
  });

  nextBtn.addEventListener("click", () => {
    const list = (window.filteredSorPlans && window.filteredSorPlans.length > 0)
      ? window.filteredSorPlans
      : (window.sorPlans || []);
    const totalPages = Math.ceil(list.length / window.itemsPerPageSP);
    if (window.currentPageSP < totalPages) {
      window.currentPageSP++;
      renderSorPlanTable();
    }
  });
}

/**
 * Edit SOR Plan
 */
async function editSorPlan(id) {
  const sp = window.sorPlans.find(c => c.sor_plan_id === id);
  if (!sp) return;

  document.getElementById("sorPlanId").value = sp.sor_plan_id;
  document.getElementById("sorPlanRate").value = sp.rate;
  document.getElementById("sorPlanBarring").checked = sp.barring;
  document.getElementById("sorPlanCurrent").checked = sp.is_current;

  // First, let's get all operators to find the country for this operator
  try {
    const operatorsRes = await fetch(`${BASE_API}/settings/operators`);
    if (operatorsRes.ok) {
      const allOperators = await operatorsRes.json();
      const currentOperator = allOperators.find(op => op.operator_id === sp.operator_id);
      
      if (currentOperator) {
        // Load countries and set the current country (read-only)
        await loadCountriesSP();
        const countrySelect = document.getElementById("sorPlanCountry");
        countrySelect.value = currentOperator.country_id;
        countrySelect.disabled = true; // Make country read-only
        
        // Now load operators for this specific country (read-only)
        await loadOperatorsByCountrySP(currentOperator.country_id);
        const operatorSelect = document.getElementById("sorPlanOperator");
        operatorSelect.value = sp.operator_id;
        operatorSelect.disabled = true; // Make operator read-only
        
        // Add visual indication that these fields are read-only
        countrySelect.classList.add('bg-light');
        operatorSelect.classList.add('bg-light');
      }
    }
  } catch (err) {
    console.error("Failed to load operator data for edit:", err);
  }

  // Set routage type (editable)
  loadPredefinedRoutageTypesSP();
  document.getElementById("sorPlanRoutage").value = sp.routage_type_id || "";

  document.getElementById("sorPlanModalLabel").textContent = "Edit SOR Plan";

  const modal = new bootstrap.Modal(document.getElementById("sorPlanModal"));
  modal.show();
}

/**
 * Delete SOR Plan
 */
async function deleteSorPlan(id) {
  if (!confirm("Are you sure you want to delete this SOR plan?")) return;

  try {
    const res = await fetch(`${BASE_API}/settings/sor_plan/${id}`, { method: 'DELETE' });
    if (!res.ok) throw new Error(`HTTP ${res.status}`);

    window.sorPlans = window.sorPlans.filter(c => c.sor_plan_id !== id);
    renderSorPlanTable();
    AppUtils.notify("Delete SOR Plan", `Deleted SOR plan ID ${id}`, "warning");
  } catch (err) {
    console.error("Failed to delete SOR plan:", err);
    AppUtils.notify("Delete SOR Plan", `Failed to delete SOR plan ID ${id}`, "error");
  }
}

/**
 * Add / Save SOR Plan Buttons
 */
function initAddSorPlanButton() {
  const btn = document.getElementById("addSorPlanBtn");
  if (!btn) return;

  btn.addEventListener("click", () => {
    document.getElementById("sorPlanForm").reset();
    document.getElementById("sorPlanId").value = "";
    document.getElementById("sorPlanModalLabel").textContent = "Add SOR Plan";

    // Enable country and operator fields for adding
    const countrySelect = document.getElementById("sorPlanCountry");
    const operatorSelect = document.getElementById("sorPlanOperator");
    
    if (countrySelect) {
      countrySelect.disabled = false;
      countrySelect.classList.remove('bg-light');
    }
    if (operatorSelect) {
      operatorSelect.disabled = false;
      operatorSelect.classList.remove('bg-light');
    }

    // Load countries and predefined routage types
    loadCountriesSP();
    loadPredefinedRoutageTypesSP();

    // Reset operator dropdown
    if (operatorSelect) {
      operatorSelect.innerHTML = '<option value="">Select a country first</option>';
    }

    const modal = new bootstrap.Modal(document.getElementById("sorPlanModal"));
    modal.show();
  });
}

function initSaveSorPlanButton() {
  const btn = document.getElementById("saveSorPlanBtn");
  if (!btn) return;

  btn.addEventListener("click", async () => {
    const id = document.getElementById("sorPlanId").value;
    const operatorId = document.getElementById("sorPlanOperator").value;
    const routageTypeId = document.getElementById("sorPlanRoutage").value;
    const rate = document.getElementById("sorPlanRate").value;
    const barring = document.getElementById("sorPlanBarring").checked;
    const isCurrent = document.getElementById("sorPlanCurrent").checked;

    if (!operatorId || !routageTypeId || !rate) {
      AppUtils.notify("Error", "Operator, Routage Type, and Rate are required", "error");
      return;
    }

    if (id) await updateSorPlan(id, operatorId, routageTypeId, rate, barring, isCurrent);
    else await addSorPlan(operatorId, routageTypeId, rate, barring, isCurrent);
  });
}

/**
 * Update / Add SOR Plan API
 */
async function updateSorPlan(id, operatorId, routageTypeId, rate, barring, isCurrent) {
  try {
    const res = await fetch(`${BASE_API}/settings/sor_plan/${id}`, {
      method: "PUT",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ 
        operator_id: parseInt(operatorId),
        routage_type_id: routageTypeId ? parseInt(routageTypeId) : null,
        rate: rate,
        barring: barring,
        is_current: isCurrent,
        updated_by: "dali"
      })
    });
    if (!res.ok) throw new Error(`HTTP ${res.status}`);

    const updatedSorPlan = await res.json();
    const idx = window.sorPlans.findIndex(c => c.sor_plan_id == id);
    window.sorPlans[idx] = updatedSorPlan;

    AppUtils.notify("Edit SOR Plan", "SOR Plan updated", "info");
    renderSorPlanTable();

    const modalEl = document.getElementById("sorPlanModal");
    bootstrap.Modal.getOrCreateInstance(modalEl).hide();
  } catch (err) {
    console.error("Failed to update SOR plan:", err);
    AppUtils.notify("Edit SOR Plan", `Failed: ${err.message}`, "error");
  }
}

async function addSorPlan(operatorId, routageTypeId, rate, barring, isCurrent) {
  try {
    const res = await fetch(`${BASE_API}/settings/sor_plan`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ 
        operator_id: parseInt(operatorId),
        routage_type_id: routageTypeId ? parseInt(routageTypeId) : null,
        rate: rate,
        barring: barring,
        is_current: isCurrent,
        created_by: "dali"
      })
    });
    if (!res.ok) throw new Error(`HTTP ${res.status}`);

    const newSorPlan = await res.json();
    window.sorPlans.push(newSorPlan);

    AppUtils.notify("Add SOR Plan", "SOR Plan added", "success");
    renderSorPlanTable();

    const modalEl = document.getElementById("sorPlanModal");
    bootstrap.Modal.getOrCreateInstance(modalEl).hide();
  } catch (err) {
    console.error("Failed to add SOR plan:", err);
    AppUtils.notify("Add SOR Plan", `Failed: ${err.message}`, "error");
  }
}

/**
 * Search
 */
function initSearchSP() {
  const searchInput = document.getElementById("sorPlanSearch");
  if (!searchInput) return;

  searchInput.addEventListener("input", () => {
    const query = searchInput.value.toLowerCase();
    window.filteredSorPlans = window.sorPlans.filter(c =>
      (c.country_name && c.country_name.toLowerCase().includes(query)) ||
      (c.operator_name && c.operator_name.toLowerCase().includes(query)) ||
      (c.routage_type_name && c.routage_type_name.toLowerCase().includes(query)) ||
      (c.rate && c.rate.toLowerCase().includes(query))
    );
    window.currentPageSP = 1;
    renderSorPlanTable();
  });
}

/**
 * Load Countries for Dropdown - Add country dropdown dynamically
 */
async function loadCountriesSP() {
  try {
    const res = await fetch(`${BASE_API}/settings/countries`);
    if (!res.ok) throw new Error(`HTTP ${res.status}`);
    const countries = await res.json();
    
    // Check if country dropdown already exists, if not create it
    let countrySelect = document.getElementById("sorPlanCountry");
    const operatorSelect = document.getElementById("sorPlanOperator");
    
    if (!countrySelect) {
      // Create country dropdown and insert it before the operator dropdown
      const operatorLabel = document.querySelector('label[for="sorPlanOperator"]');
      const operatorField = operatorSelect.parentElement;
      
      const countryHtml = `
        <div class="mb-3">
          <label for="sorPlanCountry" class="form-label">Country *</label>
          <select id="sorPlanCountry" class="form-control" required>
            <option value="">Select a country</option>
          </select>
        </div>
      `;
      
      operatorField.insertAdjacentHTML('beforebegin', countryHtml);
      countrySelect = document.getElementById("sorPlanCountry");
    }
    
    // Populate countries
    countrySelect.innerHTML = '<option value="">Select a country</option>';
    countries.forEach(country => {
      const opt = document.createElement("option");
      opt.value = country.country_id;
      opt.textContent = country.country_name;
      countrySelect.appendChild(opt);
    });

    // Add event listener for country change
    countrySelect.addEventListener("change", function() {
      const countryId = this.value;
      if (countryId) {
        loadOperatorsByCountrySP(countryId);
      } else {
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
 * Load Operators by Country
 */
async function loadOperatorsByCountrySP(countryId) {
  try {
    const res = await fetch(`${BASE_API}/settings/operators/country/${countryId}`);
    if (!res.ok) throw new Error(`HTTP ${res.status}`);
    const operators = await res.json();
    const select = document.getElementById("sorPlanOperator");
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
    const select = document.getElementById("sorPlanOperator");
    if (select) {
      select.innerHTML = '<option value="">Error loading operators</option>';
    }
  }
}

/**
 * Load Predefined Routage Types
 */
function loadPredefinedRoutageTypesSP() {
  const routageTypes = [
    { routage_type_id: 1, routage_type_name: 'Bilateral' },
    { routage_type_id: 2, routage_type_name: 'Orange Hub' },
    { routage_type_id: 3, routage_type_name: 'Comfone' },
    { routage_type_id: 4, routage_type_name: 'N/A' }
  ];

  const select = document.getElementById("sorPlanRoutage");
  if (!select) return;

  select.innerHTML = '<option value="">Select a routage type</option>';
  routageTypes.forEach(rt => {
    const opt = document.createElement("option");
    opt.value = rt.routage_type_id;
    opt.textContent = rt.routage_type_name;
    select.appendChild(opt);
  });
}

// ✅ Expose globally
window.editSorPlan = editSorPlan;
window.deleteSorPlan = deleteSorPlan;