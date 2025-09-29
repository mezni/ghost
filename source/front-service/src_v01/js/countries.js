// Countries Management JavaScript
class CountriesManager {
    constructor() {
        // Use your local API endpoint
        this.apiBaseUrl = 'http://127.0.0.1:3000/api/v1';
        this.countriesTable = null;
        this.init();
    }

    init() {
        console.log('CountriesManager initialized');
        this.initializeDataTable();
        this.loadCountries();
        this.setupEventListeners();
    }

    initializeDataTable() {
        this.countriesTable = $('#countriesTable').DataTable({
            "paging": true,
            "lengthChange": true,
            "searching": true,
            "ordering": true,
            "info": true,
            "autoWidth": false,
            "responsive": true,
            "language": {
                "emptyTable": "No countries found",
                "loadingRecords": "Loading countries...",
                "processing": "Processing..."
            },
            "columns": [
                { "data": "country_id" },
                { "data": "iso_code" },
                { "data": "country_name" },
                { "data": "created_by" },
                { 
                    "data": "created_at",
                    "render": function(data) {
                        return data ? new Date(data).toLocaleDateString() : 'N/A';
                    }
                },
                {
                    "data": null,
                    "render": function(data, type, row) {
                        return `
                            <div class="btn-group">
                                <button class="btn btn-warning btn-sm edit-btn" data-id="${row.country_id}">
                                    <i class="fas fa-edit"></i>
                                </button>
                                <button class="btn btn-danger btn-sm delete-btn" data-id="${row.country_id}">
                                    <i class="fas fa-trash"></i>
                                </button>
                            </div>
                        `;
                    }
                }
            ]
        });
    }

    async loadCountries() {
        console.log('Loading countries from API...');
        
        try {
            // Show loading state
            this.showAlert('Loading countries...', 'info', 0); // 0 = no auto-close
            
            const response = await fetch(`${this.apiBaseUrl}/countries`, {
                method: 'GET',
                headers: {
                    'Content-Type': 'application/json',
                    'Accept': 'application/json'
                }
            });

            console.log('API Response status:', response.status);

            if (!response.ok) {
                throw new Error(`HTTP error! status: ${response.status}`);
            }
            
            const countries = await response.json();
            console.log('Countries loaded successfully:', countries);
            
            this.countriesTable.clear().rows.add(countries).draw();
            
            // Close loading alert and show success
            this.removeAlert();
            this.showAlert(`Successfully loaded ${countries.length} countries`, 'success', 3000);
            
        } catch (error) {
            console.error('Error loading countries:', error);
            
            // Remove loading alert and show error
            this.removeAlert();
            this.showAlert('Error loading countries: ' + error.message, 'danger', 5000);
            
            // Fallback to mock data for development
            const mockCountries = [
                {
                    "country_id": 1,
                    "iso_code": "US",
                    "country_name": "United States",
                    "created_at": "2025-09-29T01:59:27.583148",
                    "created_by": "admin@example.com",
                    "updated_at": null,
                    "updated_by": null
                },
                {
                    "country_id": 2, 
                    "iso_code": "TN",
                    "country_name": "Tunisia",
                    "created_at": "2025-09-29T02:02:09.144547",
                    "created_by": "system",
                    "updated_at": null,
                    "updated_by": null
                },
                {
                    "country_id": 3,
                    "iso_code": "FR", 
                    "country_name": "France",
                    "created_at": "2025-09-29T03:00:00.000000",
                    "created_by": "admin@example.com",
                    "updated_at": null,
                    "updated_by": null
                }
            ];
            
            this.countriesTable.clear().rows.add(mockCountries).draw();
            this.showAlert('Using demo data (API unavailable)', 'warning', 4000);
        }
    }

    setupEventListeners() {
        console.log('Setting up event listeners');
        
        // Add country form submission
        $('#addCountryForm').on('submit', (e) => {
            e.preventDefault();
            this.addCountry();
        });

        // Edit country form submission
        $('#editCountryForm').on('submit', (e) => {
            e.preventDefault();
            this.updateCountry();
        });

        // Handle edit and delete buttons
        $('#countriesTable').on('click', '.edit-btn', (e) => {
            const countryId = $(e.currentTarget).data('id');
            this.editCountry(countryId);
        });

        $('#countriesTable').on('click', '.delete-btn', (e) => {
            const countryId = $(e.currentTarget).data('id');
            this.deleteCountry(countryId);
        });

        // Reset form when modal is closed
        $('#addCountryModal').on('hidden.bs.modal', () => {
            $('#addCountryForm')[0].reset();
        });

        $('#editCountryModal').on('hidden.bs.modal', () => {
            $('#editCountryForm')[0].reset();
        });
    }

    async addCountry() {
        const formData = {
            iso_code: $('#isoCode').val().toUpperCase(),
            country_name: $('#countryName').val(),
            created_by: 'admin@example.com'
        };

        // Basic validation
        if (!formData.iso_code || !formData.country_name) {
            this.showAlert('Please fill in all required fields', 'danger', 3000);
            return;
        }

        if (formData.iso_code.length !== 2) {
            this.showAlert('ISO Code must be exactly 2 characters', 'danger', 3000);
            return;
        }

        try {
            this.showAlert('Adding country...', 'info', 0);
            
            const response = await fetch(`${this.apiBaseUrl}/countries`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                    'Accept': 'application/json'
                },
                body: JSON.stringify(formData)
            });

