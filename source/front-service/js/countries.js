console.log("🎯 countries.js loaded");

window.countriesInit = async function() {
  console.log("💠 Countries page initialized");

  // Initialize state
  window.countries = [];
  window.filteredCountries = [];
  window.currentPage = 1;
  window.itemsPerPage = 10;

  await loadCountries();
  initPaginationButtons();
  initAddCountryButton();
  initSaveCountryButton();
  initSearch();
};

// -------------------------
// Load countries from API
// -------------------------
async function loadCountries() {
  const apiUrl = window.API_URL || "http://localhost:3000/api/v1";
  try {
    const res = await fetch(`${apiUrl}/countries`);
    if (!res.ok) throw new Error(`HTTP ${res.status}`);
    window.countries = await res.json();
    window.currentPage = 1;
    renderTable();
  } catch (err) {
    console.error("❌ Failed to load countries:", err);
    const tbody = document.querySelector("#countriesTable tbody");
    if (tbody) {
      tbody.innerHTML = `<tr><td colspan="4" class="text-center text-danger">
        Error loading countries: ${err.message}</td></tr>`;
    }
  }
}

// -------------------------
// Render table with pagination
// -------------------------
function renderTable() {
  const tbody = document.querySelector("#countriesTable tbody");
  tbody.innerHTML = "";

  const list = window.filteredCountries && window.filteredCountries.length > 0
    ? window.filteredCountries
    : window.countries;

  const start = (window.currentPage - 1) * window.itemsPerPage;
  const end = start + window.itemsPerPage;
  const pageItems = list.slice(start, end);

  if (pageItems.length === 0) {
    tbody.innerHTML = `<tr><td colspan="4" class="text-center">No countries found</td></tr>`;
    document.getElementById("paginationInfo").textContent = "";
    return;
  }

  pageItems.forEach((c, i) => {
    const row = document.createElement("tr");
    row.innerHTML = `
      <td>${start + i + 1}</td>
      <td>${c.iso_code || 'N/A'}</td>
      <td>${c.country_name || 'N/A'}</td>
      <td>
        <button class="btn btn-sm btn-primary me-1" onclick="editCountry(${c.country_id})">
          <i class="fas fa-edit"></i> Edit
        </button>
        <button class="btn btn-sm btn-danger" onclick="deleteCountry(${c.country_id})">
          <i class="fas fa-trash"></i> Delete
        </button>
      </td>
    `;
    tbody.appendChild(row);
  });

  const totalPages = Math.ceil(list.length / window.itemsPerPage);
  document.getElementById("paginationInfo").textContent =
    `Page ${window.currentPage} of ${totalPages}`;
}

// -------------------------
// Pagination Buttons
// -------------------------
function initPaginationButtons() {
  document.getElementById("prevPageBtn").addEventListener("click", () => {
    if (window.currentPage > 1) {
      window.currentPage--;
      renderTable();
    }
  });

  document.getElementById("nextPageBtn").addEventListener("click", () => {
    const list = window.filteredCountries && window.filteredCountries.length > 0
      ? window.filteredCountries
      : window.countries;

    const totalPages = Math.ceil(list.length / window.itemsPerPage);
    if (window.currentPage < totalPages) {
      window.currentPage++;
      renderTable();
    }
  });
}

// -------------------------
// Edit Country
// -------------------------
function editCountry(id) {
  const country = window.countries.find(c => c.country_id === id);
  if (!country) return;

  document.getElementById("countryId").value = country.country_id;
  document.getElementById("countryCode").value = country.iso_code;
  document.getElementById("countryName").value = country.country_name;

  // Make ISO code read-only when editing
  document.getElementById("countryCode").readOnly = true;

  document.getElementById("countryModalLabel").textContent = "Edit Country";

  const modal = new bootstrap.Modal(document.getElementById("countryModal"));
  modal.show();
}

