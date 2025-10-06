/**
 * Countries Management Controller
 * Handles CRUD operations for countries management
 */

class CountriesManager {
    static app = null;
    static countries = [];
    static currentPage = 1;
    static itemsPerPage = 10;
    static totalItems = 0;
    static searchTerm = '';
    static statusFilter = '';
    static regionFilter = '';

    static async init(app) {
        this.app = app;
        await this.loadCountries();
        this.setupEventListeners();
        this.initializeDataTable();
    }

    /**
     * Load countries data from API
     */
    static async loadCountries() {
        try {
            this.showLoading();
            
            // Simulate API call with mock data
            const response = await this.getMockCountriesData();
            this.countries = response.data;
            this.totalItems = response.total;
            
            this.renderCountriesTable();
            this.updatePagination();
            this.updateTableInfo();
            
        } catch (error) {
            console.error('Error loading countries:', error);
            this.app.showError('Failed to load countries data');
            this.showErrorState();
        } finally {
            this.hideLoading();
        }
    }

    /**
     * Render countries table
     */
    static renderCountriesTable() {
        const tbody = document.getElementById('countries-tbody');
        if (!tbody) return;

        // Filter countries based on search and filters
        const filteredCountries = this.getFilteredCountries();
        const paginatedCountries = this.getPaginatedData(filteredCountries);

        if (paginatedCountries.length === 0) {
            tbody.innerHTML = `
                <tr>
                    <td colspan="8" class="text-center text-muted">
                        <i class="fas fa-inbox"></i> No countries found
                    </td>
                </tr>
            `;
            return;
        }

        tbody.innerHTML = paginatedCountries.map((country, index) => `
            <tr>
                <td>
                    <input type="checkbox" class="country-checkbox" value="${country.id}">
                    ${(this.currentPage - 1) * this.itemsPerPage + index + 1}
                </td>
                <td>
                    <strong>${country.code}</strong>
                </td>
                <td>
                    <div class="d-flex align-items-center">
                        <span class="flag-icon flag-icon-${country.code.toLowerCase()} mr-2" 
                              style="font-size: 1.2em;" title="${country.name}"></span>
                        ${country.name}
                    </div>
                </td>
                <td>
                    <span class="badge badge-light">${this.capitalizeFirstLetter(country.region)}</span>
                </td>
                <td>
                    <span class="badge badge-info">${country.operatorCount}</span>
                </td>
                <td>
                    <span class="badge badge-${country.status === 'active' ? 'success' : 'danger'}">
                        ${country.status.toUpperCase()}
                    </span>
                </td>
                <td>
                    <small class="text-muted">${this.formatDate(country.updatedAt)}</small>
                </td>
                <td>
                    <div class="btn-group btn-group-sm">
                        <button class="btn btn-outline-primary edit-country" 
                                data-id="${country.id}" title="Edit">
                            <i class="fas fa-edit"></i>
                        </button>
                        <button class="btn btn-outline-${country.status === 'active' ? 'warning' : 'success'} toggle-status" 
                                data-id="${country.id}" data-status="${country.status}"
                                title="${country.status === 'active' ? 'Deactivate' : 'Activate'}">
                            <i class="fas fa-${country.status === 'active' ? 'pause' : 'play'}"></i>
                        </button>
                        <button class="btn btn-outline-danger delete-country" 
                                data-id="${country.id}" data-name="${country.name}" title="Delete">
                            <i class="fas fa-trash"></i>
                        </button>
                    </div>
                </td>
            </tr>
        `).join('');
    }

    /**
     * Get filtered countries based on search and filters
     */
    static getFilteredCountries() {
        return this.countries.filter(country => {
            const matchesSearch = !this.searchTerm || 
                country.name.toLowerCase().includes(this.searchTerm.toLowerCase()) ||
                country.code.toLowerCase().includes(this.searchTerm.toLowerCase());
            
            const matchesStatus = !this.statusFilter || country.status === this.statusFilter;
            const matchesRegion = !this.regionFilter || country.region === this.regionFilter;
            
            return matchesSearch && matchesStatus && matchesRegion;
        });
    }

    /**
     * Get paginated data
     */
    static getPaginatedData(data) {
        const startIndex = (this.currentPage - 1) * this.itemsPerPage;
        return data.slice(startIndex, startIndex + this.itemsPerPage);
    }

