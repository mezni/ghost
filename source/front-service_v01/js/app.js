// ==============================
// RoamAdmin App - Unified Loader
// ==============================

$(document).ready(function () {
  loadLayout();
  initCustomFeatures();
  initPageLoader();
});

// ------------------------------
// Load Header, Sidebar, Footer
// ------------------------------
function loadLayout() {
  $("#header").load("pages/header.html");
  $("#sidebar").load("pages/sidebar.html", highlightActiveSidebarLink);
  $("#footer").load("pages/footer.html");

  // Default: load dashboard
  loadPage("pages/dashboard.html");
}

// ------------------------------
// Custom Features
// ------------------------------
function initCustomFeatures() {
  console.log("✅ RoamAdmin Initialized");

  $(document).on("click", "[data-widget='pushmenu']", () => {
    setTimeout(() => $(document).trigger("sidebarToggled"), 300);
  });

  $(document).on("sidebarToggled", () => console.log("Sidebar toggled"));
}

// ------------------------------
// Dynamic Page Loader
// ------------------------------
function initPageLoader() {
  $(document).on("click", ".nav-sidebar a", function (e) {
    const url = $(this).attr("href");
    if (url && url.startsWith("pages/")) {
      e.preventDefault();
      $(".nav-link").removeClass("active");
      $(this).addClass("active");

      loadPage(url);
    }
  });
}

// ------------------------------
// Load page content and trigger page JS
// ------------------------------
function loadPage(url) {
  $("#content-area").load(url, function (response, status) {
    if (status === "error") {
      $("#content-area").html(`
        <div class="alert alert-danger mt-3">
          <i class="fas fa-exclamation-triangle me-2"></i>
          Failed to load ${url}.
        </div>
      `);
    } else {
      console.log(`✅ Loaded ${url}`);
      updatePageHeader(url);
      triggerPageJS(url);
    }
  });
}

// ------------------------------
// Update Page Title + Breadcrumb
// ------------------------------
function updatePageHeader(url) {
  const pageName = url.split("/").pop().replace(".html", "");
  const titles = { dashboard: "Dashboard", roamin: "Roam IN", roamout: "Roam OUT" };
  const title = titles[pageName] || "Dashboard";

  $("#page-title").text(title);
  $("#breadcrumb-list").html(`
    <li class="breadcrumb-item"><a href="index.html">Home</a></li>
    <li class="breadcrumb-item active" aria-current="page">${title}</li>
  `);
}

// ------------------------------
// Trigger Page-specific JS
// ------------------------------
function triggerPageJS(url) {
  const pageName = url.split("/").pop().replace(".html", "");
  switch (pageName) {
    case "dashboard":
      if (window.dashboardInit) dashboardInit();
      if (window.updateDashboardMetrics) updateDashboardMetrics();
      break;
    case "roamin":
      if (window.roaminInit) roaminInit(); // fixed function name
      break;
    case "roamout":
      if (window.roamOutInit) roamOutInit();
      break;
  }
}

// ------------------------------
// Highlight active sidebar link
// ------------------------------
function highlightActiveSidebarLink() {
  const current = window.location.pathname.split("/").pop() || "index.html";
  $("#sidebar a.nav-link").removeClass("active");
  $("#sidebar a.nav-link").each(function () {
    const href = $(this).attr("href");
    if (href && current.includes(href)) $(this).addClass("active");
  });
}

// ------------------------------
// Simple notification utility
// ------------------------------
const AppUtils = {
  notify(title, message, type = "info") {
    const icons = {
      info: "bi bi-info-circle",
      success: "bi bi-check-circle",
      warning: "bi bi-exclamation-triangle",
      danger: "bi bi-x-circle",
    };
    const icon = icons[type] || icons.info;

    const $alert = $(`
      <div class="alert alert-${type} alert-dismissible fade show position-fixed top-0 end-0 m-3 shadow" style="z-index:1050;min-width:300px;">
        <i class="${icon} me-2"></i><strong>${title}</strong><br>${message}
        <button type="button" class="btn-close" data-bs-dismiss="alert"></button>
      </div>
    `);
    $("body").append($alert);
    setTimeout(() => $alert.alert("close"), 3500);
  },
};

window.AppUtils = AppUtils;

// ------------------------------
// Dashboard API Helpers
// ------------------------------
async function fetchMetric(direction) {
  try {
    const res = await fetch("http://localhost:3000/api/v1/metrics", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ metric: "metric", dimension: "global", direction, timeWindow: 1 })
    });
    const data = await res.json();
    if (data.status === "success" && data.data.length > 0) {
      return data.data[0];
    }
    return null;
  } catch (err) {
    console.error(`Error fetching ${direction} metric:`, err);
    return null;
  }
}

function formatDate(dateStr) {
  const date = new Date(dateStr);
  return `${String(date.getDate()).padStart(2,"0")}/${String(date.getMonth()+1).padStart(2,"0")}/${date.getFullYear()}`;
}

// ------------------------------
// Update dashboard info boxes dynamically
// ------------------------------
async function updateDashboardMetrics() {
  const roamIn = await fetchMetric("IN");
  const roamOut = await fetchMetric("OUT");

  if (roamIn) {
    const box = document.querySelector("#roamInBox");
    box.querySelector(".info-box-number").textContent = roamIn.value;
    box.querySelector(".info-box-content .fs-7").textContent = `Updated: ${formatDate(roamIn.date)}`;
  }

  if (roamOut) {
    const box = document.querySelector("#roamOutBox");
    box.querySelector(".info-box-number").textContent = roamOut.value;
    box.querySelector(".info-box-content .fs-7").textContent = `Updated: ${formatDate(roamOut.date)}`;
  }
}

// Expose to window for triggerPageJS
window.updateDashboardMetrics = updateDashboardMetrics;
