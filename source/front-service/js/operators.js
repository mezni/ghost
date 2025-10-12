console.log("🎯 operators.js loaded");

window.operatorsInit = async function () {
  console.log("💠 Operators page initialized");

  // Initialize state
  window.operators = [];
  window.filteredOperators = [];
  window.currentPage = 1;
  window.itemsPerPage = 10;

  await loadOperators();
  initPaginationButtons();
  initAddOperatorButton();
  initSaveOperatorButton();
  initSearch();
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
// Load operators from API
// -------------------------
async function loadOperators() {
  const apiUrl = window.API_URL || "http://localhost:3000/api/v1/settings";
  try {
    const res = await fetch(`${apiUrl}/operators`);
    if (!res.ok) throw new Error(`HTTP ${res.status}`);
    window.operators = await res.json();
    window.currentPage = 1;
    await renderTable();
  } catch (err) {
    console.error("❌ Failed to load operators:", err);
    const tbody = await waitForElement("#operatorsTable tbody");
    tbody.innerHTML = `<tr><td colspan="4" class="text-center text-danger">
      Error loading operators: ${err.message}</td></tr>`;
  }
}

// -------------------------
// Render table with pagination
// -------------------------
async function renderTable() {
  const tbody = await waitForElement("#operatorsTable tbody");
  tbody.innerHTML = "";

  const list = (window.filteredOperators && window.filteredOperators.length > 0)
    ? window.filteredOperators
    : window.operators;

  const start = (window.currentPage - 1) * window.itemsPerPage;
  const end = start + window.itemsPerPage;
  const pageItems = list.slice(start, end);

  if (pageItems.length === 0) {
    tbody.innerHTML = `<tr><td colspan="4" class="text-center">No operators found</td></tr>`;
    const infoEl = document.getElementById("paginationInfoOp");
    if (infoEl) infoEl.textContent = "";
    return;
  }

  pageItems.forEach((op, i) => {
    const row = document.createElement("tr");
    row.innerHTML = `
      <td>${start + i + 1}</td>
      <td>${op.operator_name || 'N/A'}</td>
      <td>${op.country_name || 'N/A'}</td>
      <td>
        <button class="btn btn-sm btn-primary me-1" onclick="editOperator(${op.operator_id})">
          <i class="fas fa-edit"></i> Edit
        </button>
        <button class="btn btn-sm btn-danger" onclick="deleteOperator(${op.operator_id})">
          <i class="fas fa-trash"></i> Delete
        </button>
      </td>
    `;
    tbody.appendChild(row);
  });

  const totalPages = Math.ceil(list.length / window.itemsPerPage);
  const infoEl = document.getElementById("paginationInfoOp");
  if (infoEl) infoEl.textContent = `Page ${window.currentPage} of ${totalPages}`;
}

// -------------------------
// Pagination Buttons
// -------------------------
async function initPaginationButtons() {
  const prevBtn = await waitForElement("#prevPageBtnOp");
  const nextBtn = await waitForElement("#nextPageBtnOp");

  prevBtn.addEventListener("click", async () => {
    if (window.currentPage > 1) {
      window.currentPage--;
      await renderTable();
    }
  });

  nextBtn.addEventListener("click", async () => {
    const list = (window.filteredOperators && window.filteredOperators.length > 0)
      ? window.filteredOperators
      : window.operators;

    const totalPages = Math.ceil(list.length / window.itemsPerPage);
    if (window.currentPage < totalPages) {
      window.currentPage++;
      await renderTable();
    }
  });
}

// -------------------------
// Edit / Delete / Add / Search
// -------------------------
function editOperator(id) {
  const op = window.operators.find(o => o.operator_id === id);
  if (!op) return;

  document.getElementById("operatorId").value = op.operator_id;
  document.getElementById("operatorName").value = op.operator_name;
  document.getElementById("operatorCountry").value = op.country_name;

  document.getElementById("operatorModalLabel").textContent = "Edit Operator";

  const modal = new bootstrap.Modal(document.getElementById("operatorModal"));
  modal.show();
}

async function deleteOperator(id) {
  if (!confirm("Are you sure you want to delete this operator?")) return;

  try {
    const apiUrl = window.API_URL || "http://localhost:3000/api/v1/settings";
    const res = await fetch(`${apiUrl}/operators/${id}`, { method: 'DELETE' });
    if (!res.ok) throw new Error(`HTTP ${res.status}`);

    window.operators = window.operators.filter(o => o.operator_id !== id);
    await renderTable();
    AppUtils.notify("Delete Operator", `Deleted operator ID ${id}`, "warning");
  } catch (err) {
    console.error("Failed to delete operator:", err);
    AppUtils.notify("Delete Operator", `Failed to delete operator ID ${id}`, "error");
  }
}

function initAddOperatorButton() {
  waitForElement("#addOperatorBtn").then(btn => {
    btn.addEventListener("click", () => {
      document.getElementById("operatorForm").reset();
      document.getElementById("operatorId").value = "";
      document.getElementById("operatorModalLabel").textContent = "Add Operator";

      const modal = new bootstrap.Modal(document.getElementById("operatorModal"));
      modal.show();
    });
  });
}

function initSaveOperatorButton() {
  waitForElement("#saveOperatorBtn").then(btn => {
    btn.addEventListener("click", async () => {
      const id = document.getElementById("operatorId").value;
      const name = document.getElementById("operatorName").value;
      const country = document.getElementById("operatorCountry").value;

      if (!name || !country) {
        AppUtils.notify("Error", "All fields are required", "error");
        return;
      }

      if (id) await updateOperator(id, name, country);
      else await addOperator(name, country);
    });
  });
}

async function updateOperator(id, name, country) {
  try {
    const apiUrl = window.API_URL || "http://localhost:3000/api/v1/settings";
    const res = await fetch(`${apiUrl}/operators/${id}`, {
      method: "PUT",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ operator_name: name, country_name: country })
    });
    if (!res.ok) throw new Error(`HTTP ${res.status}`);

    const updatedOp = await res.json();
    const idx = window.operators.findIndex(o => o.operator_id == id);
    window.operators[idx] = updatedOp;

    AppUtils.notify("Edit Operator", "Operator updated", "info");
    await renderTable();

    const modalEl = document.getElementById("operatorModal");
    bootstrap.Modal.getOrCreateInstance(modalEl).hide();
  } catch (err) {
    console.error("Failed to update operator:", err);
    AppUtils.notify("Edit Operator", `Failed: ${err.message}`, "error");
  }
}

