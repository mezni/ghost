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
      const container = document.getElementById(containerId);
      if (container) container.innerHTML = html;
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
        document.querySelectorAll("#sidebar .nav-treeview").forEach(ul => ul.style.display = "none");
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
  if (!container) return;

  try {
    const res = await fetch(`pages/${page}.html`);
    const html = await res.text();
    container.innerHTML = html;

    // Wait for browser to parse HTML
    await new Promise(resolve => requestAnimationFrame(resolve));

    // Update title and breadcrumb
    const titleEl = document.getElementById("page-title");
    const titleMap = {
      dashboard: "Dashboard",
      roamin: "Roam IN",
      roamout: "Roam OUT", 
      countries: "Countries",
      operators: "Operators",
      networks: "Networks",
      sorplans: "SOR Plans",
      prefixes: "Prefixes",
    };
    if (titleEl) titleEl.textContent = titleMap[page] || "Dashboard";

    const breadcrumbEl = document.querySelector(".breadcrumb .active");
    if (breadcrumbEl) breadcrumbEl.textContent = titleMap[page] || "Dashboard";

    // Sidebar highlight
    document.querySelectorAll("#sidebar a.nav-link").forEach(link => {
      link.classList.remove("active");
      const hrefPage = link.getAttribute("href")?.replace(".html", "").replace("#", "");
      if (hrefPage === page) link.classList.add("active");
    });

    // Load page-specific JS dynamically
    const scriptPath = `js/${page}.js`;

    // Remove previous page script
    document.querySelectorAll(`script[data-page-script]`).forEach(s => s.remove());

    const script = document.createElement('script');
    script.src = scriptPath;
    script.setAttribute("data-page-script", page);
    script.onload = () => {
      console.log(`✅ Loaded ${scriptPath}`);
      const initFunc = `${page}Init`;
      if (typeof window[initFunc] === "function") {
        console.log(`🚀 Calling ${initFunc}()`);
        window[initFunc]();
      }
    };
    script.onerror = () => {
      console.warn(`⚠️ Could not load ${scriptPath}, trying cached init function`);
      const initFunc = `${page}Init`;
      if (typeof window[initFunc] === "function") {
        console.log(`🚀 Calling ${initFunc}() from cache`);
        window[initFunc]();
      }
    };
    document.head.appendChild(script);

  } catch (err) {
    console.error(`❌ Failed to load page ${page}:`, err);
    container.innerHTML = `<div class="text-danger">Failed to load page: ${err.message}</div>`;
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
