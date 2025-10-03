// Chart instances
let lineChart = null;
let pieChart = null;
let operatorLineChart = null;

// Function to load HTML into an element by ID
function loadHTML(id, url) {
    fetch(url)
        .then(response => {
            if (!response.ok) {
                throw new Error(`Failed to load ${url}: ${response.status}`);
            }
            return response.text();
        })
        .then(html => {
            document.getElementById(id).innerHTML = html;
        })
        .catch(err => console.warn('Error loading HTML:', err));
}

// Function to safely destroy a chart
function destroyChart(chartInstance) {
    if (chartInstance && typeof chartInstance.destroy === 'function') {
        chartInstance.destroy();
    }
    return null;
}

// Function to fetch main chart data
async function fetchChartData() {
    const API_URL = "http://localhost:3000/api/v1/metrics";
    const requestData = {
        type: "Metric",
        dataset: {
            aggregation: "Global",
            direction: "IN"
        },
        timePeriod: { window: 10 },
        filter: {}
    };

    try {
        console.log('Fetching main chart data from API...');
        const response = await fetch(API_URL, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(requestData)
        });

        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }

        const result = await response.json();
        console.log('API Response:', result);

        if (result && result.data && Array.isArray(result.data) && result.data.length > 0) {
            processChartData(result.data);
        } else {
            console.warn('No valid data returned from API, using sample data');
            createSampleData();
        }
    } catch (error) {
        console.error('Error fetching chart data:', error);
        createSampleData();
    }
}

// Function to fetch operator data (you'll need to implement this based on your API)
async function fetchOperatorData(countryCode = '') {
    const API_URL = "http://localhost:3000/api/v1/metrics/operators"; // Adjust this endpoint
    const requestData = {
        type: "Metric",
        dataset: {
            aggregation: "Operator",
            direction: "IN",
            country: countryCode
        },
        timePeriod: { window: 10 },
        filter: {}
    };

    try {
        console.log('Fetching operator data from API...');
        const response = await fetch(API_URL, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(requestData)
        });

        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }

        const result = await response.json();
        console.log('Operator API Response:', result);

        if (result && result.data) {
            processOperatorData(result.data);
        } else {
            console.warn('No operator data returned, using sample data');
            createSampleOperatorData();
        }
    } catch (error) {
        console.error('Error fetching operator data:', error);
        createSampleOperatorData();
    }
}

// Process data for main charts
function processChartData(data) {
    console.log('Processing main chart data:', data);
    
    const sortedData = [...data].sort((a, b) => {
        const dateA = new Date(a.date || a.timestamp || Date.now());
        const dateB = new Date(b.date || b.timestamp || Date.now());
        return dateA - dateB;
    });

    // Line chart data
    const lineData = sortedData.map(item => {
        let dateString;
        if (item.date) {
            const date = new Date(item.date);
            dateString = date.toISOString().split('T')[0];
        } else {
            dateString = new Date().toISOString().split('T')[0];
        }
        
        return {
            x: dateString,
            y: item.value || item.count || 0
        };
    });

    // Pie chart data
    const pieData = {
        labels: sortedData.map(item => {
            if (item.date) {
                const date = new Date(item.date);
                return isNaN(date.getTime()) ? 'Unknown' : date.toLocaleDateString();
            }
            return 'Unknown';
        }),
        data: sortedData.map(item => item.value || item.count || 0)
    };

    updateLineChart(lineData);
    updatePieChart(pieData);
}

// Process operator data
function processOperatorData(operatorData) {
    console.log('Processing operator data:', operatorData);
    
    // This depends on your API response structure
    // Example structure: { operators: ['Operator1', 'Operator2'], data: { dates: [], values: {} } }
    
    if (operatorData.operators && operatorData.dates) {
        const labels = operatorData.dates;
        const datasets = operatorData.operators.map((operator, index) => {
            const colors = [
                'rgba(54, 162, 235, 1)',
                'rgba(255, 99, 132, 1)',
                'rgba(75, 192, 192, 1)',
                'rgba(255, 159, 64, 1)',
                'rgba(153, 102, 255, 1)'
            ];
            
            return {
                label: operator,
                data: operatorData.values[operator] || [],
                borderColor: colors[index % colors.length],
                backgroundColor: colors[index % colors.length].replace('1)', '0.1)'),
                borderWidth: 2,
                tension: 0.4,
                fill: false
            };
        });
        
        updateOperatorLineChart(labels, datasets);
    } else {
        console.warn('Unexpected operator data structure');
        createSampleOperatorData();
    }
}

