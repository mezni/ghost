// ==========================
// Dashboard Page Script
// ==========================
function dashboardInit() {
  console.log("💠 Dashboard page initialized");

  updateMetric("IN", "#roamInBox");
  updateMetric("OUT", "#roamOutBox");

  // Initialize overview chart
  const ctx = document.getElementById("overviewChart")?.getContext("2d");
  if (!ctx) return;

  new Chart(ctx, {
    type: "line",
    data: {
      labels: ["Jan","Feb","Mar","Apr","May","Jun"],
      datasets: [
        { label: "Roam IN", data: [120,140,150,130,160,170], borderColor:"#007bff", fill:false },
        { label: "Roam OUT", data: [200,190,180,210,230,250], borderColor:"#28a745", fill:false }
      ]
    },
  });
}

// Fetch metric from API and update info box
async function updateMetric(direction, selector) {
  const box = document.querySelector(selector);
  if (!box) return;

  try {
    const response = await fetch("http://localhost:3000/api/v1/metrics", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        metric: "metric",
        dimension: "global",
        direction: direction
      })
    });

    const result = await response.json();
    if (result.status === "success" && result.data.length > 0) {
      const { value, date } = result.data[0];
      box.querySelector(".info-box-number").textContent = value;
      box.querySelector(".info-box-text.fs-7").textContent = `Updated: ${formatDate(date)}`;
    }
  } catch (error) {
    console.error(`Failed to fetch ${direction} metric:`, error);
  }
}

// Format date from YYYY-MM-DD to DD/MM/YYYY
function formatDate(dateStr) {
  const date = new Date(dateStr);
  const day = String(date.getDate()).padStart(2,"0");
  const month = String(date.getMonth()+1).padStart(2,"0");
  const year = date.getFullYear();
  return `${day}/${month}/${year}`;
}
