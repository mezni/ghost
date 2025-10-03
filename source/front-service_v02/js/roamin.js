// Function to load HTML into an element by ID
function loadHTML(id, url) {
    fetch(url)
        .then(response => {
            if (!response.ok) {
                throw new Error(`Failed to load ${url}`);
            }
            return response.text();
        })
        .then(html => {
            document.getElementById(id).innerHTML = html;
        })
        .catch(err => console.warn('Error loading HTML:', err));
}

// Function to fetch data for the line chart
async function fetchLineData() {
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
        const response = await fetch(API_URL, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(requestData)
        });

        const result = await response.json();
        console.log(result);  // Log the API response

        if (result.status === 'success' && result.data.length > 0) {
            const lineData = result.data.map(item => {
                const parsedDate = new Date(item.date);
                console.log(parsedDate);  // Log the parsed date to make sure it's correct
                return {
                    x: parsedDate.getTime(),  // Convert the string date to timestamp
                    y: item.value
                };
            });
            updateLineChart(lineData);
        } else {
            console.warn('No data returned for line chart');
        }
    } catch (error) {
        console.error('Error fetching line chart data:', error);
    }
}

// Function to update the line chart
function updateLineChart(lineData) {
    const lineCtx = document.getElementById('lineChart').getContext('2d');
    const lineChart = new Chart(lineCtx, {
        type: 'line',  // Set chart type to line
        data: {
            datasets: [{
                label: 'Roam IN',
                data: lineData,
                backgroundColor: 'rgba(54, 162, 235, 0.2)', // Light blue fill
                borderColor: 'rgba(54, 162, 235, 1)',     // Blue line
                borderWidth: 2,
                fill: true // Optional: Add fill under the line chart
            }]
        },
        options: {
            responsive: true,
            plugins: {
                legend: {
                    position: 'top',
                },
                tooltip: {
                    enabled: true,
                    callbacks: {
                        label: function(tooltipItem) {
                            const date = new Date(tooltipItem.raw.x);
                            return `Date: ${date.toLocaleDateString()} - Value: ${tooltipItem.raw.y}`;
                        }
                    }
                }
            },
            scales: {
                x: {
                    type: 'time',  // Use time scale for x-axis
                    time: {
                        unit: 'day',  // Display time in days
                        tooltipFormat: 'll',  // Format date in tooltips
                        displayFormats: {
                            day: 'MMM DD, YYYY', // Format the date on the x-axis
                        },
                    },
                    title: {
                        display: true,
                        text: 'Date'
                    }
                },
                y: {
                    beginAtZero: true,
                    title: {
                        display: true,
                        text: 'Value'
                    }
                }
            }
        }
    });
}

// Function to fetch data for the pie chart
async function fetchPieData() {
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
        const response = await fetch(API_URL, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(requestData)
        });

        const result = await response.json();
        if (result.status === 'success' && result.data.length > 0) {
            const pieData = result.data.reduce((acc, item) => {
                acc.labels.push(item.date);
                acc.data.push(item.value);
                return acc;
            }, { labels: [], data: [] });

            updatePieChart(pieData);
        } else {
            console.warn('No data returned for pie chart');
        }
    } catch (error) {
        console.error('Error fetching pie chart data:', error);
    }
}

// Function to update the pie chart
function updatePieChart(pieData) {
    const pieCtx = document.getElementById('pieChart').getContext('2d');
    const pieChart = new Chart(pieCtx, {
        type: 'pie',
        data: {
            labels: pieData.labels,
            datasets: [{
                label: 'Roam IN',
                data: pieData.data,
                backgroundColor: ['rgba(54, 162, 235, 0.6)', 'rgba(255, 159, 64, 0.6)', 'rgba(255, 99, 132, 0.6)', 'rgba(75, 192, 192, 0.6)'],
                borderColor: 'rgba(0,0,0,0.1)',
                borderWidth: 1
            }]
        },
        options: {
            responsive: true,
            plugins: {
                legend: {
                    position: 'top',
                },
                tooltip: {
                    enabled: true
                }
            }
        }
    });
}

// Call the functions on page load
document.addEventListener('DOMContentLoaded', () => {
    // Load common HTML parts (header, sidebar, footer)
    loadHTML('header-placeholder', '../pages/header.html');
    loadHTML('sidebar-placeholder', '../pages/sidebar.html');
    loadHTML('footer-placeholder', '../pages/footer.html');

    // Fetch data for charts
    fetchLineData();
    fetchPieData();
});