// -------------------------
// Delete Country
// -------------------------
async function deleteCountry(id) {
  if (confirm("Are you sure you want to delete this country?")) {
    try {
      const apiUrl = window.API_URL || "http://localhost:3000/api/v1";
      const res = await fetch(`${apiUrl}/countries/${id}`, {
        method: 'DELETE'
      });
      if (!res.ok) throw new Error(`HTTP ${res.status}`);

      window.countries = window.countries.filter(c => c.country_id !== id);
      renderTable();
      AppUtils.notify("Delete Country", `Deleted country ID ${id}`, "warning");
    } catch (err) {
      console.error("Failed to delete country:", err);
      AppUtils.notify("Delete Country", `Failed to delete country ID ${id}`, "error");
    }
  }
}

// -------------------------
// Add Country Button
// -------------------------
function initAddCountryButton() {
  document.getElementById("addCountryBtn").addEventListener("click", () => {
    document.getElementById("countryForm").reset();
    document.getElementById("countryId").value = "";
    
    // ISO code editable when adding
    document.getElementById("countryCode").readOnly = false;

    document.getElementById("countryModalLabel").textContent = "Add Country";
    const modal = new bootstrap.Modal(document.getElementById("countryModal"));
    modal.show();
  });
}

// -------------------------
// Save Country Button
// -------------------------
async function initSaveCountryButton() {
  document.getElementById("saveCountryBtn").addEventListener("click", async () => {
    const id = document.getElementById("countryId").value;
    const code = document.getElementById("countryCode").value;
    const name = document.getElementById("countryName").value;

    if (!code || !name) {
      AppUtils.notify("Error", "All fields are required", "error");
      return;
    }

    if (id) {
      await updateCountry(id, code, name);
    } else {
      await addCountry(code, name);
    }
  });
}

async function updateCountry(id, code, name) {
  try {
    const apiUrl = window.API_URL || "http://localhost:3000/api/v1";
    const res = await fetch(`${apiUrl}/countries/${id}`, {
      method: "PUT",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ country_name: name })
    });
    if (!res.ok) throw new Error(`HTTP ${res.status}`);

    const updatedCountry = await res.json();
    const idx = window.countries.findIndex(c => c.country_id == id);
    window.countries[idx] = updatedCountry;

    AppUtils.notify("Edit Country", "Country updated", "info");
    renderTable();

    // Close modal
    const modalEl = document.getElementById("countryModal");
    const modal = bootstrap.Modal.getOrCreateInstance
      ? bootstrap.Modal.getOrCreateInstance(modalEl)
      : new bootstrap.Modal(modalEl); // fallback
    modal.hide();
  } catch (err) {
    console.error("Failed to update country:", err);
    AppUtils.notify("Edit Country", `Failed: ${err.message}`, "error");
  }
}

async function addCountry(code, name) {
  try {
    const apiUrl = window.API_URL || "http://localhost:3000/api/v1";
    const res = await fetch(`${apiUrl}/countries`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ iso_code: code, country_name: name, created_by: "dali" })
    });
    if (!res.ok) {
      const errText = await res.text();
      throw new Error(`HTTP ${res.status}: ${errText}`);
    }

    const newCountry = await res.json();
    window.countries.push(newCountry);

    AppUtils.notify("Add Country", "Country added", "success");
    renderTable();

    // Close modal
    const modalEl = document.getElementById("countryModal");
    const modal = bootstrap.Modal.getOrCreateInstance
      ? bootstrap.Modal.getOrCreateInstance(modalEl)
      : new bootstrap.Modal(modalEl); // fallback
    modal.hide();
  } catch (err) {
    console.error("Failed to add country:", err);
    AppUtils.notify("Add Country", `Failed: ${err.message}`, "error");
  }
}

// -------------------------
// Search Countries
// -------------------------
function initSearch() {
  const searchInput = document.getElementById("countrySearch");
  searchInput.addEventListener("input", () => {
    const query = searchInput.value.toLowerCase();
    window.filteredCountries = window.countries.filter(c => {
      return (
        (c.iso_code && c.iso_code.toLowerCase().includes(query)) ||
        (c.country_name && c.country_name.toLowerCase().includes(query))
      );
    });
    window.currentPage = 1;
    renderTable();
  });
}

// -------------------------
// Expose globally
// -------------------------
window.editCountry = editCountry;
window.deleteCountry = deleteCountry;
