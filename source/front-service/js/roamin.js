/**
 * Roam IN Page Script
 * Handles charts and country filter logic
 */

const API_URL = "http://localhost:3000/api/v1"; // ✅ central variable

async function roaminInit() {
  console.log("💠 Roam IN page initialized");

  if (typeof Chart === "undefined") {
    console.error("Chart.js not loaded.");
    return;
  }

  await loadRoamInLineChart();
  await loadRoamInPieChart();
  await loadCountryFilter(); // ✅ load dropdown + chart
}

/**
 * Format date to dd/mm/yyyy
 */
function formatDateDDMMYYYY(dateStr) {
  const d = new Date(dateStr);
  const dd = String(d.getDate()).padStart(2, "0");
  const mm = String(d.getMonth() + 1).padStart(2, "0");
  const yyyy = d.getFullYear();
  return `${dd}/${mm}/${yyyy}`;
}

/**
 * Load Line Chart - Roam IN Global History
 */
async function loadRoamInLineChart() {
  const ctx = document.getElementById("roamInLineChart");
  if (!ctx) return;

  try {
    const res = await fetch(`${API_URL}/metrics`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        dimension: "global",
        aggregation: "history",
        filter: [{ key: "direction", value: "in" }]
      })
    });

    const data = await res.json();
    const labels = data.data.map(item => formatDateDDMMYYYY(item.date));
    const values = data.data.map(item => item.value);

    renderLineChart(ctx, "Roam IN", labels, values, "#007bff");
  } catch (err) {
    console.error("Failed to load line chart:", err);
  }
}

/**
 * Load Pie Chart - Roam IN by Country
 */
async function loadRoamInPieChart() {
  const ctx = document.getElementById("roamInPieChart");
  if (!ctx) return;

  try {
    const res = await fetch(`${API_URL}/metrics`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        dimension: "country",
        aggregation: "top",
        filter: [{ key: "direction", value: "in" }]
      })
    });

    const data = await res.json();
    const labels = data.data.map(item => item.country);
    const values = data.data.map(item => item.value);

    const colors = labels.map((_, i) => `hsl(${(i * 360) / labels.length}, 70%, 55%)`);

    new Chart(ctx, {
      type: "pie",
      data: {
        labels,
        datasets: [
          {
            data: values,
            backgroundColor: colors,
            borderColor: "#fff",
            borderWidth: 2
          }
        ]
      },
      options: {
        responsive: true,
        plugins: {
          legend: { position: "bottom" },
          tooltip: {
            callbacks: {
              label: (context) => `${context.label}: ${context.parsed.toLocaleString()}`
            }
          }
        }
      }
    });
  } catch (err) {
    console.error("Failed to load pie chart:", err);
  }
}

/**
 * Load Country Filter (Dropdown)
 */
async function loadCountryFilter() {
  const select = document.getElementById("countryFilter");
  if (!select) return;

  try {
    const res = await fetch(`${API_URL}/countries`);
    const countries = await res.json();

    // ✅ Add default "All" option
    select.innerHTML = `<option value="all">All</option>`;

    countries.forEach(country => {
      const opt = document.createElement("option");
      opt.value = country.country_name;
      opt.textContent = country.country_name;
      select.appendChild(opt);
    });

    // ✅ Load initial chart (All countries)
    await loadCountryLineChart("all");

    // ✅ Event listener for filter changes
    select.addEventListener("change", async (e) => {
      const selected = e.target.value;

      // 🔹 Update pie chart title dynamically
      const titleEl = document.getElementById("distributionTitle");
      if (titleEl) {
        titleEl.textContent =
          selected === "all"
            ? "Distribution"
            : `Distribution (${selected})`;
      }

      // 🔹 Reload the line chart for the selected country
      await loadCountryLineChart(selected);
    });
  } catch (err) {
    console.error("Failed to load countries:", err);
  }
}

/**
 * Load Country Line Chart (Dynamic)
 */
async function loadCountryLineChart(countryName = "all") {
  const ctx = document.getElementById("country-line-chart");
  if (!ctx) return;

  try {
    const payload =
      countryName === "all"
        ? {
            dimension: "global",
            aggregation: "history",
            filter: [{ key: "direction", value: "in" }]
          }
        : {
            dimension: "country",
            aggregation: "history",
            filter: [
              { key: "direction", value: "in" },
              { key: "country", value: countryName }
            ]
          };

    const res = await fetch(`${API_URL}/metrics`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(payload)
    });

    const data = await res.json();
    const labels = data.data.map(item => formatDateDDMMYYYY(item.date));
    const values = data.data.map(item => item.value);

    renderLineChart(ctx, countryName === "all" ? "All Countries" : countryName, labels, values, "#28a745");
  } catch (err) {
    console.error("Failed to load country line chart:", err);
  }
}

/**
 * Render Line Chart (Reusable)
 */
function renderLineChart(ctx, label, labels, values, color) {
  const minValue = Math.min(...values);
  const maxValue = Math.max(...values);
  const padding = (maxValue - minValue) * 0.1;

  // Destroy existing chart instance if any (to prevent overlap)
  if (ctx.chartInstance) {
    ctx.chartInstance.destroy();
  }

  ctx.chartInstance = new Chart(ctx, {
    type: "line",
    data: {
      labels,
      datasets: [
        {
          label,
          data: values,
          borderColor: color,
          backgroundColor: `${color}33`,
          fill: true,
          tension: 0.3,
          pointRadius: 3
        }
      ]
    },
    options: {
      responsive: true,
      maintainAspectRatio: false,
      plugins: { legend: { display: true } },
      scales: {
        x: { title: { display: false } }, // ✅ no x-axis title
        y: {
          title: { display: true, text: "Value" },
          beginAtZero: false,
          min: minValue - padding,
          max: maxValue + padding
        }
      }
    }
  });
}
