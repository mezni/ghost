document.addEventListener('DOMContentLoaded', () => {
    // Initialize the page once it has loaded
    LoadGlobalRoamersInData();
    LoadGlobalRoamersOutData(); // Call function for Roamers OUT
});

// Function to fetch Global Roamers IN data from API
function LoadGlobalRoamersInData() {
    const url = 'http://localhost:3000/api/v1/metrics'; // API endpoint

    const requestData = {
        type: "Metric",
        dataset: {
            aggregation: "Global",
            direction: "IN"
        },
        timePeriod: {},
        filter: {}
    };

    // Make the API call for Global Roamers IN
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
            const roamersInValue = data.data[0].value;
            const lastUpdatedDate = formatDate(data.data[0].date); // Format the date from API response

            // Update the values in the DOM
            document.getElementById('roamers-in-value').textContent = roamersInValue.toLocaleString(); // Format the number with commas
            document.getElementById('roamers-in-date').textContent = `Last refresh: ${lastUpdatedDate}`;
        } else {
            console.error('Failed to retrieve Global Roamers IN data');
            // Set fallback values in case of failure
            document.getElementById('roamers-in-value').textContent = 'N/A';
            document.getElementById('roamers-in-date').textContent = 'Last refresh: N/A';
        }
    })
    .catch(error => {
        console.error('Error fetching Global Roamers IN data:', error);
        // Handle error and set fallback values
        document.getElementById('roamers-in-value').textContent = 'N/A';
        document.getElementById('roamers-in-date').textContent = 'Last refresh: N/A';
    });
}

// Function to fetch Global Roamers OUT data from API
function LoadGlobalRoamersOutData() {
    const url = 'http://localhost:3000/api/v1/metrics'; // API endpoint

    const requestData = {
        type: "Metric",
        dataset: {
            aggregation: "Global",
            direction: "OUT"
        },
        timePeriod: {},
        filter: {}
    };

    // Make the API call for Global Roamers OUT
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
            const roamersOutValue = data.data[0].value;
            const lastUpdatedDate = formatDate(data.data[0].date); // Format the date from API response

            // Update the values in the DOM
            document.getElementById('roamers-out-value').textContent = roamersOutValue.toLocaleString(); // Format the number with commas
            document.getElementById('roamers-out-date').textContent = `Last refresh: ${lastUpdatedDate}`;
        } else {
            console.error('Failed to retrieve Global Roamers OUT data');
            // Set fallback values in case of failure
            document.getElementById('roamers-out-value').textContent = 'N/A';
            document.getElementById('roamers-out-date').textContent = 'Last refresh: N/A';
        }
    })
    .catch(error => {
        console.error('Error fetching Global Roamers OUT data:', error);
        // Handle error and set fallback values
        document.getElementById('roamers-out-value').textContent = 'N/A';
        document.getElementById('roamers-out-date').textContent = 'Last refresh: N/A';
    });
}

// Function to format the date from "YYYY-DD-MM" to "DD/MM/YYYY"
function formatDate(dateStr) {
    const dateParts = dateStr.split('-'); // Split the string by "-"
    const year = dateParts[0]; // First part is year
    const month = String(dateParts[1]).padStart(2, '0'); // Second part is month, padded to 2 digits
    const day = String(dateParts[2]).padStart(2, '0'); // Third part is day, padded to 2 digits

    return `${day}/${month}/${year}`; // Return date in DD/MM/YYYY format
}