// Create sample data for main charts
function createSampleData() {
    console.log('Creating sample main data for testing');
    
    const sampleData = [];
    const now = new Date();
    
    for (let i = 6; i >= 0; i--) {
        const date = new Date(now);
        date.setDate(now.getDate() - i);
        
        sampleData.push({
            date: date.toISOString(),
            value: Math.floor(Math.random() * 100) + 50
        });
    }
    
    processChartData(sampleData);
}

// Create sample operator data
function createSampleOperatorData() {
    console.log('Creating sample operator data for testing');
    
    const operators = ['Operator A', 'Operator B', 'Operator C'];
    const dates = [];
    const now = new Date();
    
    for (let i = 6; i >= 0; i--) {
        const date = new Date(now);
        date.setDate(now.getDate() - i);
        dates.push(date.toISOString().split('T')[0]);
    }
    
    const datasets = operators.map((operator, index) => {
        const colors = [
            'rgba(54, 162, 235, 1)',
            'rgba(255, 99, 132, 1)',
            'rgba(75, 192, 192, 1)'
        ];
        
        const data = dates.map(() => Math.floor(Math.random() * 100) + 20);
        
        return {
            label: operator,
            data: data,
            borderColor: colors[index],
            backgroundColor: colors[index].replace('1)', '0.1)'),
            borderWidth: 2,
            tension: 0.4,
            fill: false
        };
    });
    
    updateOperatorLineChart(dates, datasets);
}

// Function to update the line chart with better scaling
function updateLineChart(lineData) {
    const lineCtx = document.getElementById('lineChart');
    
    if (!lineCtx) {
        console.error('Line chart canvas element not found');
        return;
    }

    // Destroy existing chart
    lineChart = destroyChart(lineChart);

    // Calculate safe min/max values
    const yValues = lineData.map(d => d.y);
    const minY = Math.min(...yValues);
    const maxY = Math.max(...yValues);
    const range = maxY - minY;
    
    // Use category scale instead of time scale to avoid date adapter issues
    lineChart = new Chart(lineCtx, {
        type: 'line',
        data: {
            datasets: [{
                label: 'Roam IN',
                data: lineData,
                backgroundColor: 'rgba(54, 162, 235, 0.2)',
                borderColor: 'rgba(54, 162, 235, 1)',
                borderWidth: 2,
                fill: true,
                tension: 0.4
            }]
        },
        options: {
            responsive: true,
            maintainAspectRatio: false,
            plugins: {
                legend: {
                    position: 'top',
                },
                tooltip: {
                    callbacks: {
                        title: function(tooltipItems) {
                            return `Date: ${tooltipItems[0].label}`;
                        },
                        label: function(context) {
                            return `Value: ${context.parsed.y}`;
                        }
                    }
                }
            },
            scales: {
                x: {
                    type: 'category',
                    title: {
                        display: true,
                        text: 'Date'
                    },
                    ticks: {
                        maxTicksLimit: 7,
                        autoSkip: true,
                    }
                },
                y: {
                    beginAtZero: false,
                    suggestedMin: minY - (range * 0.1), // 10% padding
                    suggestedMax: maxY + (range * 0.1), // 10% padding
                    title: {
                        display: true,
                        text: 'Value'
                    },
                    ticks: {
                        stepSize: Math.ceil(range / 5), // Dynamic step size
                        maxTicksLimit: 6,
                        callback: function(value) {
                            return `${value}`;
                        }
                    }
                }
            }
        }
    });
}

// Function to update the pie chart
function updatePieChart(pieData) {
    const pieCtx = document.getElementById('pieChart');
    
    if (!pieCtx) {
        console.error('Pie chart canvas element not found');
        return;
    }

    // Destroy existing chart
    pieChart = destroyChart(pieChart);

    pieChart = new Chart(pieCtx, {
        type: 'pie',
        data: {
            labels: pieData.labels,
            datasets: [{
                label: 'Roam IN Values',
                data: pieData.data,
                backgroundColor: [
                    'rgba(54, 162, 235, 0.8)',
                    'rgba(255, 159, 64, 0.8)',
                    'rgba(255, 99, 132, 0.8)',
                    'rgba(75, 192, 192, 0.8)',
                    'rgba(153, 102, 255, 0.8)',
                    'rgba(255, 205, 86, 0.8)',
                    'rgba(201, 203, 207, 0.8)'
                ],
                borderColor: 'rgba(255, 255, 255, 0.8)',
                borderWidth: 2
            }]
        },
        options: {
            responsive: true,
            maintainAspectRatio: false,
            plugins: {
                legend: {
                    position: 'top',
                },
                tooltip: {
                    callbacks: {
                        label: function(context) {
                            const label = context.label || '';
                            const value = context.raw || 0;
                            const total = context.dataset.data.reduce((a, b) => a + b, 0);
                            const percentage = Math.round((value / total) * 100);
                            return `${label}: ${value} (${percentage}%)`;
                        }
                    }
                }
            }
        }
    });
}

