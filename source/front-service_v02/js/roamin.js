/**
 * Roam IN Page Controller
 * Fetches last 30 days Roam IN data and renders charts
 */

const API_URL = "http://localhost:3000/api/v1/metrics";

async function roaminInit() {
  console.log("💠 Roam IN page initialized");

  if (typeof Chart === "undefined") {
    console.error("Chart.js is not loaded yet.");
    AppUtils.notify("Error", "Chart.js library not loaded", "danger");
    return;
  }

  const lineChartEl = document.getElementById("global-chart");
  const pieChartEl = document.getElementById("country-pie-chart");
  const countryFilter = document.getElementById("countryFilter");

  if (!lineChartEl) console.warn("Canvas #global-chart not found. Skipping line chart.");
  if (!pieChartEl) console.warn("Canvas #country-pie-chart not found. Skipping pie chart.");

  try {
    if (lineChartEl) await fetchRoamInTrend();
    if (pieChartEl) await fetchTopCountriesPie();
    if (countryFilter) {
      await loadCountryFilter();
      setupCountryFilterListener();
    }
  } catch (err) {
    console.error("Error initializing Roam IN charts:", err);
    AppUtils.notify("Error", "Failed to load Roam IN or Top Countries data", "danger");
  }
}

// ------------------------------
// Fetch 30-day Roam IN data (Global Trend)
// ------------------------------
async function fetchRoamInTrend() {
  console.log("Fetching Roam IN global trend...");
  try {
    const response = await fetch(API_URL, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        dimension: "global",
        aggregation: "history",
        filter: [{ "key": "direction", "value": "in" }],
      }),
    });

    const data = await response.json();
    console.log("Roam IN Trend API response:", data);

    if (data.status !== "success" || !data.data || data.data.length === 0) {
      console.warn("No trend data returned from API.");
      renderRoamInChart([], []);
      return;
    }

    const labels = data.data.map(item => item.date);
    const values = data.data.map(item => item.value);

    renderRoamInChart(labels, values);
  } catch (err) {
    console.error("Error fetching Roam IN trend:", err);
    AppUtils.notify("Error", "Failed to fetch Roam IN trend", "danger");
  }
}

// ------------------------------
// Render Roam IN Global Trend Line Chart
// ------------------------------
function renderRoamInChart(labels, values) {
  const ctx = document.getElementById("global-chart").getContext("2d");

  if (window.roamInChart) window.roamInChart.destroy();

  window.roamInChart = new Chart(ctx, {
    type: "line",
    data: {
      labels,
      datasets: [{
        label: "Roam IN (Last 30 Days)",
        data: values,
        borderColor: "#007bff",
        backgroundColor: "rgba(0,123,255,0.1)",
        fill: true,
        tension: 0.3,
        pointRadius: 3,
      }],
    },
    options: {
      responsive: true,
      maintainAspectRatio: false,
      scales: {
        x: { ticks: { color: "#6c757d" }, title: { display: true, text: "Date", color: "#6c757d" } },
        y: { ticks: { color: "#6c757d" }, title: { display: true, text: "Roam IN Count", color: "#6c757d" }, beginAtZero: false, grace: "5%" },
      },
      plugins: {
        legend: { display: true, position: "bottom" },
        tooltip: { mode: "index", intersect: false },
      },
    },
  });
}

// ------------------------------
// Fetch Top 5 countries (Pie Chart)
// ------------------------------
async function fetchTopCountriesPie() {
  console.log("Fetching Top 5 countries...");
  try {
    const response = await fetch(API_URL, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        metric: "metric",
        dimension: "country",
        direction: "IN",
        aggregation: { measure: "Top", size: 5 }, // fixed typo: "measure" not "mesure"
      }),
    });

    const data = await response.json();
    console.log("Top countries API response:", data);

    if (data.status !== "success" || !data.data || data.data.length === 0) {
      console.warn("No top countries data returned.");
      renderCountriesPieChart([], [], []);
      return;
    }

    const labels = data.data.map(item => item.country);
    const values = data.data.map(item => item.value);
    const colors = ["#007bff", "#28a745", "#dc3545", "#ffc107", "#17a2b8"];

    renderCountriesPieChart(labels, values, colors);
  } catch (err) {
    console.error("Error fetching Top Countries:", err);
    AppUtils.notify("Error", "Failed to fetch Top Countries", "danger");
  }
}

