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
    // Simple alert placeholder
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

// Initialize common layout (header, footer, sidebar)
async function loadLayout() {
  await AppUtils.loadHTML("pages/header.html", "header");
  await AppUtils.loadHTML("pages/footer.html", "footer");
  await AppUtils.loadHTML("pages/sidebar.html", "sidebar");
}

// Load page content into #content-area and call its init function
async function loadPage(page) {
  const container = document.getElementById("content-area");
  try {
    const res = await fetch(`pages/${page}.html`);
    container.innerHTML = await res.text();

    // Call page-specific init if exists
    const initFuncName = `${page}Init`;
    if (typeof window[initFuncName] === "function") {
      window[initFuncName]();
    }
  } catch (err) {
    console.error(`Failed to load page ${page}:`, err);
  }
}

// Detect hash change (URL fragment) to load pages dynamically
window.addEventListener("hashchange", () => {
  const page = location.hash.replace("#", "") || "dashboard";
  loadPage(page);
});

// Initial load
window.addEventListener("DOMContentLoaded", async () => {
  await loadLayout();

  const initialPage = location.hash.replace("#", "") || "dashboard";
  loadPage(initialPage);
});
