// ==========================
// Dashboard Page Script
// ==========================

// --------------------------
// Helpers
// --------------------------

// Format date from YYYY-MM-DD → DD/MM/YYYY
function formatDate(dateStr) {
  const [year, month, day] = dateStr.split("-");
  return `${day}/${month}/${year}`;
}

// Animate number update
function animateNumber(el) {
  if (!el) return;
  el.classList.add("updated");
  setTimeout(() => el.classList.remove("updated"), 500);
}

// Safe fetch and update DOM
async function fetchMetric(body, valueId, dateId) {
  try {
    const res = await fetch(`${BASE_API}/metrics`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(body)
    });
    const data = await res.json();

    const elValue = document.getElementById(valueId);
    const elDate = document.getElementById(dateId);

    if (!elValue || !elDate) return;

    if (data?.status === "success" && data?.data?.length) {
      const latest = data.data[0];
      elValue.innerText = latest.value;
      elDate.innerText = `Updated at: ${formatDate(latest.date)}`;
      animateNumber(elValue);
    } else {
      elValue.innerText = "0";
      elDate.innerText = "Updated at: N/A";
    }
  } catch (err) {
    console.error(`Error fetching ${valueId}:`, err);
    const elValue = document.getElementById(valueId);
    const elDate = document.getElementById(dateId);
    if (elValue) elValue.innerText = "0";
    if (elDate) elDate.innerText = "Updated at: Error";
  }
}

// --------------------------
// Global Trend Chart
// --------------------------
async function loadGlobalTrendChart() {
  const chartEl = document.getElementById("global-chart");
  if (!chartEl) return;

  try {
    // Fetch Roam IN history
    const resIn = await fetch(`${BASE_API}/metrics`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        dimension: "global",
        aggregation: "history",
        filter: [{ key: "direction", value: "in" }]
      })
    });
    const dataIn = await resIn.json();

    // Fetch Roam OUT history
    const resOut = await fetch(`${BASE_API}/metrics`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        dimension: "global",
        aggregation: "history",
        filter: [{ key: "direction", value: "out" }]
      })
    });
    const dataOut = await resOut.json();

    // Prepare chart data
    const labels = [];
    const inValues = [];
    const outValues = [];

    if (dataIn?.status === "success" && dataOut?.status === "success") {
      dataIn.data.forEach((item, idx) => {
        labels.push(formatDate(item.date));
        inValues.push(item.value);
        outValues.push(dataOut.data[idx]?.value || 0);
      });
    }

    new Chart(chartEl.getContext("2d"), {
      type: "line",
      data: {
        labels,
        datasets: [
          {
            label: "Roam IN",
            data: inValues,
            borderColor: "blue",
            backgroundColor: "rgba(0,0,255,0.1)",
            tension: 0.2,
          },
          {
            label: "Roam OUT",
            data: outValues,
            borderColor: "green",
            backgroundColor: "rgba(0,255,0,0.1)",
            tension: 0.2,
          }
        ]
      },
      options: {
        responsive: true,
        maintainAspectRatio: false,
        plugins: {
          legend: { position: "top" }
        },
        scales: {
          x: {
            title: { display: true, text: "Date" }
          },
          y: {
            title: { display: true, text: "Value" },
            beginAtZero: true
          }
        }
      }
    });
  } catch (err) {
    console.error("Error loading Global Trend chart:", err);
  }
}

// --------------------------
// Dashboard Initialization
// --------------------------
async function dashboardInit() {
  console.log("💠 Dashboard initialized");

  // Metrics for info boxes
  const metrics = [
    { body: { dimension: "global", aggregation: "latest", filter: [{ key: "direction", value: "in" }] }, valueId: "roamInValue", dateId: "roamInDate" },
    { body: { dimension: "global", aggregation: "latest", filter: [{ key: "direction", value: "out" }] }, valueId: "roamOutValue", dateId: "roamOutDate" },
    { body: { dimension: "global", aggregation: "latest", filter: [{ key: "type", value: "alerts" }] }, valueId: "alertsValue", dateId: "alertsDate" },
    { body: { dimension: "notification", aggregation: "summary" }, valueId: "notificationsValue", dateId: "notificationsDate" }
  ];

  for (const m of metrics) {
    await fetchMetric(m.body, m.valueId, m.dateId);
  }

  // Load Global Trend chart
  await loadGlobalTrendChart();
}

// Note: app.js will call dashboardInit(), do not call it here