    /**
     * Update pagination controls
     */
    static updatePagination() {
        const pagination = document.getElementById('pagination');
        if (!pagination) return;

        const filteredCountries = this.getFilteredCountries();
        const totalPages = Math.ceil(filteredCountries.length / this.itemsPerPage);
        
        // Ensure current page is valid
        if (this.currentPage > totalPages) {
            this.currentPage = Math.max(1, totalPages);
        }

        let paginationHTML = '';
        
        // Previous button
        paginationHTML += `
            <li class="page-item ${this.currentPage === 1 ? 'disabled' : ''}">
                <a class="page-link" href="#" data-page="${this.currentPage - 1}">Previous</a>
            </li>
        `;

        // Page numbers
        for (let i = 1; i <= totalPages; i++) {
            if (i === 1 || i === totalPages || (i >= this.currentPage - 2 && i <= this.currentPage + 2)) {
                paginationHTML += `
                    <li class="page-item ${i === this.currentPage ? 'active' : ''}">
                        <a class="page-link" href="#" data-page="${i}">${i}</a>
                    </li>
                `;
            } else if (i === this.currentPage - 3 || i === this.currentPage + 3) {
                paginationHTML += `<li class="page-item disabled"><span class="page-link">...</span></li>`;
            }
        }

        // Next button
        paginationHTML += `
            <li class="page-item ${this.currentPage === totalPages ? 'disabled' : ''}">
                <a class="page-link" href="#" data-page="${this.currentPage + 1}">Next</a>
            </li>
        `;

        pagination.innerHTML = paginationHTML;
    }

    /**
     * Update table information
     */
    static updateTableInfo() {
        const infoElement = document.getElementById('table-info');
        if (!infoElement) return;

        const filteredCountries = this.getFilteredCountries();
        const startIndex = (this.currentPage - 1) * this.itemsPerPage + 1;
        const endIndex = Math.min(startIndex + this.itemsPerPage - 1, filteredCountries.length);

        infoElement.textContent = `Showing ${startIndex} to ${endIndex} of ${filteredCountries.length} entries`;
    }

    /**
     * Setup event listeners
     */
    static setupEventListeners() {
        // Add country button
        document.getElementById('add-country-btn')?.addEventListener('click', () => {
            this.openCountryModal();
        });

        // Save country button
        document.getElementById('save-country-btn')?.addEventListener('click', () => {
            this.saveCountry();
        });

        // Search functionality
        const searchInput = document.getElementById('search-countries');
        if (searchInput) {
            searchInput.addEventListener('input', this.app.debounce(() => {
                this.searchTerm = searchInput.value;
                this.currentPage = 1;
                this.renderCountriesTable();
                this.updatePagination();
                this.updateTableInfo();
            }, 300));
        }

        // Filter functionality
        document.getElementById('status-filter')?.addEventListener('change', (e) => {
            this.statusFilter = e.target.value;
            this.currentPage = 1;
            this.renderCountriesTable();
            this.updatePagination();
            this.updateTableInfo();
        });

        document.getElementById('region-filter')?.addEventListener('change', (e) => {
            this.regionFilter = e.target.value;
            this.currentPage = 1;
            this.renderCountriesTable();
            this.updatePagination();
            this.updateTableInfo();
        });

        // Table actions (using event delegation)
        document.addEventListener('click', (e) => {
            // Edit country
            if (e.target.closest('.edit-country')) {
                const button = e.target.closest('.edit-country');
                const countryId = button.getAttribute('data-id');
                this.editCountry(parseInt(countryId));
            }

            // Toggle status
            if (e.target.closest('.toggle-status')) {
                const button = e.target.closest('.toggle-status');
                const countryId = button.getAttribute('data-id');
                const currentStatus = button.getAttribute('data-status');
                this.toggleCountryStatus(parseInt(countryId), currentStatus);
            }

            // Delete country
            if (e.target.closest('.delete-country')) {
                const button = e.target.closest('.delete-country');
                const countryId = button.getAttribute('data-id');
                const countryName = button.getAttribute('data-name');
                this.confirmDeleteCountry(parseInt(countryId), countryName);
            }

            // Pagination
            if (e.target.closest('.page-link')) {
                e.preventDefault();
                const page = parseInt(e.target.getAttribute('data-page'));
                if (page && page !== this.currentPage) {
                    this.currentPage = page;
                    this.renderCountriesTable();
                    this.updatePagination();
                    this.updateTableInfo();
                }
            }
        });

        // Delete confirmation
        document.getElementById('confirm-delete-btn')?.addEventListener('click', () => {
            this.deleteCountry();
        });

        // Refresh button
        document.getElementById('refresh-countries')?.addEventListener('click', () => {
            this.loadCountries();
        });

        // Export CSV
        document.getElementById('export-csv')?.addEventListener('click', () => {
            this.exportToCSV();
        });

        // Bulk actions
        document.getElementById('apply-bulk-action')?.addEventListener('click', () => {
            this.applyBulkAction();
        });
    }

