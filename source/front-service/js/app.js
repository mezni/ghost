const BASE_API = "http://localhost:3000/api/v1";

const AppUtils = {
  fetchData: async (endpoint) => {
    try {
      const response = await fetch(`${BASE_API}/${endpoint}`);
      if (!response.ok) throw new Error(`API error: ${response.status}`);
      return await response.json();
    } catch (err) {
      console.error("❌ API Fetch Error:", err);
      return null;
    }
  },

  notify: (title, message, type = "info") => {
    alert(`${title}: ${message}`);
  },

  loadHTML: async (url, containerId) => {
    try {
      const res = await fetch(url);
      const html = await res.text();
      document.getElementById(containerId).innerHTML = html;
    } catch (err) {
      console.error(`❌ Failed to load ${url}:`, err);
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
  initTreeview();
}

// ==========================
// Sidebar treeview toggle
// ==========================
function initTreeview() {
  document.querySelectorAll("#sidebar .nav-item > a").forEach(link => {
    const submenu = link.nextElementSibling;
    if (submenu && submenu.classList.contains("nav-treeview")) {
      link.addEventListener("click", (e) => {
        e.preventDefault();
        const isVisible = submenu.style.display === "block";

        document.querySelectorAll("#sidebar .nav-treeview").forEach(ul => {
          ul.style.display = "none";
        });

        submenu.style.display = isVisible ? "none" : "block";
      });
    }
  });
}

// ==========================
// Load page content
// ==========================
async function loadPage(page) {
  const container = document.getElementById("content-area");
  try {
    const res = await fetch(`pages/${page}.html`);
    const html = await res.text();
    container.innerHTML = html;

    // --- Update title and breadcrumb ---
    const titleEl = document.getElementById("page-title");
    const titleMap = {
      dashboard: "Dashboard",
      roamin: "Roam IN",
      roamout: "Roam OUT",
      countries: "Countries",
      operators: "Operators",
      networks: "Networks",
      sorplans: "SOR Plans",
    };
    if (titleEl) titleEl.textContent = titleMap[page] || "Dashboard";

    const breadcrumbEl = document.querySelector(".breadcrumb .active");
    if (breadcrumbEl) breadcrumbEl.textContent = titleMap[page] || "Dashboard";

    // --- Sidebar highlight ---
    document.querySelectorAll("#sidebar a.nav-link").forEach(link => {
      link.classList.remove("active");
      const hrefPage = link.getAttribute("href")?.replace(".html", "").replace("#", "");
      if (hrefPage === page) link.classList.add("active");
    });

    // --- Page-specific init ---
    const initFuncName = `${page}Init`;
    console.log(`🔍 Looking for init function: ${initFuncName}`);

    // Wait a tick to ensure other scripts (like countries.js) are loaded
    await new Promise(r => setTimeout(r, 100));

    if (typeof window[initFuncName] === "function") {
      console.log(`🚀 Calling ${initFuncName}()`);
      await window[initFuncName]();
    } else {
      console.warn(`⚠️ No function found for ${initFuncName}`);
    }

  } catch (err) {
    console.error(`❌ Failed to load page ${page}:`, err);
  }
}

// ==========================
// Handle navigation
// ==========================
function handleNavigation() {
  const page = location.hash.replace("#", "") || "dashboard";
  console.log(`📄 handleNavigation -> ${page}`);
  loadPage(page);
}

window.addEventListener("hashchange", handleNavigation);

// ==========================
// Initial load
// ==========================
window.addEventListener("DOMContentLoaded", async () => {
  await loadLayout();
  handleNavigation();
});
