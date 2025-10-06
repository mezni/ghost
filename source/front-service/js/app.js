/**
 * RoamAdmin - Main Application Controller
 * Handles page navigation, template loading, and API communication
 */

class App {
    constructor() {
        this.currentPage = 'dashboard';
        this.apiBaseUrl = 'http://localhost:3000/api'; // Update with your API URL
        this.isInitialized = false;
        this.init();
    }

    async init() {
        try {
            // Load all templates first
            await this.loadTemplates();
            
            // Setup event listeners
            this.setupEventListeners();
            
            // Initialize AdminLTE components
            this.initAdminLTE();
            
            // Load the initial page
            await this.loadPage('dashboard');
            
            this.isInitialized = true;
            console.log('RoamAdmin application initialized successfully');
            
        } catch (error) {
            console.error('Failed to initialize application:', error);
            this.showError('Failed to initialize application. Please refresh the page.');
        }
    }

    /**
     * Load HTML templates for header, sidebar, and footer
     */
    async loadTemplates() {
        try {
            const templates = await Promise.all([
                this.fetchTemplate('pages/header.html'),
                this.fetchTemplate('pages/sidebar.html'),
                this.fetchTemplate('pages/footer.html')
            ]);
            
            document.getElementById('header-placeholder').innerHTML = templates[0];
            document.getElementById('sidebar-placeholder').innerHTML = templates[1];
            document.getElementById('footer-placeholder').innerHTML = templates[2];
            
        } catch (error) {
            console.error('Error loading templates:', error);
            throw new Error('Failed to load page templates');
        }
    }

    /**
     * Fetch template from server
     */
    async fetchTemplate(url) {
        const response = await fetch(url);
        if (!response.ok) {
            throw new Error(`Failed to load template: ${url}`);
        }
        return await response.text();
    }

    /**
     * Initialize AdminLTE components
     */
    initAdminLTE() {
        // Initialize push menu
        if (typeof $.fn.pushMenu !== 'undefined') {
            $('[data-widget="pushmenu"]').pushMenu();
        }
        
        // Initialize treeview
        if (typeof $.fn.treeview !== 'undefined') {
            $('[data-widget="treeview"]').treeview();
        }
        
        // Initialize dropdowns
        if (typeof $.fn.dropdown !== 'undefined') {
            $('.dropdown-toggle').dropdown();
        }

        // Initialize tooltips if available
        if (typeof $.fn.tooltip !== 'undefined') {
            $('[data-toggle="tooltip"]').tooltip();
        }
    }

    /**
     * Setup global event listeners
     */
    setupEventListeners() {
        // Navigation event listeners
        document.addEventListener('click', (e) => {
            const target = e.target.closest('[data-page]');
            if (target) {
                e.preventDefault();
                const page = target.getAttribute('data-page');
                this.loadPage(page);
            }
        });

        // Breadcrumb navigation
        document.addEventListener('click', (e) => {
            const breadcrumbLink = e.target.closest('.breadcrumb a[data-page]');
            if (breadcrumbLink) {
                e.preventDefault();
                const page = breadcrumbLink.getAttribute('data-page');
                this.loadPage(page);
            }
        });

        // Handle browser back/forward buttons
        window.addEventListener('popstate', (e) => {
            if (e.state && e.state.page) {
                this.loadPage(e.state.page, false);
            }
        });

        // Handle initial hash URL
        if (window.location.hash) {
            const page = window.location.hash.substring(1);
            if (page && page !== 'dashboard') {
                setTimeout(() => this.loadPage(page, false), 100);
            }
        }
    }