    /**
     * Open country modal for adding new country
     */
    static openCountryModal(country = null) {
        const modal = document.getElementById('countryModal');
        const title = document.getElementById('modal-title');
        const form = document.getElementById('country-form');
        
        if (country) {
            title.textContent = 'Edit Country';
            this.populateForm(country);
        } else {
            title.textContent = 'Add New Country';
            form.reset();
            document.getElementById('country-id').value = '';
        }
        
        $(modal).modal('show');
    }

    /**
     * Populate form with country data
     */
    static populateForm(country) {
        document.getElementById('country-id').value = country.id;
        document.getElementById('country-code').value = country.code;
        document.getElementById('country-name').value = country.name;
        document.getElementById('country-region').value = country.region;
        document.getElementById('country-timezone').value = country.timezone || '';
        document.getElementById('country-currency').value = country.currency || '';
        document.getElementById('country-status').value = country.status;
        document.getElementById('country-notes').value = country.notes || '';
        document.getElementById('country-roaming-enabled').checked = country.roamingEnabled || false;
    }

    /**
     * Edit country
     */
    static editCountry(countryId) {
        const country = this.countries.find(c => c.id === countryId);
        if (country) {
            this.openCountryModal(country);
        }
    }

    /**
     * Save country (create or update)
     */
    static async saveCountry() {
        const form = document.getElementById('country-form');
        
        if (!form.checkValidity()) {
            form.reportValidity();
            return;
        }

        try {
            const formData = this.getFormData();
            let result;

            if (formData.id) {
                // Update existing country
                result = await this.updateCountry(formData);
            } else {
                // Create new country
                result = await this.createCountry(formData);
            }

            $('#countryModal').modal('hide');
            await this.loadCountries();
            this.app.showSuccess(`Country ${formData.id ? 'updated' : 'created'} successfully`);

        } catch (error) {
            console.error('Error saving country:', error);
            this.app.showError('Failed to save country');
        }
    }

    /**
     * Get form data
     */
    static getFormData() {
        return {
            id: document.getElementById('country-id').value || null,
            code: document.getElementById('country-code').value.toUpperCase(),
            name: document.getElementById('country-name').value,
            region: document.getElementById('country-region').value,
            timezone: document.getElementById('country-timezone').value,
            currency: document.getElementById('country-currency').value.toUpperCase(),
            status: document.getElementById('country-status').value,
            notes: document.getElementById('country-notes').value,
            roamingEnabled: document.getElementById('country-roaming-enabled').checked
        };
    }

    /**
     * Toggle country status
     */
    static async toggleCountryStatus(countryId, currentStatus) {
        try {
            const newStatus = currentStatus === 'active' ? 'inactive' : 'active';
            await this.updateCountryStatus(countryId, newStatus);
            await this.loadCountries();
            this.app.showSuccess(`Country status updated to ${newStatus}`);
        } catch (error) {
            console.error('Error updating country status:', error);
            this.app.showError('Failed to update country status');
        }
    }

    /**
     * Confirm country deletion
     */
    static confirmDeleteCountry(countryId, countryName) {
        document.getElementById('delete-country-name').textContent = countryName;
        document.getElementById('confirm-delete-btn').setAttribute('data-country-id', countryId);
        $('#deleteModal').modal('show');
    }

