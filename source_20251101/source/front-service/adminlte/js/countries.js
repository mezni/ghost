console.log("🎯 countries.js loaded");

// Utility to wait for an element to exist
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

// -------------------------
// Initialize Countries Page
// -------------------------
window.countriesInit = async function () {
  console.log("💠 Countries page initialized");

  window.countries = [];
  window.filteredCountries = [];
  window.currentPage = 1;
  window.itemsPerPage = 10;

  try {
    await loadCountries();
    await waitForElement("#prevPageBtn"); // ensure DOM is ready
    initPaginationButtons();
    initAddCountryButton();
    initSaveCountryButton();
    initSearch();
  } catch (err) {
    console.error("❌ Failed to init countries page:", err);
  }
};

// -------------------------
// Load countries from API
// -------------------------
async function loadCountries() {
  const apiUrl = window.API_URL || "http://localhost:3000/api/v1/settings";
  try {
    const res = await fetch(`${apiUrl}/countries`);
    if (!res.ok) throw new Error(`HTTP ${res.status}`);
    window.countries = await res.json() || [];
    window.currentPage = 1;
    renderTable();
  } catch (err) {
    console.error("❌ Failed to load countries:", err);
    window.countries = [];
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
  if (!tbody) {
    console.warn("⚠️ countriesTable tbody not found in DOM");
    return;
  }

  tbody.innerHTML = "";

  const list = (window.filteredCountries && window.filteredCountries.length > 0)
    ? window.filteredCountries
    : (window.countries || []);

  const start = (window.currentPage - 1) * window.itemsPerPage;
  const end = start + window.itemsPerPage;
  const pageItems = list.slice(start, end);

  if (pageItems.length === 0) {
    tbody.innerHTML = `<tr><td colspan="4" class="text-center">No countries found</td></tr>`;
    const infoEl = document.getElementById("paginationInfo");
    if (infoEl) infoEl.textContent = "";
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
  const infoEl = document.getElementById("paginationInfo");
  if (infoEl) infoEl.textContent = `Page ${window.currentPage} of ${totalPages}`;
}

// -------------------------
// Pagination Buttons
// -------------------------
function initPaginationButtons() {
  const prevBtn = document.getElementById("prevPageBtn");
  const nextBtn = document.getElementById("nextPageBtn");

  if (!prevBtn || !nextBtn) return;

  prevBtn.addEventListener("click", () => {
    if (window.currentPage > 1) {
      window.currentPage--;
      renderTable();
    }
  });

  nextBtn.addEventListener("click", () => {
    const list = (window.filteredCountries && window.filteredCountries.length > 0)
      ? window.filteredCountries
      : (window.countries || []);
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

  document.getElementById("countryCode").readOnly = true;
  document.getElementById("countryModalLabel").textContent = "Edit Country";

  const modal = new bootstrap.Modal(document.getElementById("countryModal"));
  modal.show();
}

// -------------------------
// Delete Country
// -------------------------
async function deleteCountry(id) {
  if (!confirm("Are you sure you want to delete this country?")) return;

  try {
    const apiUrl = window.API_URL || "http://localhost:3000/api/v1/settings";
    const res = await fetch(`${apiUrl}/countries/${id}`, { method: 'DELETE' });
    if (!res.ok) throw new Error(`HTTP ${res.status}`);

    window.countries = window.countries.filter(c => c.country_id !== id);
    renderTable();
    AppUtils.notify("Delete Country", `Deleted country ID ${id}`, "warning");
  } catch (err) {
    console.error("Failed to delete country:", err);
    AppUtils.notify("Delete Country", `Failed to delete country ID ${id}`, "error");
  }
}

// -------------------------
// Add / Save Country Buttons
// -------------------------
function initAddCountryButton() {
  const btn = document.getElementById("addCountryBtn");
  if (!btn) return;

  btn.addEventListener("click", () => {
    document.getElementById("countryForm").reset();
    document.getElementById("countryId").value = "";
    document.getElementById("countryCode").readOnly = false;
    document.getElementById("countryModalLabel").textContent = "Add Country";

    const modal = new bootstrap.Modal(document.getElementById("countryModal"));
    modal.show();
  });
}

function initSaveCountryButton() {
  const btn = document.getElementById("saveCountryBtn");
  if (!btn) return;

  btn.addEventListener("click", async () => {
    const id = document.getElementById("countryId").value;
    const code = document.getElementById("countryCode").value;
    const name = document.getElementById("countryName").value;

    if (!code || !name) {
      AppUtils.notify("Error", "All fields are required", "error");
      return;
    }

    if (id) await updateCountry(id, name);
    else await addCountry(code, name);
  });
}

// -------------------------
// Update / Add Country API
// -------------------------
async function updateCountry(id, name) {
  try {
    const apiUrl = window.API_URL || "http://localhost:3000/api/v1/settings";
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

    const modalEl = document.getElementById("countryModal");
    bootstrap.Modal.getOrCreateInstance(modalEl).hide();
  } catch (err) {
    console.error("Failed to update country:", err);
    AppUtils.notify("Edit Country", `Failed: ${err.message}`, "error");
  }
}

async function addCountry(code, name) {
  try {
    const apiUrl = window.API_URL || "http://localhost:3000/api/v1/settings";
    const res = await fetch(`${apiUrl}/countries`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ iso_code: code, country_name: name, created_by: "dali" })
    });
    if (!res.ok) throw new Error(`HTTP ${res.status}`);

    const newCountry = await res.json();
    window.countries.push(newCountry);

    AppUtils.notify("Add Country", "Country added", "success");
    renderTable();

    const modalEl = document.getElementById("countryModal");
    bootstrap.Modal.getOrCreateInstance(modalEl).hide();
  } catch (err) {
    console.error("Failed to add country:", err);
    AppUtils.notify("Add Country", `Failed: ${err.message}`, "error");
  }
}

// -------------------------
// Search
// -------------------------
function initSearch() {
  const searchInput = document.getElementById("countrySearch");
  if (!searchInput) return;

  searchInput.addEventListener("input", () => {
    const query = searchInput.value.toLowerCase();
    window.filteredCountries = window.countries.filter(c =>
      (c.iso_code && c.iso_code.toLowerCase().includes(query)) ||
      (c.country_name && c.country_name.toLowerCase().includes(query))
    );
    window.currentPage = 1;
    renderTable();
  });
}

// -------------------------
// Expose globally
// -------------------------
window.editCountry = editCountry;
window.deleteCountry = deleteCountry;
