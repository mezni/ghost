// Countries Management JavaScript
class CountriesManager {
    constructor() {
        this.apiBaseUrl = 'http://127.0.0.1:3000/api/v1';
        this.countriesTable = null;
        console.log('CountriesManager initialized');
        this.init();
    }

    init() {
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
                "emptyTable": "No countries available",
                "info": "Showing _START_ to _END_ of _TOTAL_ countries",
                "infoEmpty": "Showing 0 to 0 of 0 countries",
                "infoFiltered": "(filtered from _MAX_ total countries)",
                "lengthMenu": "Show _MENU_ countries",
                "loadingRecords": "Loading...",
                "processing": "Processing...",
                "search": "Search:",
                "zeroRecords": "No matching countries found",
                "paginate": {
                    "first": "First",
                    "last": "Last",
                    "next": "Next",
                    "previous": "Previous"
                }
            },
            "columns": [
                { 
                    "data": "country_id",
                    "className": "text-center",
                    "width": "5%"
                },
                { 
                    "data": "iso_code",
                    "className": "text-center",
                    "width": "10%"
                },
                { 
                    "data": "country_name",
                    "width": "25%"
                },
                { 
                    "data": "created_by",
                    "width": "15%"
                },
                { 
                    "data": "created_at",
                    "width": "15%",
                    "render": function(data) {
                        return data ? new Date(data).toLocaleDateString('en-US', {
                            year: 'numeric',
                            month: 'short',
                            day: 'numeric'
                        }) : 'N/A';
                    }
                },
                {
                    "data": null,
                    "className": "text-center",
                    "width": "20%",
                    "render": (data, type, row) => {
                        return `
                            <div class="btn-group action-buttons">
                                <button type="button" class="btn btn-warning btn-sm edit-btn" data-id="${row.country_id}" title="Edit">
                                    <i class="fas fa-edit"></i>
                                    <span class="btn-text">Edit</span>
                                </button>
                                <button type="button" class="btn btn-danger btn-sm delete-btn" data-id="${row.country_id}" title="Delete">
                                    <i class="fas fa-trash"></i>
                                    <span class="btn-text">Delete</span>
                                </button>
                            </div>
                        `;
                    }
                }
            ]
        });
    }

    async loadCountries() {
        try {
            console.log('Loading countries from API...');
            
            const response = await fetch(`${this.apiBaseUrl}/countries`);
            
            if (!response.ok) {
                throw new Error(`HTTP error! status: ${response.status}`);
            }
            
            const countries = await response.json();
            console.log('Countries loaded:', countries);
            
            if (countries.length === 0) {
                this.countriesTable.clear().draw();
                return;
            }
            
            this.countriesTable.clear().rows.add(countries).draw();
            
        } catch (error) {
            console.error('Error loading countries:', error);
            
            // Fallback to mock data
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
                    "updated_at": "2025-09-29T14:43:29.961383",
                    "updated_by": "admin@example.com"
                },
                {
                    "country_id": 3,
                    "iso_code": "FR",
                    "country_name": "France",
                    "created_at": "2025-09-28T10:30:00.000000",
                    "created_by": "admin@example.com",
                    "updated_at": null,
                    "updated_by": null
                }
            ];
            
            this.countriesTable.clear().rows.add(mockCountries).draw();
        }
    }

    setupEventListeners() {
        console.log('Setting up event listeners');
        
        // Add country form submission
        $('#addCountryForm').on('submit', (e) => {
            e.preventDefault();
            this.addCountry();
        });

        // Edit button click
        $(document).on('click', '#countriesTable .edit-btn', (e) => {
            e.preventDefault();
            e.stopPropagation();
            
            const countryId = $(e.currentTarget).data('id');
            console.log('Edit button clicked for country:', countryId);
            
            if (countryId) {
                this.openEditModal(countryId);
            }
        });

        // Edit form submission
        $('#editCountryForm').on('submit', (e) => {
            e.preventDefault();
            this.updateCountry();
        });

        // Delete button click
        $(document).on('click', '#countriesTable .delete-btn', (e) => {
            e.preventDefault();
            e.stopPropagation();
            
            const countryId = $(e.currentTarget).data('id');
            console.log('Delete button clicked for country:', countryId);
            
            if (countryId) {
                this.deleteCountry(countryId);
            }
        });
    }

    // ADD ACTION: Create new country
    async addCountry() {
        const formData = {
            iso_code: $('#isoCode').val().toUpperCase().trim(),
            country_name: $('#countryName').val().trim(),
            created_by: 'admin@example.com'
        };

        // Validation
        if (!formData.iso_code || !formData.country_name) {
            this.showAlert('Please fill in all required fields', 'danger');
            return;
        }

        if (formData.iso_code.length !== 2) {
            this.showAlert('ISO Code must be exactly 2 characters', 'danger');
            return;
        }

        try {
            console.log('Adding country:', formData);
            
            const response = await fetch(`${this.apiBaseUrl}/countries`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify(formData)
            });

            if (!response.ok) {
                const errorText = await response.text();
                throw new Error(`HTTP error! status: ${response.status}, message: ${errorText}`);
            }

            const newCountry = await response.json();
            console.log('Country added successfully:', newCountry);
            
            // Add the new country to the table
            this.countriesTable.row.add(newCountry).draw();
            
            // Close modal and reset form
            $('#addCountryModal').modal('hide');
            $('#addCountryForm')[0].reset();
            
            this.showAlert('Country added successfully!', 'success');
            
        } catch (error) {
            console.error('Error adding country:', error);
            this.showAlert('Error adding country: ' + error.message, 'danger');
        }
    }

    // OPEN EDIT MODAL: Open modal with country data
    openEditModal(countryId) {
        console.log('Opening edit modal for country:', countryId);
        
        // Find the country data
        const countries = this.countriesTable.rows().data().toArray();
        const country = countries.find(c => c.country_id === parseInt(countryId));
        
        if (!country) {
            this.showAlert('Country not found', 'danger');
            return;
        }

        // Populate the edit form
        $('#editCountryId').val(country.country_id);
        $('#editIsoCode').val(country.iso_code);
        $('#editCountryName').val(country.country_name);

        // Show the modal
        $('#editCountryModal').modal('show');
    }

    // UPDATE COUNTRY: Update country via modal form
    async updateCountry() {
        const countryId = $('#editCountryId').val();
        const formData = {
            iso_code: $('#editIsoCode').val().toUpperCase().trim(),
            country_name: $('#editCountryName').val().trim(),
            updated_by: 'admin@example.com'
        };

        // Validation
        if (!formData.iso_code || !formData.country_name) {
            this.showAlert('Please fill in all required fields', 'danger');
            return;
        }

        if (formData.iso_code.length !== 2) {
            this.showAlert('ISO Code must be exactly 2 characters', 'danger');
            return;
        }

        try {
            console.log('Updating country ID:', countryId, 'with data:', formData);
            
            const response = await fetch(`${this.apiBaseUrl}/countries/${countryId}`, {
                method: 'PUT',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify(formData)
            });

            if (!response.ok) {
                const errorText = await response.text();
                throw new Error(`HTTP error! status: ${response.status}, message: ${errorText}`);
            }

            const updatedCountry = await response.json();
            console.log('Country updated successfully:', updatedCountry);
            
            // Update the row in the table
            const rowIndex = this.findRowIndexByCountryId(countryId);
            if (rowIndex !== -1) {
                this.countriesTable.row(rowIndex).data(updatedCountry).draw();
            }
            
            // Close modal and reset form
            $('#editCountryModal').modal('hide');
            $('#editCountryForm')[0].reset();
            
            this.showAlert('Country updated successfully!', 'success');
            
        } catch (error) {
            console.error('Error updating country:', error);
            this.showAlert('Error updating country: ' + error.message, 'danger');
        }
    }

    // Helper method to find row index by country_id
    findRowIndexByCountryId(countryId) {
        const rows = this.countriesTable.rows().data().toArray();
        for (let i = 0; i < rows.length; i++) {
            if (rows[i].country_id === parseInt(countryId)) {
                return i;
            }
        }
        return -1;
    }

    // DELETE ACTION: Remove a country
    async deleteCountry(countryId) {
        // Get country name for confirmation message
        const rowIndex = this.findRowIndexByCountryId(countryId);
        if (rowIndex === -1) {
            this.showAlert('Country not found', 'danger');
            return;
        }
        
        const rowData = this.countriesTable.row(rowIndex).data();
        const countryName = rowData.country_name;
        
        if (!confirm(`Are you sure you want to delete "${countryName}"? This action cannot be undone.`)) {
            return;
        }

        try {
            console.log('Deleting country ID:', countryId);
            
            const response = await fetch(`${this.apiBaseUrl}/countries/${countryId}`, {
                method: 'DELETE'
            });

            if (!response.ok) {
                const errorText = await response.text();
                throw new Error(`HTTP error! status: ${response.status}, message: ${errorText}`);
            }

            // Remove the row from the table
            this.countriesTable.row(rowIndex).remove().draw();
            
            console.log('Country deleted successfully');
            this.showAlert('Country deleted successfully!', 'success');
            
        } catch (error) {
            console.error('Error deleting country:', error);
            this.showAlert('Error deleting country: ' + error.message, 'danger');
        }
    }

    // Show alert message
    showAlert(message, type = 'info') {
        // Remove existing alerts
        $('.alert-dismissible').remove();
        
        const alertClass = `alert alert-${type} alert-dismissible`;
        const alertHtml = `
            <div class="${alertClass}">
                <button type="button" class="close" data-dismiss="alert" aria-hidden="true">×</button>
                <h5><i class="icon fas fa-${type === 'success' ? 'check' : type === 'danger' ? 'ban' : 'info'}"></i> ${type.charAt(0).toUpperCase() + type.slice(1)}</h5>
                ${message}
            </div>
        `;
        
        $('.content-header').after(alertHtml);
        
        // Auto remove after 5 seconds
        setTimeout(() => {
            $('.alert-dismissible').fadeOut();
        }, 5000);
    }
}

// Initialize when document is ready
$(document).ready(function() {
    console.log('Document ready, initializing CountriesManager...');
    window.countriesManager = new CountriesManager();
});