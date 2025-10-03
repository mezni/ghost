// API base URL
const API_URL = "http://localhost:3000/api/v1/metrics";

// Generic function to fetch a metric and update a stat box
async function updateMetric(direction, valueId, dateId) {
    try {
        const response = await fetch(API_URL, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({
                type: "Metric",
                dataset: { aggregation: "Global", direction },
                timePeriod: {},
                filter: {}
            })
        });

        const result = await response.json();
        if (result.status === 'success' && result.data.length > 0) {
            const metric = result.data[0];
            document.getElementById(valueId).innerText = metric.value.toLocaleString();
            document.getElementById(dateId).innerText = `Last refresh: ${metric.date}`;
        } else {
            console.warn(`No data returned for ${direction}`);
        }
    } catch (error) {
        console.error(`Error fetching ${direction} metric:`, error);
    }
}

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

// Load the common parts (header, sidebar, footer)
document.addEventListener("DOMContentLoaded", function() {
    loadHTML('header-placeholder', './pages/header.html');  // Path to your header file
    loadHTML('sidebar-placeholder', './pages/sidebar.html');  // Path to your sidebar file
    loadHTML('footer-placeholder', './pages/footer.html');  // Path to your footer file

    // Load the specific content for this page (dashboard content)
    loadHTML('main-content-placeholder', './pages/dashboard-content.html');  // Path to your dashboard content

    // Fetch and update the metrics
    updateMetric("IN", "roamers-in-value", "roamers-in-date");
    updateMetric("OUT", "roamers-out-value", "roamers-out-date");
});
