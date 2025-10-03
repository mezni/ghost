// Chart instances
let lineChart = null;
let pieChart = null;

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

// Function to fetch data for charts
async function fetchChartData() {
    const API_URL = "http://localhost:3000/api/v1/metrics"; // Replace with your actual API URL
    const requestData = {
        type: "Metric",
        dataset: {
            aggregation: "Global",
            direction: "OUT"
        },
        timePeriod: { window: 10 },
        filter: {}
    };

    try {
        console.log('Fetching data from API...');
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

        // Check if data exists and has expected structure
        if (result && result.data && Array.isArray(result.data) && result.data.length > 0) {
            processChartData(result.data);
        } else {
            console.warn('No valid data returned from API, using sample data');
            createSampleData();
        }
    } catch (error) {
        console.error('Error fetching chart data:', error);
        // Create sample data for testing when API fails
        createSampleData();
    }
}

// Process data for both charts
function processChartData(data) {
    console.log('Processing data:', data);
    
    // Sort data by date to ensure proper ordering
    const sortedData = [...data].sort((a, b) => {
        const dateA = new Date(a.date || a.timestamp || Date.now());
        const dateB = new Date(b.date || b.timestamp || Date.now());
        return dateA - dateB;
    });

    // Line chart data - use string dates instead of timestamps to avoid date adapter issues
    const lineData = sortedData.map(item => {
        let dateString;
        if (item.date) {
            // Parse and format date as string to avoid date adapter issues
            const date = new Date(item.date);
            dateString = date.toISOString().split('T')[0]; // YYYY-MM-DD format
        } else {
            dateString = new Date().toISOString().split('T')[0];
        }
        
        return {
            x: dateString, // Use string instead of timestamp
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

// Create sample data for testing
function createSampleData() {
    console.log('Creating sample data for testing');
    
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

// Function to update the line chart with scaling options
function updateLineChart(lineData) {
    const lineCtx = document.getElementById('lineChartRoamOut');
    
    if (!lineCtx) {
        console.error('Line chart canvas element not found');
        return;
    }

    // Destroy existing chart
    lineChart = destroyChart(lineChart);

    // Use category scale instead of time scale to avoid date adapter issues
    lineChart = new Chart(lineCtx, {
        type: 'line',
        data: {
            datasets: [{
                label: 'Roam OUT',
                data: lineData,
                backgroundColor: 'rgba(75, 192, 192, 0.2)', // Light green area under the line
                borderColor: 'rgba(75, 192, 192, 1)', // Green line color
                borderWidth: 2,
                fill: true,
                tension: 0.4 // Smooth the line
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
                            // Show the date in tooltip
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
                    type: 'category', // Use category scale instead of time scale
                    title: {
                        display: true,
                        text: 'Date'
                    },
                    ticks: {
                        maxTicksLimit: 7, // Limit the number of ticks to fit the chart
                        autoSkip: true, // Skip some ticks for better readability
                    }
                },
                y: {
                    beginAtZero: false, // Do not force the Y-axis to start at zero
                    suggestedMin: Math.min(...lineData.map(d => d.y)) - 10, // Minimum value of Y axis with a margin
                    suggestedMax: Math.max(...lineData.map(d => d.y)) + 10, // Maximum value of Y axis with a margin
                    title: {
                        display: true,
                        text: 'Value'
                    },
                    ticks: {
                        stepSize: 50, // Step size between ticks
                        maxTicksLimit: 5, // Limit the number of ticks on the Y-axis
                        callback: function(value) {
                            return `${value}`; // Format Y-axis values as numbers
                        }
                    }
                }
            }
        }
    });
}

// Function to update the pie chart
function updatePieChart(pieData) {
    const pieCtx = document.getElementById('pieChartRoamOut');
    
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
                label: 'Roam OUT Values',
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

// Initialize charts when DOM is ready
function initializeCharts() {
    console.log('Initializing charts...');
    
    // Ensure canvas elements exist
    const lineCanvas = document.getElementById('lineChartRoamOut');
    const pieCanvas = document.getElementById('pieChartRoamOut');
    
    if (!lineCanvas || !pieCanvas) {
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
    `;
    document.head.appendChild(style);
    
    // Fetch data
    fetchChartData();
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
