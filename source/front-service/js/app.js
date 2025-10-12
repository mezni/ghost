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
// Load page content (Enhanced)
// ==========================
async function loadPage(page) {
  const container = document.getElementById("content-area");
  try {
    const res = await fetch(`pages/${page}.html`);
    const html = await res.text();
    if (!container) return;

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

    // Load page-specific JS if exists
    const scriptPath = `js/${page}.js`;
    try {
      // Remove existing script if any
      const existingScript = document.querySelector(`script[src="${scriptPath}"]`);
      if (existingScript) existingScript.remove();
      
      // Load new script
      const script = document.createElement('script');
      script.src = scriptPath;
      script.onload = () => {
        console.log(`✅ ${scriptPath} loaded successfully`);
        // Call page-specific init after script loads
        const initFuncName = `${page}Init`;
        if (typeof window[initFuncName] === 'function') {
          console.log(`🚀 Calling ${initFuncName}()`);
          window[initFuncName]();
        } else {
          console.warn(`⚠️ ${initFuncName} not found after script load`);
        }
      };
      script.onerror = () => {
        console.warn(`⚠️ ${scriptPath} not found, skipping`);
        // Try calling init anyway in case function exists from previous load
        const initFuncName = `${page}Init`;
        if (typeof window[initFuncName] === 'function') {
          console.log(`🚀 Calling ${initFuncName}() from cache`);
          window[initFuncName]();
        }
      };
      document.head.appendChild(script);
    } catch (err) {
      console.error(`❌ Error loading ${scriptPath}:`, err);
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
