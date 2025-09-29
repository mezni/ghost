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
        try {
            console.log('Loading countries from API...');
            
            const response = await fetch(`${this.apiBaseUrl}/countries`);
            
            if (!response.ok) {
                throw new Error(`HTTP error! status: ${response.status}`);
            }
            
            const countries = await response.json();
            console.log('Countries loaded:', countries);
            
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
                    "created_by": "admin@example.com"
                },
                {
                    "country_id": 2, 
                    "iso_code": "TN",
                    "country_name": "Tunisia",
                    "created_at": "2025-09-29T02:02:09.144547",
                    "created_by": "system"
                }
            ];
            
            this.countriesTable.clear().rows.add(mockCountries).draw();
        }
    }

    setupEventListeners() {
        console.log('Setting up event listeners');
        
        $('#addCountryForm').on('submit', (e) => {
            e.preventDefault();
            this.addCountry();
        });

        $('#editCountryForm').on('submit', (e) => {
            e.preventDefault();
            this.updateCountry();
        });

        $('#countriesTable').on('click', '.edit-btn', (e) => {
            const countryId = $(e.currentTarget).data('id');
            this.editCountry(countryId);
        });

        $('#countriesTable').on('click', '.delete-btn', (e) => {
            const countryId = $(e.currentTarget).data('id');
            this.deleteCountry(countryId);
        });
    }

    async addCountry() {
        const formData = {
            iso_code: $('#isoCode').val().toUpperCase(),
            country_name: $('#countryName').val(),
            created_by: 'admin@example.com'
        };

        try {
            const response = await fetch(`${this.apiBaseUrl}/countries`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify(formData)
            });

            if (!response.ok) throw new Error(`HTTP error! status: ${response.status}`);

            const newCountry = await response.json();
            this.countriesTable.row.add(newCountry).draw();
            $('#addCountryModal').modal('hide');
            $('#addCountryForm')[0].reset();
            
        } catch (error) {
            console.error('Error adding country:', error);
        }
    }

    async editCountry(countryId) {
        try {
            const response = await fetch(`${this.apiBaseUrl}/countries/${countryId}`);
            if (!response.ok) throw new Error(`HTTP error! status: ${response.status}`);
            
            const country = await response.json();
            $('#editCountryId').val(country.country_id);
            $('#editIsoCode').val(country.iso_code);
            $('#editCountryName').val(country.country_name);
            $('#editCountryModal').modal('show');
            
        } catch (error) {
            console.error('Error fetching country:', error);
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
            const response = await fetch(`${this.apiBaseUrl}/countries/${countryId}`, {
                method: 'PUT',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify(formData)
            });

            if (!response.ok) throw new Error(`HTTP error! status: ${response.status}`);

            const updatedCountry = await response.json();
            const rowIndex = this.countriesTable.row((idx, data) => data.country_id === parseInt(countryId)).index();
            if (rowIndex !== undefined) {
                this.countriesTable.row(rowIndex).data(updatedCountry).draw();
            }
            $('#editCountryModal').modal('hide');
            
        } catch (error) {
            console.error('Error updating country:', error);
        }
    }

    async deleteCountry(countryId) {
        const rowData = this.countriesTable.row((idx, data) => data.country_id === parseInt(countryId)).data();
        if (!rowData) return;
        
        if (!confirm(`Delete "${rowData.country_name}"?`)) return;

        try {
            const response = await fetch(`${this.apiBaseUrl}/countries/${countryId}`, {
                method: 'DELETE'
            });

            if (!response.ok) throw new Error(`HTTP error! status: ${response.status}`);

            this.countriesTable.row((idx, data) => data.country_id === parseInt(countryId)).remove().draw();
            
        } catch (error) {
            console.error('Error deleting country:', error);
        }
    }
}

// Initialize when document is ready
$(document).ready(function() {
    console.log('Document ready, initializing CountriesManager...');
    new CountriesManager();
});