// ------------------------------
// Render Top 5 countries pie chart
// ------------------------------
function renderCountriesPieChart(labels, values, colors) {
  const ctx = document.getElementById("country-pie-chart").getContext("2d");

  if (window.countryPieChart) window.countryPieChart.destroy();

  window.countryPieChart = new Chart(ctx, {
    type: "pie",
    data: { labels, datasets: [{ data: values, backgroundColor: colors }] },
    options: { responsive: true, maintainAspectRatio: false, plugins: { legend: { position: "bottom" } } },
  });
}

// ------------------------------
// COUNTRY FILTER
// ------------------------------
async function loadCountryFilter() {
  console.log("Loading country filter...");
  try {
    const response = await fetch(API_URL, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ metric: "definition", dimension: "country", direction: "IN" }),
    });

    const json = await response.json();
    console.log("Country filter API response:", json);

    if (json.status !== "success" || !json.data || !json.data.data) {
      console.warn("No countries returned from API.");
      return;
    }

    const select = document.getElementById("countryFilter");
    select.innerHTML = '<option value="all" selected>All</option>';

    json.data.data.forEach(country => {
      const option = document.createElement("option");
      option.value = country;
      option.textContent = country;
      select.appendChild(option);
    });
  } catch (err) {
    console.error("Error loading country filter:", err);
    AppUtils.notify("Error", "Failed to load country list", "danger");
  }
}

function setupCountryFilterListener() {
  const select = document.getElementById("countryFilter");
  if (!select) return;

  select.addEventListener("change", async (e) => {
    const selectedCountry = e.target.value;
    await fetchCountryTrend(selectedCountry);
  });
}

// ------------------------------
// Fetch and render line chart for selected country
// ------------------------------
async function fetchCountryTrend(country) {
  try {
    const body = { metric: "metric", dimension: "country", direction: "IN" };
    if (country !== "all") body.filter = [{ key: "country", value: country }];

    const response = await fetch(API_URL, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(body),
    });

    const json = await response.json();
    console.log(`Country trend API response for ${country}:`, json);

    if (!json.data || json.data.length === 0) {
      AppUtils.notify("Info", `No data for ${country}`, "info");
      renderCountryLineChart([], [], country);
      return;
    }

    const labels = json.data.map(item => item.date);
    const values = json.data.map(item => item.value);

    renderCountryLineChart(labels, values, country);
  } catch (err) {
    console.error("Error fetching country trend:", err);
    AppUtils.notify("Error", "Failed to load country trend", "danger");
  }
}

// ------------------------------
// Render Country Trend line chart
// ------------------------------
function renderCountryLineChart(labels, values, country) {
  const ctx = document.getElementById("country-line-chart").getContext("2d");

  if (window.countryLineChart) window.countryLineChart.destroy();

  window.countryLineChart = new Chart(ctx, {
    type: "line",
    data: {
      labels,
      datasets: [{
        label: country === "all" ? "All Countries" : `Roam IN - ${country}`,
        data: values,
        borderColor: "#28a745",
        backgroundColor: "rgba(40,167,69,0.1)",
        fill: true,
        tension: 0.3,
        pointRadius: 3,
      }],
    },
    options: {
      responsive: true,
      maintainAspectRatio: false,
      scales: {
        x: { ticks: { color: "#6c757d" }, title: { display: true, text: "Date", color: "#6c757d" } },
        y: { ticks: { color: "#6c757d" }, title: { display: true, text: "Roam IN Count", color: "#6c757d" }, beginAtZero: false, grace: "5%" },
      },
      plugins: { legend: { display: true, position: "bottom" }, tooltip: { mode: "index", intersect: false } },
    },
  });
}

// ------------------------------
// Expose globally
// ------------------------------
window.roaminInit = roaminInit;
