// ==========================
// Dashboard Page Script
// ==========================

// ✅ Base API URL
const API_URL = "http://localhost:3000/api/v1/metrics";

// ==========================
// Init Dashboard
// ==========================
function dashboardInit() {
  console.log("💠 Dashboard page initialized");

  // Load API data
  updateMetric("IN", "#roamInBox");
  updateMetric("OUT", "#roamOutBox");
  updateNotifications("#notificationsBox");
  updateDirectChat(".direct-chat"); // Direct Chat messages

  // ==========================
  // Chart Example (Static)
  // ==========================
  const ctx = document.getElementById("overviewChart")?.getContext("2d");
  if (!ctx) return;

  new Chart(ctx, {
    type: "line",
    data: {
      labels: ["Jan", "Feb", "Mar", "Apr", "May", "Jun"],
      datasets: [
        {
          label: "Roam IN",
          data: [120, 140, 150, 130, 160, 170],
          borderColor: "#007bff",
          fill: false
        },
        {
          label: "Roam OUT",
          data: [200, 190, 180, 210, 230, 250],
          borderColor: "#28a745",
          fill: false
        }
      ]
    },
    options: {
      responsive: true,
      maintainAspectRatio: false,
      plugins: { legend: { position: "bottom" } },
      scales: {
        y: { beginAtZero: false }
      }
    }
  });
}

// ==========================
// Fetch metric from API and update info box (Roam IN / Roam OUT)
// ==========================
async function updateMetric(direction, selector) {
  const box = document.querySelector(selector);
  if (!box) return;

  try {
    const response = await fetch(API_URL, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        dimension: "global",
        aggregation: "latest",
        filter: [{ key: "direction", value: direction }]
      })
    });

    const result = await response.json();
    if (result.status === "success" && result.data.length > 0) {
      const { value, date } = result.data[0];
      box.querySelector(".info-box-number").textContent = value;
      box.querySelector(".info-box-text.text-muted.fs-7").textContent = `Updated: ${formatDate(date)}`;
    } else {
      box.querySelector(".info-box-number").textContent = "0";
      box.querySelector(".info-box-text.text-muted.fs-7").textContent = "Updated: --/--/----";
    }
  } catch (error) {
    console.error(`❌ Failed to fetch ${direction} metric:`, error);
  }
}

// ==========================
// Fetch notifications count and update box
// ==========================
async function updateNotifications(selector) {
  const box = document.querySelector(selector);
  if (!box) return;

  try {
    const response = await fetch(API_URL, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        dimension: "notification",
        aggregation: "summary"
      })
    });

    const result = await response.json();
    if (result.status === "success" && result.data.length > 0) {
      const { value, date } = result.data[0];
      box.querySelector(".info-box-number").textContent = value;
      box.querySelector(".info-box-text.text-muted.fs-7").textContent = `Updated: ${formatDate(date)}`;
    } else {
      box.querySelector(".info-box-number").textContent = "0";
      box.querySelector(".info-box-text.text-muted.fs-7").textContent = "Updated: --/--/----";
    }
  } catch (error) {
    console.error("❌ Failed to fetch notifications:", error);
  }
}

// ==========================
// Fetch notifications detail and update Direct Chat messages
// ==========================
async function updateDirectChat(selector) {
  const container = document.querySelector(selector);
  if (!container) return;

  try {
    const response = await fetch(API_URL, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        dimension: "notification",
        aggregation: "detail"
      })
    });

    const result = await response.json();
    if (result.status === "success" && result.data.length > 0) {
      const messagesContainer = container.querySelector(".direct-chat-messages");
      messagesContainer.innerHTML = ""; // Clear previous messages

      result.data.forEach(msg => {
        const msgHtml = document.createElement("div");
        msgHtml.classList.add("direct-chat-msg");

        msgHtml.innerHTML = `
          <div class="direct-chat-infos clearfix">
            <span class="direct-chat-name float-start">SYSTEM</span>
            <span class="direct-chat-timestamp float-end">${formatDate(msg.date)}</span>
          </div>
          <img class="direct-chat-img" src="https://ui-avatars.com/api/?name=SY" alt="user image">
          <div class="direct-chat-text">
            ${msg.value}
          </div>
        `;
        messagesContainer.appendChild(msgHtml);
      });

      // Scroll to bottom
      messagesContainer.scrollTop = messagesContainer.scrollHeight;
    }
  } catch (error) {
    console.error("❌ Failed to fetch Direct Chat messages:", error);
  }
}

// ==========================
// Format date from YYYY-MM-DD to DD/MM/YYYY
// ==========================
function formatDate(dateStr) {
  if (!dateStr) return "--/--/----";
  const parts = dateStr.split("-");
  if (parts.length !== 3) return "--/--/----";
  const [year, month, day] = parts;
  return `${day}/${month}/${year}`;
}

// ==========================
// Expose globally
// ==========================
window.dashboardInit = dashboardInit;