async function addOperator(name, country) {
  try {
    const apiUrl = window.API_URL || "http://localhost:3000/api/v1/settings";
    const res = await fetch(`${apiUrl}/operators`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ operator_name: name, country_name: country, created_by: "dali" })
    });
    if (!res.ok) {
      const errText = await res.text();
      throw new Error(`HTTP ${res.status}: ${errText}`);
    }

    const newOp = await res.json();
    window.operators.push(newOp);

    AppUtils.notify("Add Operator", "Operator added", "success");
    await renderTable();

    const modalEl = document.getElementById("operatorModal");
    bootstrap.Modal.getOrCreateInstance(modalEl).hide();
  } catch (err) {
    console.error("Failed to add operator:", err);
    AppUtils.notify("Add Operator", `Failed: ${err.message}`, "error");
  }
}

function initSearch() {
  waitForElement("#operatorSearch").then(input => {
    input.addEventListener("input", () => {
      const query = input.value.toLowerCase();
      window.filteredOperators = window.operators.filter(o => 
        (o.operator_name && o.operator_name.toLowerCase().includes(query)) ||
        (o.country_name && o.country_name.toLowerCase().includes(query))
      );
      window.currentPage = 1;
      renderTable();
    });
  });
}

// -------------------------
// Expose globally
// -------------------------
window.editOperator = editOperator;
window.deleteOperator = deleteOperator;