    /**
     * Load and display a page
     */
/**
 * Load and display a page
 */
async loadPage(page, pushState = true) {
    try {
        if (this.currentPage === page && this.isInitialized) {
            return; // Already on this page
        }

        console.log(`Loading page: ${page}`); // DEBUG
        this.showLoading();
        this.currentPage = page;

        // Update browser history
        if (pushState) {
            window.history.pushState({ page }, '', `#${page}`);
        }

        const pageConfig = this.getPageConfig(page);
        console.log('Page config:', pageConfig); // DEBUG
        
        const pageContent = await this.fetchTemplate(pageConfig.template);
        console.log('Page content loaded'); // DEBUG
        
        // Update page content
        document.getElementById('content-area').innerHTML = pageContent;
        document.getElementById('page-title').textContent = pageConfig.title;
        document.getElementById('breadcrumb-active').textContent = pageConfig.title;

        // Update active menu state
        this.updateActiveMenu();

        // Load page-specific JavaScript
        console.log('Loading page script:', pageConfig.script); // DEBUG
        await this.loadPageScript(pageConfig.script);

        // Add fade-in animation
        document.getElementById('content-area').classList.add('fade-in');
        
        this.hideLoading();
        console.log(`Page ${page} loaded successfully`); // DEBUG

        // Scroll to top
        window.scrollTo(0, 0);

        // Update document title
        document.title = `RoamAdmin - ${pageConfig.title}`;

    } catch (error) {
        console.error(`Error loading page ${page}:`, error);
        this.hideLoading();
        this.showPageError(page, error);
    }
}
    /**
     * Get configuration for a page
     */
    getPageConfig(page) {
        const config = {
            'dashboard': {
                template: 'pages/dashboard.html',
                script: 'js/dashboard.js',
                title: 'Dashboard'
            },
            'roamin': {
                template: 'pages/roamin.html',
                script: 'js/roamin.js',
                title: 'Roam IN Analytics'
            },
            'roamout': {
                template: 'pages/roamout.html',
                script: 'js/roamout.js',
                title: 'Roam OUT Analytics'
            },
            'countries': {
                template: 'pages/countries.html',
                script: 'js/countries.js',
                title: 'Countries Management'
            },
            'operators': {
                template: 'pages/operators.html',
                script: 'js/operators.js',
                title: 'Operators Management'
            },
            'plans': {
                template: 'pages/plans.html',
                script: 'js/plans.js',
                title: 'SOR Plans Management'
            }
        };
        
        return config[page] || {
            template: 'pages/dashboard.html',
            script: 'js/dashboard.js',
            title: 'Dashboard'
        };
    }

    /**
     * Load page-specific JavaScript
     */
    async loadPageScript(scriptPath) {
        if (!scriptPath) return;

        try {
            // Remove previously loaded script if exists
            const existingScript = document.querySelector(`script[data-page-script]`);
            if (existingScript) {
                existingScript.remove();
            }

            // Create new script element
            return new Promise((resolve, reject) => {
                const script = document.createElement('script');
                script.src = scriptPath;
                script.setAttribute('data-page-script', this.currentPage);
                script.onload = () => {
                    console.log(`Loaded script: ${scriptPath}`);
                    resolve();
                };
                script.onerror = () => {
                    console.error(`Failed to load script: ${scriptPath}`);
                    reject(new Error(`Failed to load script: ${scriptPath}`));
                };
                
                document.body.appendChild(script);
            });
            
        } catch (error) {
            console.error('Error loading page script:', error);
            throw error;
        }
    }

    /**
     * Update active state in sidebar menu
     */
    updateActiveMenu() {
        // Remove active class from all menu items
        document.querySelectorAll('.nav-link').forEach(link => {
            link.classList.remove('active');
        });

        // Add active class to current page
        const currentLink = document.querySelector(`[data-page="${this.currentPage}"]`);
        if (currentLink) {
            currentLink.classList.add('active');
            
            // Also activate parent menu items if any
            let parent = currentLink.closest('.nav-treeview');
            if (parent) {
                const parentLink = parent.previousElementSibling;
                if (parentLink && parentLink.classList.contains('nav-link')) {
                    parentLink.classList.add('active');
                }
            }
        }

        // Close other open menus for better UX
        document.querySelectorAll('.nav-treeview').forEach(menu => {
            if (!menu.contains(currentLink)) {
                const parentItem = menu.closest('.nav-item');
                if (parentItem) {
                    parentItem.classList.remove('menu-open');
                }
            }
        });
    }

    /**
     * Show loading indicator - MODIFIED: Disabled popup
     */
    showLoading() {
        // Disabled loading modal
        // const loadingModal = document.getElementById('loadingModal');
        // if (loadingModal) {
        //     $(loadingModal).modal('show');
        // }
        
        // Optional: Add a subtle loading indicator instead
        const contentArea = document.getElementById('content-area');
        if (contentArea) {
            contentArea.style.opacity = '0.7';
            contentArea.style.pointerEvents = 'none';
        }
    }

    /**
     * Hide loading indicator - MODIFIED: Disabled popup
     */
    hideLoading() {
        // Disabled loading modal
        // const loadingModal = document.getElementById('loadingModal');
        // if (loadingModal) {
        //     $(loadingModal).modal('hide');
        // }
        
        // Restore content area if using subtle loading
        const contentArea = document.getElementById('content-area');
        if (contentArea) {
            contentArea.style.opacity = '1';
            contentArea.style.pointerEvents = 'auto';
            contentArea.classList.remove('fade-in');
        }
    }

    /**
     * Display error message for page load failure
     */
    showPageError(page, error) {
        document.getElementById('content-area').innerHTML = `
            <div class="alert alert-danger">
                <h4><i class="fas fa-exclamation-triangle"></i> Error Loading Page</h4>
                <p>Failed to load ${page}: ${error.message}</p>
                <div class="mt-3">
                    <button class="btn btn-primary" onclick="app.loadPage('dashboard')">
                        <i class="fas fa-home mr-2"></i>Return to Dashboard
                    </button>
                    <button class="btn btn-secondary ml-2" onclick="location.reload()">
                        <i class="fas fa-redo mr-2"></i>Reload Page
                    </button>
                </div>
            </div>
        `;
    }

