/**
 * Roam IN Page Script
 * Handles line chart (history) and pie chart (by country)
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

    const minValue = Math.min(...values);
    const maxValue = Math.max(...values);
    const padding = (maxValue - minValue) * 0.1;

    new Chart(ctx, {
      type: "line",
      data: {
        labels,
        datasets: [
          {
            label: "Roam IN",
            data: values,
            borderColor: "#007bff",
            backgroundColor: "rgba(0,123,255,0.2)",
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
          x: { title: { display: true, text: "Date" } },
          y: { 
            title: { display: true, text: "Value" },
            beginAtZero: false,
            min: minValue - padding,
            max: maxValue + padding
          }
        }
      }
    });

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
