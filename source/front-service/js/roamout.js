document.addEventListener('DOMContentLoaded', () => {
    // Initialize the page once it has loaded
    loadGlobalRoamersOutData();
    loadGlobalRoamersOutByCountryData();
});

// Function to fetch Roamers OUT data from API
function loadGlobalRoamersOutData() {
    const url = 'http://localhost:3000/api/v1/metrics'; // API endpoint

    const requestData = {
        type: "Metric",
        dataset: {
            aggregation: "Global",
            direction: "OUT"
        },
        timePeriod: { window: 10 },
        filter: {}
    };

    // Make the API call for Roamers OUT
    fetch(url, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(requestData)
    })
    .then(response => response.json()) // Parse the JSON response
    .then(data => {
        if (data.status === 'success' && data.data.length > 0) {
            const roamersOutData = processRoamersData(data.data);
            plotRoamersOutLineChart(roamersOutData);
        } else {
            console.error('Failed to retrieve Roamers OUT data');
        }
    })
    .catch(error => {
        console.error('Error fetching Roamers OUT data:', error);
    });
}

// Function to fetch Roamers OUT by Country data from API
function loadGlobalRoamersOutByCountryData() {
    const url = 'http://localhost:3000/api/v1/metrics'; // API endpoint

    const requestData = {
        type: "Metric",
        dataset: {
            aggregation: "Country",
            direction: "OUT"
        },
        timePeriod: {},
        filter: {}
    };

    // Make the API call for Roamers OUT by Country
    fetch(url, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(requestData)
    })
    .then(response => response.json()) // Parse the JSON response
    .then(data => {
        if (data.status === 'success' && data.data.length > 0) {
            const countryData = processCountryData(data.data);
            plotRoamersOutPieChart(countryData);
        } else {
            console.error('Failed to retrieve Roamers OUT by Country data');
        }
    })
    .catch(error => {
        console.error('Error fetching Roamers OUT by Country data:', error);
    });
}

// Process Country data to extract country names and values for the pie chart
function processCountryData(data) {
    const processedData = {
        countries: [],  // To store country names
        values: []      // To store the values for each country
    };

    data.forEach(item => {
        processedData.countries.push(item.country);
        processedData.values.push(item.value);
    });

    return processedData;
}

// Function to plot Roamers OUT by Country data on a pie chart
function plotRoamersOutPieChart(data) {
    const ctx = document.getElementById('roamersOutPieChart').getContext('2d');

    // Generate colors for each country
    const backgroundColors = generateColors(data.countries.length);

    const chartData = {
        labels: data.countries,
        datasets: [{
            data: data.values,
            backgroundColor: backgroundColors,
            borderColor: backgroundColors.map(color => color.replace('0.7', '1')), // Darker borders
            borderWidth: 2,
            hoverOffset: 15
        }]
    };

    // Create or update the pie chart
    new Chart(ctx, {
        type: 'pie',
        data: chartData,
        options: {
            responsive: true,
            maintainAspectRatio: false,
            plugins: {
                legend: {
                    position: 'bottom',
                    labels: {
                        padding: 20,
                        usePointStyle: true,
                        pointStyle: 'circle'
                    }
                },
                tooltip: {
                    callbacks: {
                        label: function(context) {
                            const label = context.label || '';
                            const value = context.raw || 0;
                            const total = context.dataset.data.reduce((a, b) => a + b, 0);
                            const percentage = Math.round((value / total) * 100);
                            return `${label}: ${value.toLocaleString()} (${percentage}%)`;
                        }
                    }
                }
            }
        }
    });
}

// Helper function to generate colors for the pie chart
function generateColors(count) {
    const baseColors = [
        'rgba(255, 99, 132, 0.7)',    // Red
        'rgba(54, 162, 235, 0.7)',    // Blue
        'rgba(255, 206, 86, 0.7)',    // Yellow
        'rgba(75, 192, 192, 0.7)',    // Green
        'rgba(153, 102, 255, 0.7)',   // Purple
        'rgba(255, 159, 64, 0.7)',    // Orange
        'rgba(199, 199, 199, 0.7)',   // Gray
        'rgba(83, 102, 255, 0.7)',    // Indigo
        'rgba(40, 159, 64, 0.7)',     // Dark Green
        'rgba(210, 105, 30, 0.7)'     // Chocolate
    ];

    // If we need more colors than available in baseColors, generate random ones
    if (count <= baseColors.length) {
        return baseColors.slice(0, count);
    }

    const colors = [...baseColors];
    for (let i = baseColors.length; i < count; i++) {
        const r = Math.floor(Math.random() * 255);
        const g = Math.floor(Math.random() * 255);
        const b = Math.floor(Math.random() * 255);
        colors.push(`rgba(${r}, ${g}, ${b}, 0.7)`);
    }

    return colors;
}

// Process Roamers OUT data to extract time series data for the chart
function processRoamersData(data) {
    const processedData = {
        labels: [],  // To store the dates for x-axis
        values: []   // To store the values for y-axis
    };

    data.forEach(item => {
        const date = formatDate(item.date);  // Format the date from API response
        processedData.labels.push(date);
        processedData.values.push(item.value);
    });

    return processedData;
}

// Function to plot Roamers OUT data on a line chart
function plotRoamersOutLineChart(data) {
    const ctx = document.getElementById('roamersOutLineChart').getContext('2d');

    const chartData = {
        labels: data.labels,
        datasets: [{
            label: 'Roamers OUT',
            data: data.values,
            borderColor: 'rgba(255, 99, 132, 1)', // Red color
            backgroundColor: 'rgba(255, 99, 132, 0.2)', // Light Red color
            fill: true,
            tension: 0.4,
            borderWidth: 2
        }]
    };

    // Create or update the line chart
    new Chart(ctx, {
        type: 'line',
        data: chartData,
        options: {
            responsive: true,
            maintainAspectRatio: false,
            scales: {
                x: {
                    title: {
                        display: true,
                        text: 'Date'
                    },
                    ticks: {
                        autoSkip: true,
                        maxTicksLimit: 7
                    }
                },
                y: {
                    beginAtZero: false,
                    title: {
                        display: true,
                        text: 'Value'
                    },
                    ticks: {
                        stepSize: Math.ceil(Math.max(...data.values) / 5), // Scale the step size dynamically
                        callback: function(value) {
                            return value.toLocaleString();  // Format numbers with commas
                        }
                    }
                }
            }
        }
    });
}

// Function to format the date from "YYYY-MM-DD" to "MM/DD/YYYY"
function formatDate(dateStr) {
    const dateParts = dateStr.split('-'); // Split the string by "-"
    const year = dateParts[0]; // First part is year
    const month = String(dateParts[1]).padStart(2, '0'); // Second part is month, padded to 2 digits
    const day = String(dateParts[2]).padStart(2, '0'); // Third part is day, padded to 2 digits

    return `${day}/${month}/${year}`; // Return date in DD/MM/YYYY format
}