    /**
     * Make API calls - MODIFIED: Remove loading for API calls
     */
    async apiCall(endpoint, options = {}) {
        const defaultOptions = {
            headers: {
                'Content-Type': 'application/json',
                ...options.headers
            },
            credentials: 'include'
        };

        const config = {
            ...defaultOptions,
            ...options
        };

        if (config.body && typeof config.body === 'object') {
            config.body = JSON.stringify(config.body);
        }

        try {
            // REMOVED: this.showLoading();
            const response = await fetch(`${this.apiBaseUrl}${endpoint}`, config);

            if (!response.ok) {
                const errorText = await response.text();
                throw new Error(`HTTP error! status: ${response.status}, message: ${errorText}`);
            }

            const contentType = response.headers.get('content-type');
            if (contentType && contentType.includes('application/json')) {
                return await response.json();
            } else {
                return await response.text();
            }

        } catch (error) {
            console.error('API call failed:', error);
            this.showError(`API Error: ${error.message}`);
            throw error;
        } finally {
            // REMOVED: this.hideLoading();
        }
    }

    /**
     * Show success message
     */
    showSuccess(message, title = 'Success') {
        this.showNotification(message, title, 'success');
    }

    /**
     * Show error message
     */
    showError(message, title = 'Error') {
        this.showNotification(message, title, 'danger');
    }

    /**
     * Show warning message
     */
    showWarning(message, title = 'Warning') {
        this.showNotification(message, title, 'warning');
    }

    /**
     * Show info message
     */
    showInfo(message, title = 'Info') {
        this.showNotification(message, title, 'info');
    }

    /**
     * Display notification
     */
    showNotification(message, title, type) {
        // Create notification element
        const notification = document.createElement('div');
        notification.className = `alert alert-${type} alert-dismissible fade show`;
        notification.style.cssText = `
            position: fixed;
            top: 80px;
            right: 20px;
            z-index: 9999;
            min-width: 300px;
            box-shadow: 0 4px 12px rgba(0,0,0,0.15);
        `;
        notification.innerHTML = `
            <button type="button" class="close" data-dismiss="alert">
                <span>&times;</span>
            </button>
            <strong>${title}</strong><br>
            ${message}
        `;

        // Add to page
        document.body.appendChild(notification);

        // Auto remove after 5 seconds
        setTimeout(() => {
            if (notification.parentNode) {
                $(notification).alert('close');
            }
        }, 5000);
    }

    /**
     * Format number with commas
     */
    formatNumber(num) {
        return new Intl.NumberFormat().format(num);
    }

    /**
     * Format date
     */
    formatDate(date, includeTime = false) {
        if (!date) return 'N/A';
        
        const dateObj = new Date(date);
        if (isNaN(dateObj.getTime())) return 'Invalid Date';

        const options = {
            year: 'numeric',
            month: 'short',
            day: 'numeric'
        };

        if (includeTime) {
            options.hour = '2-digit';
            options.minute = '2-digit';
        }

        return dateObj.toLocaleDateString(undefined, options);
    }

    /**
     * Format date and time
     */
    formatDateTime(date) {
        return this.formatDate(date, true);
    }

    /**
     * Debounce function for search inputs
     */
    debounce(func, wait) {
        let timeout;
        return function executedFunction(...args) {
            const later = () => {
                clearTimeout(timeout);
                func(...args);
            };
            clearTimeout(timeout);
            timeout = setTimeout(later, wait);
        };
    }

    /**
     * Validate email format
     */
    validateEmail(email) {
        const re = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
        return re.test(email);
    }

    /**
     * Generate unique ID
     */
    generateId() {
        return Date.now().toString(36) + Math.random().toString(36).substr(2);
    }

    /**
     * Download data as file
     */
    downloadFile(data, filename, type = 'text/plain') {
        const file = new Blob([data], { type: type });
        const a = document.createElement('a');
        const url = URL.createObjectURL(file);
        a.href = url;
        a.download = filename;
        document.body.appendChild(a);
        a.click();
        setTimeout(() => {
            document.body.removeChild(a);
            window.URL.revokeObjectURL(url);
        }, 0);
    }
}

// Initialize application when DOM is loaded
document.addEventListener('DOMContentLoaded', () => {
    window.app = new App();
});

// Make app globally available for HTML onclick handlers
window.App = App;

// Export for use in other modules
if (typeof module !== 'undefined' && module.exports) {
    module.exports = App;
}