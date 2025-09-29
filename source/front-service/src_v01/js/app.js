// API endpoints
const COUNTRIES_API = 'http://127.0.0.1:3000/api/v1/countries';
// const OPERATORS_API = 'http://0.0.0.0:3000/api/v1/operators';

// Global data storage
let appData = {
    countries: [],
    operators: []
};

// Initialize application
$(document).ready(function() {
    console.log('AdminLTE App Initialized');
    
    // Initialize AdminLTE components
    $('[data-widget="pushmenu"]').PushMenu('init');
    $('[data-widget="treeview"]').Treeview('init');
    $('.preloader').fadeOut();
    
    initializeApp();
});

function initializeApp() {
    // Load dashboard data
    loadDashboardData();
    
    // Set up navigation
    setupNavigation();
    
    // Check authentication
    checkAuth();
}

function setupNavigation() {
    // Handle sidebar active states
    $('.nav-link').on('click', function() {
        $('.nav-link').removeClass('active');
        $(this).addClass('active');
        
        // Handle treeview parent active state
        if ($(this).parent().hasClass('nav-treeview')) {
            $(this).closest('.nav-item').addClass('menu-open');
            $(this).closest('.nav-item').find('> a').addClass('active');
        }
    });
}

async function loadDashboardData() {
    try {
        console.log('Loading dashboard data...');
        
        // Load countries data
        const countriesResponse = await fetchWithTimeout(COUNTRIES_API, {
            method: 'GET',
            headers: {
                'Content-Type': 'application/json',
            }
        });
        
        if (countriesResponse && countriesResponse.ok) {
            appData.countries = await countriesResponse.json();
            console.log('Countries data loaded:', appData.countries);
            $('#countries-count').text(appData.countries.length);
            $('#countries-badge').text(appData.countries.length);
            renderRecentCountries();
        } else {
            throw new Error('Failed to fetch countries');
        }
    } catch (error) {
        console.error('Error loading countries:', error);
        $('#countries-count').text('0');
        $('#countries-badge').text('0');
        $('#recent-countries').html('<li class="item"><div class="error">Failed to load countries</div></li>');
    }

    // Comment out operators for now since we don't have the API
    /*
    try {
        // Load operators data
        const operatorsResponse = await fetchWithTimeout(OPERATORS_API, {
            method: 'GET',
            headers: {
                'Content-Type': 'application/json',
            }
        });
        
        if (operatorsResponse && operatorsResponse.ok) {
            appData.operators = await operatorsResponse.json();
            $('#operators-count').text(appData.operators.length);
            $('#operators-badge').text(appData.operators.length);
            renderRecentOperators();
        } else {
            throw new Error('Failed to fetch operators');
        }
    } catch (error) {
        console.error('Error loading operators:', error);
        $('#operators-count').text('0');
        $('#operators-badge').text('0');
        $('#recent-operators').html('<li class="item"><div class="error">Failed to load operators</div></li>');
    }
    */
}

// Helper function for fetch with timeout
function fetchWithTimeout(url, options = {}) {
    const timeout = 8000; // 8 seconds
    
    return Promise.race([
        fetch(url, options),
        new Promise((_, reject) =>
            setTimeout(() => reject(new Error('Request timeout')), timeout)
        )
    ]).catch(error => {
        console.error('Fetch error:', error);
        return null;
    });
}

function renderRecentCountries() {
    const container = $('#recent-countries');
    
    if (!appData.countries || !appData.countries.length) {
        container.html('<li class="item"><div class="alert alert-info">No countries data available</div></li>');
        return;
    }

    const recentCountries = appData.countries.slice(-5).reverse(); // Show last 5, most recent first
    const html = recentCountries.map(country => `
        <li class="item">
            <div class="product-info">
                <a href="pages/countries.html" class="product-title">
                    ${country.country_name || 'Unknown Country'}
                    <span class="badge badge-info float-right">${country.iso_code || 'N/A'}</span>
                </a>
                <span class="product-description">
                    Created by ${country.created_by || 'system'} • ${country.created_at ? new Date(country.created_at).toLocaleDateString() : 'N/A'}
                </span>
            </div>
        </li>
    `).join('');
    
    container.html(html);
}

function renderRecentOperators() {
    const container = $('#recent-operators');
    
    // Since we don't have operators API yet, show a message
    container.html('<li class="item"><div class="alert alert-info">No operators data available</div></li>');
    
    /*
    if (!appData.operators || !appData.operators.length) {
        container.html('<li class="item"><div class="alert alert-info">No operators data available</div></li>');
        return;
    }

    const recentOperators = appData.operators.slice(0, 5);
    const html = recentOperators.map(operator => `
        <li class="item">
            <div class="product-info">
                <a href="javascript:void(0)" class="product-title">
                    ${operator.name || 'Unknown Operator'}
                    <span class="badge badge-success float-right">${operator.country || 'N/A'}</span>
                </a>
                <span class="product-description">
                    ${operator.type || 'Unknown Type'} 
                    ${operator.subscribers ? '| ' + (operator.subscribers / 1000).toFixed(0) + 'K subscribers' : ''}
                </span>
            </div>
        </li>
    `).join('');
    
    container.html(html);
    */
}

function checkAuth() {
    const token = localStorage.getItem('authToken');
    if (!token && !window.location.href.includes('index.html')) {
        window.location.href = 'index.html';
    }
}

function showLoading(container) {
    $(container).html('<div class="loading"><i class="fas fa-spinner fa-spin fa-2x"></i><br>Loading...</div>');
}

function showError(container, message) {
    $(container).html(`<div class="error"><i class="fas fa-exclamation-triangle"></i><br>${message}</div>`);
}

// Refresh dashboard data
function refreshDashboard() {
    loadDashboardData();
}

// Make functions globally available
window.refreshDashboard = refreshDashboard;
window.logout = logout;