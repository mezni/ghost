// ✅ Expose the init function on window
window.roamoutInit = async function() {
  console.log("💠 Roam OUT page initialized");

  if (typeof Chart === "undefined") {
    console.error("Chart.js not loaded.");
    return;
  }

  // You can add actual chart-loading calls here
  await loadRoamOutLineChart();
  await loadRoamOutPieChart();
  await loadCountryFilterOut(); 
};


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
 * Load Line Chart - Roam OUT Global History
 */
async function loadRoamOutLineChart() {
  const ctx = document.getElementById("roamOutLineChart");
  if (!ctx) return;

  try {
    const res = await fetch(`${API_URL}/analytics`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        dimension: "global",
        aggregation: "history",
        filter: [{ key: "direction", value: "out" }]
      })
    });

    const data = await res.json();
    const labels = data.data.map(item => formatDateDDMMYYYY(item.date));
    const values = data.data.map(item => item.value);

    renderLineChart(ctx, "Roam OUT", labels, values, "#dc3545");
  } catch (err) {
    console.error("Failed to load line chart:", err);
  }
}

/**
 * Load Pie Chart - Roam OUT by Country
 */
async function loadRoamOutPieChart() {
  const ctx = document.getElementById("roamOutPieChart");
  if (!ctx) return;

  try {
    const res = await fetch(`${API_URL}/analytics`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        dimension: "country",
        aggregation: "top",
        filter: [{ key: "direction", value: "out" }]
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
              label: (context) =>
                `${context.label}: ${context.parsed.toLocaleString()}`
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
async function loadCountryFilterOut() {
  const select = document.getElementById("countryFilterOut");
  if (!select) return;

  try {
    const res = await fetch(`${API_URL}/settings/countries`);
    const countries = await res.json();

    select.innerHTML = `<option value="all">All</option>`;

    countries.forEach(country => {
      const opt = document.createElement("option");
      opt.value = country.country_name;
      opt.textContent = country.country_name;
      select.appendChild(opt);
    });

    // Load initial charts (All countries)
    await loadCountryLineChartOut("all");
    await loadOperatorPieChartOut("all"); // ✅ Load initial operator pie chart

    // Event listener for filter changes
    select.addEventListener("change", async (e) => {
      const selected = e.target.value;

      const titleEl = document.getElementById("distributionTitleOut");
      if (titleEl) {
        titleEl.textContent =
          selected === "all"
            ? "Operator Distribution"
            : `Operator Distribution (${selected})`;
      }

      await loadCountryLineChartOut(selected);
      await loadOperatorPieChartOut(selected); // ✅ Update operator pie chart
    });
  } catch (err) {
    console.error("Failed to load countries:", err);
  }
}

/**
 * Load Country Line Chart (Dynamic)
 */
async function loadCountryLineChartOut(countryName = "all") {
  const ctx = document.getElementById("country-line-chart-out");
  if (!ctx) return;

  try {
    const payload =
      countryName === "all"
        ? {
            dimension: "global",
            aggregation: "history",
            filter: [{ key: "direction", value: "out" }]
          }
        : {
            dimension: "country",
            aggregation: "history",
            filter: [
              { key: "direction", value: "out" },
              { key: "country", value: countryName }
            ]
          };

    const res = await fetch(`${API_URL}/analytics`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(payload)
    });

    const data = await res.json();
    const labels = data.data.map(item => formatDateDDMMYYYY(item.date));
    const values = data.data.map(item => item.value);

    renderLineChart(
      ctx,
      countryName === "all" ? "All Countries" : countryName,
      labels,
      values,
      "#fd7e14"
    );
  } catch (err) {
    console.error("Failed to load country line chart:", err);
  }
}

/**
 * Load Operator Pie Chart (Dynamic) for Roam OUT
 */
async function loadOperatorPieChartOut(countryName = "all") {
  const ctx = document.getElementById("new-pie-chart-out");
  if (!ctx) return;

  try {
    const payload =
      countryName === "all"
        ? {
            dimension: "operator",
            aggregation: "top",
            filter: [{ key: "direction", value: "out" }]
          }
        : {
            dimension: "operator",
            aggregation: "top",
            filter: [
              { key: "direction", value: "out" },
              { key: "country", value: countryName }
            ]
          };

    const res = await fetch(`${API_URL}/analytics`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(payload)
    });

    const data = await res.json();
    
    // Handle case where no data is returned
    if (!data.data || data.data.length === 0) {
      console.warn("No operator data found for country:", countryName);
      if (ctx.chartInstance) {
        ctx.chartInstance.destroy();
        ctx.chartInstance = null;
      }
      return;
    }

    const labels = data.data.map(item => item.operator);
    const values = data.data.map(item => item.value);

    const colors = labels.map((_, i) => `hsl(${(i * 360) / labels.length}, 70%, 55%)`);

    // Destroy existing chart instance if any
    if (ctx.chartInstance) {
      ctx.chartInstance.destroy();
    }

    ctx.chartInstance = new Chart(ctx, {
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
        maintainAspectRatio: false,
        plugins: {
          legend: { 
            position: "bottom",
            labels: {
              boxWidth: 12,
              font: { size: 11 }
            }
          },
          tooltip: {
            callbacks: {
              label: (context) => `${context.label}: ${context.parsed.toLocaleString()}`
            }
          },
          title: {
            display: true,
            text: countryName === "all" ? "Top Operators" : `Top Operators - ${countryName}`,
            font: { size: 14 }
          }
        }
      }
    });
  } catch (err) {
    console.error("Failed to load operator pie chart:", err);
    // Destroy chart if there's an error
    if (ctx.chartInstance) {
      ctx.chartInstance.destroy();
      ctx.chartInstance = null;
    }
  }
}

/**
 * Render Line Chart (Reusable)
 */
function renderLineChart(ctx, label, labels, values, color) {
  const minValue = Math.min(...values);
  const maxValue = Math.max(...values);
  const padding = (maxValue - minValue) * 0.1;

  if (ctx.chartInstance) ctx.chartInstance.destroy();

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
        x: { title: { display: false } },
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