            if (!response.ok) {
                throw new Error(`HTTP error! status: ${response.status}`);
            }

            const newCountry = await response.json();
            
            // Add the new country to the table
            this.countriesTable.row.add(newCountry).draw();
            
            // Close modal and reset form
            $('#addCountryModal').modal('hide');
            $('#addCountryForm')[0].reset();
            
            this.removeAlert();
            this.showAlert('Country added successfully!', 'success', 3000);
            
        } catch (error) {
            console.error('Error adding country:', error);
            this.removeAlert();
            this.showAlert('Error adding country: ' + error.message, 'danger', 5000);
        }
    }

    async editCountry(countryId) {
        try {
            this.showAlert('Loading country data...', 'info', 0);
            
            const response = await fetch(`${this.apiBaseUrl}/countries/${countryId}`, {
                method: 'GET',
                headers: {
                    'Accept': 'application/json'
                }
            });

            if (!response.ok) {
                throw new Error(`HTTP error! status: ${response.status}`);
            }
            
            const country = await response.json();
            
            // Populate the edit form
            $('#editCountryId').val(country.country_id);
            $('#editIsoCode').val(country.iso_code);
            $('#editCountryName').val(country.country_name);
            
            // Show the edit modal
            $('#editCountryModal').modal('show');
            
            this.removeAlert();
            
        } catch (error) {
            console.error('Error fetching country:', error);
            this.removeAlert();
            this.showAlert('Error fetching country data: ' + error.message, 'danger', 5000);
        }
    }

    async updateCountry() {
        const countryId = $('#editCountryId').val();
        const formData = {
            iso_code: $('#editIsoCode').val().toUpperCase(),
            country_name: $('#editCountryName').val(),
            updated_by: 'admin@example.com'
        };

        try {
            this.showAlert('Updating country...', 'info', 0);
            
            const response = await fetch(`${this.apiBaseUrl}/countries/${countryId}`, {
                method: 'PUT',
                headers: {
                    'Content-Type': 'application/json',
                    'Accept': 'application/json'
                },
                body: JSON.stringify(formData)
            });

            if (!response.ok) {
                throw new Error(`HTTP error! status: ${response.status}`);
            }

            const updatedCountry = await response.json();
            
            // Update the row in the table
            const rowIndex = this.countriesTable.row((idx, data) => data.country_id === parseInt(countryId)).index();
            if (rowIndex !== undefined) {
                this.countriesTable.row(rowIndex).data(updatedCountry).draw();
            }
            
            // Close modal
            $('#editCountryModal').modal('hide');
            
            this.removeAlert();
            this.showAlert('Country updated successfully!', 'success', 3000);
            
        } catch (error) {
            console.error('Error updating country:', error);
            this.removeAlert();
            this.showAlert('Error updating country: ' + error.message, 'danger', 5000);
        }
    }

    async deleteCountry(countryId) {
        // Get country name for confirmation message
        const countryName = this.countriesTable.row((idx, data) => data.country_id === parseInt(countryId)).data().country_name;
        
        if (!confirm(`Are you sure you want to delete "${countryName}"?`)) {
            return;
        }

        try {
            this.showAlert('Deleting country...', 'info', 0);
            
            const response = await fetch(`${this.apiBaseUrl}/countries/${countryId}`, {
                method: 'DELETE',
                headers: {
                    'Accept': 'application/json'
                }
            });

            if (!response.ok) {
                throw new Error(`HTTP error! status: ${response.status}`);
            }

            // Remove the row from the table
            this.countriesTable.row((idx, data) => data.country_id === parseInt(countryId)).remove().draw();
            
            this.removeAlert();
            this.showAlert('Country deleted successfully!', 'success', 3000);
            
        } catch (error) {
            console.error('Error deleting country:', error);
            this.removeAlert();
            this.showAlert('Error deleting country: ' + error.message, 'danger', 5000);
        }
    }

    showAlert(message, type, autoClose = 3000) {
        // Remove any existing alerts first
        this.removeAlert();
        
        const alertClass = `alert-${type}`;
        const alertHtml = `
            <div class="alert ${alertClass} alert-dismissible fade show" role="alert" style="position: fixed; top: 80px; right: 20px; z-index: 9999; min-width: 300px;">
                <strong>${type.toUpperCase()}:</strong> ${message}
                <button type="button" class="close" data-dismiss="alert" aria-label="Close">
                    <span aria-hidden="true">&times;</span>
                </button>
            </div>
        `;
        
        // Add new alert to the body
        $('body').append(alertHtml);
        
        // Auto remove after specified time (0 = no auto-close)
        if (autoClose > 0) {
            setTimeout(() => {
                this.removeAlert();
            }, autoClose);
        }
    }

    removeAlert() {
        $('.alert').remove();
    }
}

// Initialize the countries manager when the document is ready
$(document).ready(function() {
    console.log('Document ready, initializing CountriesManager...');
    new CountriesManager();
});

// Global function for logout
function logout() {
    if (confirm('Are you sure you want to logout?')) {
        // Clear any stored authentication data
        localStorage.removeItem('authToken');
        sessionStorage.removeItem('authToken');
        
        // Redirect to home page
        window.location.href = '../index.html';
    }
}