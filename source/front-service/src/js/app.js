// Simplified app.js - No API calls
$(document).ready(function() {
    console.log('AdminLTE App Initialized');
    $('.preloader').fadeOut();
    initializeApp();
});

function initializeApp() {
    // Set static values for dashboard
    $('#countries-count').text('2');
    $('#countries-badge').text('2');
    $('#operators-count').text('0');
    
    // Show static recent countries
    renderStaticCountries();
}

function renderStaticCountries() {
    const container = $('#recent-countries');
    const staticCountries = [
        { country_name: 'United States', iso_code: 'US', created_by: 'admin' },
        { country_name: 'Tunisia', iso_code: 'TN', created_by: 'system' }
    ];
    
    const html = staticCountries.map(country => `
        <li class="item">
            <div class="product-info">
                <a href="pages/countries.html" class="product-title">
                    ${country.country_name}
                    <span class="badge badge-info float-right">${country.iso_code}</span>
                </a>
                <span class="product-description">
                    Created by ${country.created_by}
                </span>
            </div>
        </li>
    `).join('');
    
    container.html(html);
}

// Make functions globally available
window.refreshDashboard = initializeApp;