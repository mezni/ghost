const BASE_API = "http://localhost:3000/api/v1";

const AppUtils = {
  fetchData: async (endpoint) => {
    try {
      const response = await fetch(`${BASE_API}/${endpoint}`);
      if (!response.ok) throw new Error(`API error: ${response.status}`);
      return await response.json();
    } catch (err) {
      console.error(err);
      return null;
    }
  },

  notify: (title, message, type = "info") => {
    alert(`${title}: ${message}`);
  },

  // Load HTML template into a container
  loadHTML: async (url, containerId) => {
    try {
      const res = await fetch(url);
      const html = await res.text();
      document.getElementById(containerId).innerHTML = html;
    } catch (err) {
      console.error(`Failed to load ${url}:`, err);
    }
  }
};

// ==========================
// Initialize common layout
// ==========================
async function loadLayout() {
  await AppUtils.loadHTML("pages/header.html", "header");
  await AppUtils.loadHTML("pages/footer.html", "footer");
  await AppUtils.loadHTML("pages/sidebar.html", "sidebar");
}

// ==========================
// Load page content
// ==========================
async function loadPage(page) {
  const container = document.getElementById("content-area");
  try {
    const res = await fetch(`pages/${page}.html`);
    container.innerHTML = await res.text();

    // --------------------------
    // Update page title
    // --------------------------
    const titleEl = document.getElementById("page-title");
    const titleMap = {
      dashboard: "Dashboard",
      roamin: "Roam IN",
      roamout: "Roam OUT",
      countries: "Countries",
    };
    if (titleEl) titleEl.textContent = titleMap[page] || "Dashboard";

    // --------------------------
    // Update breadcrumb active item
    // --------------------------
    const breadcrumbEl = document.querySelector(".breadcrumb .active");
    if (breadcrumbEl) breadcrumbEl.textContent = titleMap[page] || "Dashboard";

    // --------------------------
    // Update sidebar active menu
    // --------------------------
    document.querySelectorAll("#sidebar a.nav-link").forEach(link => {
      link.classList.remove("active");
      const hrefPage = link.getAttribute("href").replace(".html", "").replace("#", "");
      if (hrefPage === page) link.classList.add("active");
    });

    // --------------------------
    // Call page-specific init function
    // --------------------------
    const initFuncName = `${page}Init`;
    if (typeof window[initFuncName] === "function") {
      window[initFuncName]();
    }

  } catch (err) {
    console.error(`Failed to load page ${page}:`, err);
  }
}

// ==========================
// Handle hash navigation
// ==========================
window.addEventListener("hashchange", () => {
  const page = location.hash.replace("#", "") || "dashboard";
  loadPage(page);
});

// ==========================
// Initial load
// ==========================
window.addEventListener("DOMContentLoaded", async () => {
  await loadLayout();

  // Get hash, default to 'dashboard'
  let initialPage = location.hash.replace("#", "") || "dashboard";

  // If you want to auto-load Roam OUT for testing:
  // initialPage = "roamout";

  await loadPage(initialPage);
});
