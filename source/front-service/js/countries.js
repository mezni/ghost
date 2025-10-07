/**
 * Countries Management Script
 * Handles CRUD operations with backend API
 */

const CountriesApp = {
  apiBase: "http://localhost:3000/api/v1/countries",
  countries: [],
  filteredCountries: [],
  editId: null,
  currentPage: 1,
  itemsPerPage: 10, // Changed from 5 to 10
  filters: {
    isoCode: '',
    countryName: ''
  },

  init() {
    this.loadCountries();
    this.setupEventListeners();
  },

  async loadCountries() {
    try {
      const res = await fetch(this.apiBase);
      if (!res.ok) throw new Error("Failed to fetch countries");
      this.countries = await res.json();
      this.filteredCountries = [...this.countries];
      this.applyFilters(); // Apply any existing filters
    } catch (err) {
      console.error("Error loading countries:", err);
      alert("Failed to load countries");
    }
  },

  applyFilters() {
    // Get current filter values
    const isoFilter = document.getElementById('isoFilter');
    const countryFilter = document.getElementById('countryFilter');
    
    this.filters.isoCode = isoFilter ? isoFilter.value.toLowerCase().trim() : '';
    this.filters.countryName = countryFilter ? countryFilter.value.toLowerCase().trim() : '';

    // Filter countries
    this.filteredCountries = this.countries.filter(country => {
      const matchesIso = !this.filters.isoCode || 
        country.iso_code.toLowerCase().includes(this.filters.isoCode);
      
      const matchesCountry = !this.filters.countryName || 
        country.country_name.toLowerCase().includes(this.filters.countryName);
      
      return matchesIso && matchesCountry;
    });

    // Reset to first page when filtering
    this.currentPage = 1;
    
    this.renderTable();
    this.renderPagination();
    this.updateFilterInfo();
  },

  clearFilters() {
    const isoFilter = document.getElementById('isoFilter');
    const countryFilter = document.getElementById('countryFilter');
    
    if (isoFilter) isoFilter.value = '';
    if (countryFilter) countryFilter.value = '';
    
    this.filters.isoCode = '';
    this.filters.countryName = '';
    this.filteredCountries = [...this.countries];
    this.currentPage = 1;
    
    this.renderTable();
    this.renderPagination();
    this.updateFilterInfo();
  },

  updateFilterInfo() {
    const filterInfo = document.getElementById('filter-info');
    if (!filterInfo) return;

    const activeFilters = [];
    if (this.filters.isoCode) activeFilters.push(`ISO: "${this.filters.isoCode.toUpperCase()}"`);
    if (this.filters.countryName) activeFilters.push(`Country: "${this.filters.countryName}"`);

    if (activeFilters.length > 0) {
      filterInfo.textContent = `Filtered by ${activeFilters.join(' and ')} • ${this.filteredCountries.length} countries found`;
      filterInfo.className = 'text-primary small';
    } else {
      filterInfo.textContent = 'Type to filter countries';
      filterInfo.className = 'text-muted small';
    }
  },

  renderTable() {
    const tbody = document.getElementById("country-table-body");
    if (!tbody) {
      console.error("Table body element not found");
      return;
    }
    
    tbody.innerHTML = "";

    // Calculate pagination
    const startIndex = (this.currentPage - 1) * this.itemsPerPage;
    const endIndex = startIndex + this.itemsPerPage;
    const paginatedCountries = this.filteredCountries.slice(startIndex, endIndex);

    if (paginatedCountries.length === 0) {
      const tr = document.createElement("tr");
      tr.innerHTML = `
        <td colspan="4" class="text-center text-muted py-4">
          <i class="bi bi-search display-4 d-block mb-2"></i>
          ${this.filteredCountries.length === 0 ? 'No countries found' : 'No countries match your filters'}
        </td>
      `;
      tbody.appendChild(tr);
      return;
    }

    paginatedCountries.forEach(c => {
      const tr = document.createElement("tr");
      
      // Highlight matching text in filters
      const isoCode = this.highlightText(c.iso_code, this.filters.isoCode);
      const countryName = this.highlightText(c.country_name, this.filters.countryName);
      
      tr.innerHTML = `
        <td>${c.country_id}</td>
        <td>
          <span class="badge bg-primary">${isoCode}</span>
        </td>
        <td>
          <i class="bi bi-flag-fill text-primary me-2"></i>
          ${countryName}
        </td>
        <td class="text-center">
          <button class="btn btn-sm btn-action btn-edit me-1" onclick="CountriesApp.showEditModal(${c.country_id})">
            <i class="bi bi-pencil"></i>
          </button>
          <button class="btn btn-sm btn-action btn-delete" onclick="CountriesApp.showDeleteModal(${c.country_id})">
            <i class="bi bi-trash"></i>
          </button>
        </td>
      `;
      tbody.appendChild(tr);
    });

    // Update showing information
    this.updateShowingInfo();
  },

  highlightText(text, filter) {
    if (!filter) return text;
    
    const regex = new RegExp(`(${this.escapeRegex(filter)})`, 'gi');
    return text.replace(regex, '<mark class="bg-warning">$1</mark>');
  },

  escapeRegex(string) {
    return string.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
  },

  renderPagination() {
    const paginationEl = document.getElementById("pagination");
    if (!paginationEl) return;

    const totalPages = Math.ceil(this.filteredCountries.length / this.itemsPerPage);
    
    if (totalPages <= 1) {
      paginationEl.innerHTML = '';
      return;
    }

    let paginationHTML = '';

    // Previous button
    paginationHTML += `
      <li class="page-item ${this.currentPage === 1 ? 'disabled' : ''}">
        <a class="page-link" href="javascript:void(0);" onclick="CountriesApp.changePage(${this.currentPage - 1})">
          <i class="bi bi-chevron-left"></i>
        </a>
      </li>
    `;

    // Page numbers
    for (let i = 1; i <= totalPages; i++) {
      if (i === 1 || i === totalPages || (i >= this.currentPage - 1 && i <= this.currentPage + 1)) {
        paginationHTML += `
          <li class="page-item ${i === this.currentPage ? 'active' : ''}">
            <a class="page-link" href="javascript:void(0);" onclick="CountriesApp.changePage(${i})">${i}</a>
          </li>
        `;
      } else if (i === this.currentPage - 2 || i === this.currentPage + 2) {
        paginationHTML += `<li class="page-item disabled"><span class="page-link">...</span></li>`;
      }
    }

    // Next button
    paginationHTML += `
      <li class="page-item ${this.currentPage === totalPages ? 'disabled' : ''}">
        <a class="page-link" href="javascript:void(0);" onclick="CountriesApp.changePage(${this.currentPage + 1})">
          <i class="bi bi-chevron-right"></i>
        </a>
      </li>
    `;

    paginationEl.innerHTML = paginationHTML;
  },

  changePage(page) {
    const totalPages = Math.ceil(this.filteredCountries.length / this.itemsPerPage);
    
    if (page < 1 || page > totalPages) return;
    
    this.currentPage = page;
    this.renderTable();
    this.renderPagination();
  },

  updateShowingInfo() {
    const totalItems = this.filteredCountries.length;
    const startIndex = (this.currentPage - 1) * this.itemsPerPage + 1;
    const endIndex = Math.min(this.currentPage * this.itemsPerPage, totalItems);

    document.getElementById('showing-from').textContent = totalItems === 0 ? 0 : startIndex;
    document.getElementById('showing-to').textContent = endIndex;
    document.getElementById('total-items').textContent = totalItems;
  },

  setupEventListeners() {
    const deleteBtn = document.getElementById("confirmDelete");
    if (deleteBtn) {
      deleteBtn.addEventListener("click", () => {
        if (this.editId !== null) this.deleteCountry(this.editId);
      });
    }

    // Add keyboard shortcut for clearing filters (Ctrl + Shift + F)
    document.addEventListener('keydown', (e) => {
      if (e.ctrlKey && e.shiftKey && e.key === 'F') {
        e.preventDefault();
        this.clearFilters();
      }
    });
  },

  showDeleteModal(id) {
    this.editId = id;
    const modalEl = document.getElementById("deleteModal");
    if (modalEl) {
      const modal = new bootstrap.Modal(modalEl);
      modal.show();
    } else {
      console.error("Delete modal element not found");
    }
  },

  async deleteCountry(id) {
    try {
      const res = await fetch(`${this.apiBase}/${id}`, { method: "DELETE" });
      if (!res.ok) throw new Error("Failed to delete country");
      
      // Remove from both arrays
      this.countries = this.countries.filter(c => c.country_id !== id);
      this.filteredCountries = this.filteredCountries.filter(c => c.country_id !== id);
      
      // Adjust current page if needed
      const totalPages = Math.ceil(this.filteredCountries.length / this.itemsPerPage);
      if (this.currentPage > totalPages && totalPages > 0) {
        this.currentPage = totalPages;
      }
      
      this.renderTable();
      this.renderPagination();
      this.updateFilterInfo();
      
      const modalEl = document.getElementById("deleteModal");
      if (modalEl) {
        const modal = bootstrap.Modal.getInstance(modalEl);
        if (modal) modal.hide();
      }
      
      this.showAlert('Country deleted successfully!', 'success');
    } catch (err) {
      console.error(err);
      this.showAlert('Failed to delete country', 'danger');
    }
  },

  showEditModal(id) {
    const country = this.countries.find(c => c.country_id === id);
    if (!country) return;
    
    const newName = prompt("Update country name:", country.country_name);
    if (newName && newName !== country.country_name) {
      this.updateCountry({ 
        country_id: id, 
        country_name: newName, 
        updated_by: "admin" 
      });
    }
  },

  async updateCountry(data) {
    try {
      const res = await fetch(this.apiBase, {
        method: "PUT",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(data),
      });
      
      if (!res.ok) throw new Error("Failed to update country");
      await this.loadCountries(); // Reload to refresh filters
      this.showAlert('Country updated successfully!', 'success');
    } catch (err) {
      console.error(err);
      this.showAlert('Failed to update country', 'danger');
    }
  },

  async addCountry() {
    const iso = prompt("Enter ISO code (2-3 characters):");
    const name = prompt("Enter country name:");
    
    if (!iso || !name) return;
    if (iso.length < 2 || iso.length > 3) {
      this.showAlert('ISO code must be 2-3 characters', 'warning');
      return;
    }

    const data = { 
      iso_code: iso.toUpperCase(), 
      country_name: name, 
      created_by: "admin" 
    };
    
    try {
      const res = await fetch(this.apiBase, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(data),
      });
      
      if (!res.ok) throw new Error("Failed to add country");
      await this.loadCountries(); // Reload to refresh filters
      
      // Go to last page to see the new item
      const totalPages = Math.ceil(this.filteredCountries.length / this.itemsPerPage);
      this.currentPage = totalPages;
      this.renderTable();
      this.renderPagination();
      
      this.showAlert('Country added successfully!', 'success');
    } catch (err) {
      console.error(err);
      this.showAlert('Failed to add country', 'danger');
    }
  },

  showAlert(message, type = 'info') {
    const existingAlert = document.querySelector('.alert');
    if (existingAlert) {
      existingAlert.remove();
    }
    
    const alertDiv = document.createElement('div');
    alertDiv.className = `alert alert-${type} alert-dismissible fade show`;
    alertDiv.innerHTML = `
      ${message}
      <button type="button" class="btn-close" data-bs-dismiss="alert" aria-label="Close"></button>
    `;
    
    const container = document.querySelector('.container');
    if (container) {
      container.insertBefore(alertDiv, container.firstChild);
      
      setTimeout(() => {
        if (alertDiv.parentNode) {
          alertDiv.remove();
        }
      }, 5000);
    }
  }
};

// Initialize when DOM is loaded
document.addEventListener('DOMContentLoaded', function() {
  CountriesApp.init();
});

// Add country function for button
function addNewCountry() {
  CountriesApp.addCountry();
}