    /**
     * Delete country
     */
    static async deleteCountry() {
        const countryId = document.getElementById('confirm-delete-btn').getAttribute('data-country-id');
        
        try {
            await this.deleteCountryRequest(parseInt(countryId));
            $('#deleteModal').modal('hide');
            await this.loadCountries();
            this.app.showSuccess('Country deleted successfully');
        } catch (error) {
            console.error('Error deleting country:', error);
            this.app.showError('Failed to delete country');
        }
    }

    /**
     * Apply bulk actions
     */
    static async applyBulkAction() {
        const action = document.getElementById('bulk-action').value;
        const selectedCountries = this.getSelectedCountries();

        if (selectedCountries.length === 0) {
            this.app.showWarning('Please select at least one country');
            return;
        }

        if (!action) {
            this.app.showWarning('Please select a bulk action');
            return;
        }

        try {
            switch (action) {
                case 'activate':
                    await this.bulkUpdateStatus(selectedCountries, 'active');
                    break;
                case 'deactivate':
                    await this.bulkUpdateStatus(selectedCountries, 'inactive');
                    break;
                case 'delete':
                    if (confirm(`Are you sure you want to delete ${selectedCountries.length} countries?`)) {
                        await this.bulkDelete(selectedCountries);
                    }
                    break;
            }

            await this.loadCountries();
            this.app.showSuccess(`Bulk action completed successfully`);
            
        } catch (error) {
            console.error('Error applying bulk action:', error);
            this.app.showError('Failed to apply bulk action');
        }
    }

    /**
     * Get selected countries
     */
    static getSelectedCountries() {
        const checkboxes = document.querySelectorAll('.country-checkbox:checked');
        return Array.from(checkboxes).map(cb => parseInt(cb.value));
    }

    /**
     * Export to CSV
     */
    static exportToCSV() {
        const filteredCountries = this.getFilteredCountries();
        const headers = ['Code', 'Name', 'Region', 'Timezone', 'Currency', 'Status', 'Operators', 'Last Updated'];
        
        let csvContent = headers.join(',') + '\n';
        
        filteredCountries.forEach(country => {
            const row = [
                country.code,
                `"${country.name}"`,
                country.region,
                country.timezone || '',
                country.currency || '',
                country.status,
                country.operatorCount,
                this.formatDate(country.updatedAt)
            ];
            csvContent += row.join(',') + '\n';
        });

        const blob = new Blob([csvContent], { type: 'text/csv' });
        const url = window.URL.createObjectURL(blob);
        const a = document.createElement('a');
        a.href = url;
        a.download = `countries_export_${new Date().toISOString().split('T')[0]}.csv`;
        a.click();
        window.URL.revokeObjectURL(url);
        
        this.app.showSuccess('Countries data exported successfully');
    }

    /**
     * Initialize data table (basic functionality)
     */
    static initializeDataTable() {
        // Basic sorting could be implemented here
        // For advanced features, consider integrating DataTables plugin
    }

    /**
     * Show loading state
     */
    static showLoading() {
        const tbody = document.getElementById('countries-tbody');
        if (tbody) {
            tbody.innerHTML = `
                <tr>
                    <td colspan="8" class="text-center text-muted">
                        <i class="fas fa-spinner fa-spin"></i> Loading countries...
                    </td>
                </tr>
            `;
        }
    }

    /**
     * Hide loading state
     */
    static hideLoading() {
        // Loading state is handled in renderCountriesTable
    }

    /**
     * Show error state
     */
    static showErrorState() {
        const tbody = document.getElementById('countries-tbody');
        if (tbody) {
            tbody.innerHTML = `
                <tr>
                    <td colspan="8" class="text-center text-danger">
                        <i class="fas fa-exclamation-triangle"></i> Failed to load countries
                        <br>
                        <button class="btn btn-sm btn-outline-primary mt-2" onclick="CountriesManager.loadCountries()">
                            <i class="fas fa-redo"></i> Try Again
                        </button>
                    </td>
                </tr>
            `;
        }
    }

    /**
     * Utility: Capitalize first letter
     */
    static capitalizeFirstLetter(string) {
        return string.charAt(0).toUpperCase() + string.slice(1);
    }

    /**
     * Utility: Format date
     */
    static formatDate(dateString) {
        return new Date(dateString).toLocaleDateString('en-US', {
            year: 'numeric',
            month: 'short',
            day: 'numeric'
        });
    }

