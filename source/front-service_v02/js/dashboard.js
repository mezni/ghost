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

// Call metrics on page load
document.addEventListener('DOMContentLoaded', () => {
    updateMetric("IN", "roamers-in-value", "roamers-in-date");
    updateMetric("OUT", "roamers-out-value", "roamers-out-date");
});