// Function to update the operator line chart
function updateOperatorLineChart(labels, datasets) {
    const operatorCtx = document.getElementById('operatorLineChart');
    
    if (!operatorCtx) {
        console.error('Operator line chart canvas element not found');
        return;
    }

    // Destroy existing chart
    operatorLineChart = destroyChart(operatorLineChart);

    operatorLineChart = new Chart(operatorCtx, {
        type: 'line',
        data: {
            labels: labels,
            datasets: datasets
        },
        options: {
            responsive: true,
            maintainAspectRatio: false,
            plugins: {
                legend: { 
                    position: 'top',
                    labels: {
                        usePointStyle: true,
                    }
                },
                tooltip: {
                    mode: 'index',
                    intersect: false,
                    callbacks: {
                        title: function(tooltipItems) {
                            return `Date: ${tooltipItems[0].label}`;
                        }
                    }
                }
            },
            scales: {
                x: {
                    type: 'category',
                    title: { display: true, text: 'Date' },
                    ticks: {
                        maxTicksLimit: 7,
                        autoSkip: true,
                    }
                },
                y: {
                    beginAtZero: true,
                    title: { display: true, text: 'Value' },
                    ticks: {
                        callback: function(value) {
                            return `${value}`;
                        }
                    }
                }
            },
            interaction: {
                mode: 'nearest',
                axis: 'x',
                intersect: false
            }
        }
    });
}

// Initialize country dropdown
function initializeCountryDropdown() {
    const countrySelect = document.getElementById('countrySelect');
    
    if (!countrySelect) {
        console.error('Country select element not found');
        return;
    }

    // Sample countries - replace with your actual country data
    const countries = [
        { code: '', name: 'All Countries' },
        { code: 'US', name: 'United States' },
        { code: 'GB', name: 'United Kingdom' },
        { code: 'DE', name: 'Germany' },
        { code: 'FR', name: 'France' },
        { code: 'IT', name: 'Italy' }
    ];

    // Clear existing options
    countrySelect.innerHTML = '<option value="">Select Country</option>';
    
    // Add country options
    countries.forEach(country => {
        const option = document.createElement('option');
        option.value = country.code;
        option.textContent = country.name;
        countrySelect.appendChild(option);
    });

    // Add event listener
    countrySelect.addEventListener('change', function() {
        const selectedCountry = this.value;
        console.log('Country selected:', selectedCountry);
        fetchOperatorData(selectedCountry);
    });
}

// Initialize charts when DOM is ready
function initializeCharts() {
    console.log('Initializing charts...');
    
    // Ensure canvas elements exist
    const lineCanvas = document.getElementById('lineChart');
    const pieCanvas = document.getElementById('pieChart');
    const operatorCanvas = document.getElementById('operatorLineChart');
    
    if (!lineCanvas || !pieCanvas || !operatorCanvas) {
        console.error('Chart canvas elements not found');
        return;
    }
    
    // Add CSS for chart containers
    const style = document.createElement('style');
    style.textContent = `
        .chart-container {
            position: relative;
            height: 250px;
            width: 100%;
        }
        #countrySelect {
            margin-bottom: 20px;
            max-width: 300px;
        }
    `;
    document.head.appendChild(style);
    
    // Initialize country dropdown
    initializeCountryDropdown();
    
    // Fetch main data
    fetchChartData();
    
    // Fetch initial operator data
    fetchOperatorData();
}

// Wait for DOM to be fully loaded
document.addEventListener('DOMContentLoaded', () => {
    console.log('DOM loaded, loading components...');
    
    // Load common HTML parts
    loadHTML('header-placeholder', '../pages/header.html');
    loadHTML('sidebar-placeholder', '../pages/sidebar.html');
    loadHTML('footer-placeholder', '../pages/footer.html');
    
    // Wait for components to load, then initialize charts
    setTimeout(() => {
        initializeCharts();
    }, 500);
});