    // Mock API Methods (Replace with real API calls)
    static async getMockCountriesData() {
        await new Promise(resolve => setTimeout(resolve, 1000));
        
        const mockCountries = [
            { id: 1, code: 'USA', name: 'United States', region: 'north america', timezone: 'UTC-5 to UTC-10', currency: 'USD', status: 'active', operatorCount: 15, roamingEnabled: true, notes: 'Major roaming partner', updatedAt: '2024-01-15T10:30:00Z' },
            { id: 2, code: 'GBR', name: 'United Kingdom', region: 'europe', timezone: 'UTC+0', currency: 'GBP', status: 'active', operatorCount: 8, roamingEnabled: true, notes: '', updatedAt: '2024-01-14T14:20:00Z' },
            { id: 3, code: 'DEU', name: 'Germany', region: 'europe', timezone: 'UTC+1', currency: 'EUR', status: 'active', operatorCount: 12, roamingEnabled: true, notes: 'Strong roaming network', updatedAt: '2024-01-13T09:15:00Z' },
            { id: 4, code: 'FRA', name: 'France', region: 'europe', timezone: 'UTC+1', currency: 'EUR', status: 'active', operatorCount: 6, roamingEnabled: true, notes: '', updatedAt: '2024-01-12T16:45:00Z' },
            { id: 5, code: 'JPN', name: 'Japan', region: 'asia', timezone: 'UTC+9', currency: 'JPY', status: 'active', operatorCount: 10, roamingEnabled: true, notes: 'Advanced roaming services', updatedAt: '2024-01-11T11:20:00Z' },
            { id: 6, code: 'AUS', name: 'Australia', region: 'oceania', timezone: 'UTC+8 to UTC+10', currency: 'AUD', status: 'active', operatorCount: 5, roamingEnabled: true, notes: '', updatedAt: '2024-01-10T13:10:00Z' },
            { id: 7, code: 'BRA', name: 'Brazil', region: 'south america', timezone: 'UTC-3 to UTC-5', currency: 'BRL', status: 'inactive', operatorCount: 3, roamingEnabled: false, notes: 'Under maintenance', updatedAt: '2024-01-09T08:30:00Z' },
            { id: 8, code: 'ZAF', name: 'South Africa', region: 'africa', timezone: 'UTC+2', currency: 'ZAR', status: 'active', operatorCount: 4, roamingEnabled: true, notes: '', updatedAt: '2024-01-08T15:40:00Z' },
            { id: 9, code: 'ARE', name: 'United Arab Emirates', region: 'middle east', timezone: 'UTC+4', currency: 'AED', status: 'active', operatorCount: 2, roamingEnabled: true, notes: 'Premium roaming services', updatedAt: '2024-01-07T12:25:00Z' },
            { id: 10, code: 'CAN', name: 'Canada', region: 'north america', timezone: 'UTC-3.5 to UTC-8', currency: 'CAD', status: 'active', operatorCount: 7, roamingEnabled: true, notes: '', updatedAt: '2024-01-06T10:15:00Z' }
        ];

        return {
            data: mockCountries,
            total: mockCountries.length,
            page: this.currentPage,
            totalPages: Math.ceil(mockCountries.length / this.itemsPerPage)
        };
    }

    static async createCountry(countryData) {
        await new Promise(resolve => setTimeout(resolve, 500));
        const newCountry = {
            ...countryData,
            id: Math.max(...this.countries.map(c => c.id)) + 1,
            operatorCount: 0,
            updatedAt: new Date().toISOString()
        };
        return newCountry;
    }

    static async updateCountry(countryData) {
        await new Promise(resolve => setTimeout(resolve, 500));
        return { ...countryData, updatedAt: new Date().toISOString() };
    }

    static async updateCountryStatus(countryId, status) {
        await new Promise(resolve => setTimeout(resolve, 300));
        return { success: true };
    }

    static async deleteCountryRequest(countryId) {
        await new Promise(resolve => setTimeout(resolve, 500));
        return { success: true };
    }

    static async bulkUpdateStatus(countryIds, status) {
        await new Promise(resolve => setTimeout(resolve, 800));
        return { success: true };
    }

    static async bulkDelete(countryIds) {
        await new Promise(resolve => setTimeout(resolve, 1000));
        return { success: true };
    }
}

export default CountriesManager;