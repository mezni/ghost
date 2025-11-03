console.log("🎯 sorplans.js loaded");

window.sorplansInit = async function () {
  console.log("💠 SOR Plan page initialized");

  window.sorPlans = [];
  window.filteredSorPlans = [];
  window.currentPageSP = 1;
  window.itemsPerPageSP = 10;

  await loadSorPlans();
  initPaginationButtonsSP();
  initAddSorPlanButton();
  initSaveSorPlanButton();
  initSearchSP();
};

// -------------------------
// Load SOR Plans from API
// -------------------------
async function loadSorPlans() {
  const apiUrl = window.API_URL || "http://localhost:3000/api/v1/settings";
  try {
    const res = await fetch(`${apiUrl}/sor_plan`);
    if (!res.ok) throw new Error(`HTTP ${res.status}`);
    window.sorPlans = await res.json();
    window.currentPageSP = 1;
    await renderSorPlanTable();
  } catch (err) {
    console.error("❌ Failed to load SOR plans:", err);
    const tbody = document.querySelector("#sorPlanTable tbody");
    tbody.innerHTML = `<tr><td colspan="7" class="text-center text-danger">
      Error loading SOR plans: ${err.message}</td></tr>`;
  }
}

// -------------------------
// Render table
// -------------------------
async function renderSorPlanTable() {
  const tbody = document.querySelector("#sorPlanTable tbody");
  tbody.innerHTML = "";

  const list = (window.filteredSorPlans && window.filteredSorPlans.length > 0)
    ? window.filteredSorPlans
    : window.sorPlans;

  const start = (window.currentPageSP - 1) * window.itemsPerPageSP;
  const end = start + window.itemsPerPageSP;
  const pageItems = list.slice(start, end);

  if (pageItems.length === 0) {
    tbody.innerHTML = `<tr><td colspan="7" class="text-center">No SOR plans found</td></tr>`;
    document.getElementById("paginationInfoSP").textContent = "";
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
  document.getElementById("paginationInfoSP").textContent = `Page ${window.currentPageSP} of ${totalPages}`;
}

// -------------------------
// Pagination
// -------------------------
function initPaginationButtonsSP() {
  document.getElementById("prevPageBtnSP").addEventListener("click", async () => {
    if (window.currentPageSP > 1) { window.currentPageSP--; await renderSorPlanTable(); }
  });
  document.getElementById("nextPageBtnSP").addEventListener("click", async () => {
    const list = window.filteredSorPlans.length > 0 ? window.filteredSorPlans : window.sorPlans;
    const totalPages = Math.ceil(list.length / window.itemsPerPageSP);
    if (window.currentPageSP < totalPages) { window.currentPageSP++; await renderSorPlanTable(); }
  });
}

// -------------------------
// Search
// -------------------------
function initSearchSP() {
  document.getElementById("sorPlanSearch").addEventListener("input", (e) => {
    const query = e.target.value.toLowerCase();
    window.filteredSorPlans = window.sorPlans.filter(sp =>
      (sp.country_name && sp.country_name.toLowerCase().includes(query)) ||
      (sp.operator_name && sp.operator_name.toLowerCase().includes(query)) ||
      (sp.routage_type_name && sp.routage_type_name.toLowerCase().includes(query)) ||
      (sp.rate && sp.rate.toLowerCase().includes(query))
    );
    window.currentPageSP = 1;
    renderSorPlanTable();
  });
}

// -------------------------
// Edit / Delete / Add
// -------------------------
async function editSorPlan(id) {
  const sp = window.sorPlans.find(n => n.sor_plan_id === id);
  if (!sp) return;

  document.getElementById("sorPlanId").value = sp.sor_plan_id;
  document.getElementById("sorPlanRate").value = sp.rate;
  document.getElementById("sorPlanBarring").checked = sp.barring;
  document.getElementById("sorPlanCurrent").checked = sp.is_current;

  await loadOperatorsSP();
  document.getElementById("sorPlanOperator").value = sp.operator_id;

  await loadRoutageTypesSP();
  document.getElementById("sorPlanRoutage").value = sp.routage_type_id;

  document.getElementById("sorPlanModalLabel").textContent = "Edit SOR Plan";
  new bootstrap.Modal(document.getElementById("sorPlanModal")).show();
}

async function deleteSorPlan(id) {
  if (!confirm("Are you sure you want to delete this SOR plan?")) return;
  const apiUrl = window.API_URL || "http://localhost:3000/api/v1/settings";
  try {
    const res = await fetch(`${apiUrl}/sor_plan/${id}`, { method: "DELETE" });
    if (!res.ok) throw new Error(`HTTP ${res.status}`);
    window.sorPlans = window.sorPlans.filter(n => n.sor_plan_id !== id);
    await renderSorPlanTable();
    AppUtils.notify("Delete SOR Plan", `SOR Plan ID ${id} deleted`, "warning");
  } catch (err) {
    console.error("Failed to delete SOR plan:", err);
    AppUtils.notify("Delete SOR Plan", `Failed: ${err.message}`, "error");
  }
}
