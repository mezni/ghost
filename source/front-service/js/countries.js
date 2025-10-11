/**
 * Countries Page Script
 * Handles loading, displaying, and actions for countries
 */

console.log("🎯 countries.js loaded");

// Use the centralized API_URL from app.js instead of redeclaring
window.countriesInit = async function() {
  console.log("💠 Countries page initialized");
  await loadCountriesTable();
};

async function loadCountriesTable() {
  console.log("📊 Loading countries table...");
  
  // Create the table structure if it doesn't exist
  const contentArea = document.getElementById('content-area');
  if (!contentArea) return;

  // Ensure the table structure exists
  if (!contentArea.querySelector('#countriesTable')) {
    contentArea.innerHTML = `
      <div class="card">
        <div class="card-header">
          <h3 class="card-title">Countries Management</h3>
          <button class="btn btn-primary btn-sm float-right" onclick="showAddCountryModal()">
            <i class="fas fa-plus"></i> Add Country
          </button>
        </div>
        <div class="card-body">
          <table id="countriesTable" class="table table-bordered table-striped">
            <thead>
              <tr>
                <th>#</th>
                <th>ISO Code</th>
                <th>Country Name</th>
                <th>Actions</th>
              </tr>
            </thead>
            <tbody>
              <tr>
                <td colspan="4" class="text-center">Loading countries...</td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
    `;
  }

  const tableBody = document.querySelector("#countriesTable tbody");
  if (!tableBody) return;

  try {
    // Use window.API_URL from app.js or fallback
    const apiUrl = window.API_URL || "http://localhost:3000/api/v1";
    console.log(`🌐 Fetching countries from: ${apiUrl}/countries`);
    
    const res = await fetch(`${apiUrl}/countries`);
    if (!res.ok) throw new Error(`HTTP ${res.status}`);
    
    const countries = await res.json();
    console.log(`✅ Loaded ${countries.length} countries`);

    tableBody.innerHTML = "";

    if (countries.length === 0) {
      tableBody.innerHTML = `
        <tr>
          <td colspan="4" class="text-center">No countries found</td>
        </tr>
      `;
      return;
    }

    countries.forEach((c, i) => {
      const row = document.createElement("tr");
      row.innerHTML = `
        <td>${i + 1}</td>
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
      tableBody.appendChild(row);
    });

  } catch (err) {
    console.error("❌ Failed to load countries:", err);
    tableBody.innerHTML = `
      <tr>
        <td colspan="4" class="text-center text-danger">
          Error loading countries: ${err.message}
        </td>
      </tr>
    `;
    AppUtils.notify("Error", `Failed to load countries: ${err.message}`, "error");
  }
}

function editCountry(id) {
  console.log(`✏️ Editing country ID: ${id}`);
  AppUtils.notify("Edit Country", `Edit country with ID ${id}`, "info");
  // Add your edit modal logic here
}

function deleteCountry(id) {
  console.log(`🗑️ Deleting country ID: ${id}`);
  if (confirm(`Are you sure you want to delete country ID ${id}?`)) {
    AppUtils.notify("Delete Country", `Deleted country ID ${id}`, "warning");
    // Add your delete API call here
  }
}

function showAddCountryModal() {
  console.log("➕ Showing add country modal");
  AppUtils.notify("Add Country", "Add new country functionality", "info");
  // Add your add modal logic here
}

// Expose functions globally
window.editCountry = editCountry;
window.deleteCountry = deleteCountry;
window.showAddCountryModal = showAddCountryModal;

console.log("✅ countries.js initialization complete");