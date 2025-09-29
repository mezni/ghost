// Authentication Management System
class AuthManager {
    constructor() {
        this.currentUser = null;
        this.token = null;
        this.tokenExpiry = null;
        this.init();
    }

    init() {
        console.log('Auth Manager Initialized');
        this.loadStoredAuth();
        this.setupInterceptors();
        this.checkAutoLogout();
    }

    // Load stored authentication data from localStorage
    loadStoredAuth() {
        this.token = localStorage.getItem('authToken');
        this.tokenExpiry = localStorage.getItem('tokenExpiry');
        const userData = localStorage.getItem('userData');
        
        if (userData) {
            this.currentUser = JSON.parse(userData);
        }

        // Validate token expiry
        if (this.token && this.tokenExpiry) {
            const now = new Date().getTime();
            if (now > parseInt(this.tokenExpiry)) {
                this.logout();
                return;
            }
        }

        this.updateUI();
    }

    // Setup axios interceptors for API calls (if using axios)
    setupInterceptors() {
        // This would be used if you're using axios for API calls
        if (typeof axios !== 'undefined') {
            axios.interceptors.request.use(
                (config) => {
                    if (this.token) {
                        config.headers.Authorization = `Bearer ${this.token}`;
                    }
                    return config;
                },
                (error) => {
                    return Promise.reject(error);
                }
            );

            axios.interceptors.response.use(
                (response) => response,
                (error) => {
                    if (error.response && error.response.status === 401) {
                        this.handleUnauthorized();
                    }
                    return Promise.reject(error);
                }
            );
        }
    }

    // Login method
    async login(credentials) {
        try {
            this.showLoading('Logging in...');
            
            // For demo purposes - replace with actual API call
            const response = await this.mockLogin(credentials);
            
            if (response.success) {
                this.token = response.token;
                this.currentUser = response.user;
                this.tokenExpiry = response.expiry;
                
                this.storeAuthData();
                this.updateUI();
                this.showToast('Login successful!', 'success');
                
                return { success: true, user: this.currentUser };
            } else {
                throw new Error(response.message || 'Login failed');
            }
        } catch (error) {
            console.error('Login error:', error);
            this.showToast(error.message || 'Login failed. Please try again.', 'error');
            return { success: false, message: error.message };
        } finally {
            this.hideLoading();
        }
    }

    // Mock login function - replace with actual API call
    async mockLogin(credentials) {
        return new Promise((resolve, reject) => {
            setTimeout(() => {
                const { username, password } = credentials;
                
                // Demo credentials
                const validUsers = [
                    { username: 'admin', password: 'admin', role: 'administrator', name: 'System Administrator' },
                    { username: 'user', password: 'user123', role: 'user', name: 'Regular User' }
                ];

                const user = validUsers.find(u => u.username === username && u.password === password);
                
                if (user) {
                    const token = this.generateToken(user);
                    const expiry = new Date().getTime() + (24 * 60 * 60 * 1000); // 24 hours
                    
                    resolve({
                        success: true,
                        token: token,
                        user: {
                            id: 1,
                            username: user.username,
                            name: user.name,
                            role: user.role,
                            email: `${user.username}@example.com`
                        },
                        expiry: expiry
                    });
                } else {
                    resolve({
                        success: false,
                        message: 'Invalid username or password'
                    });
                }
            }, 1500); // Simulate API delay
        });
    }

    // Generate mock JWT token
    generateToken(user) {
        const header = btoa(JSON.stringify({ alg: 'HS256', typ: 'JWT' }));
        const payload = btoa(JSON.stringify({
            sub: user.username,
            name: user.name,
            role: user.role,
            iat: Math.floor(Date.now() / 1000),
            exp: Math.floor(Date.now() / 1000) + (24 * 60 * 60) // 24 hours
        }));
        const signature = 'mock_signature';
        
        return `${header}.${payload}.${signature}`;
    }

    // Register method
    async register(userData) {
        try {
            this.showLoading('Creating account...');
            
            // For demo purposes - replace with actual API call
            const response = await this.mockRegister(userData);
            
            if (response.success) {
                this.showToast('Account created successfully! Please login.', 'success');
                return { success: true };
            } else {
                throw new Error(response.message || 'Registration failed');
            }
        } catch (error) {
            console.error('Registration error:', error);
            this.showToast(error.message || 'Registration failed. Please try again.', 'error');
            return { success: false, message: error.message };
        } finally {
            this.hideLoading();
        }
    }

    // Mock register function - replace with actual API call
    async mockRegister(userData) {
        return new Promise((resolve) => {
            setTimeout(() => {
                const { username, email, password, confirmPassword } = userData;
                
                // Basic validation
                if (password !== confirmPassword) {
                    resolve({
                        success: false,
                        message: 'Passwords do not match'
                    });
                    return;
                }
                
                if (password.length < 6) {
                    resolve({
                        success: false,
                        message: 'Password must be at least 6 characters long'
                    });
                    return;
                }
                
                // Check if user already exists (in a real app, this would check the database)
                const existingUsers = ['admin', 'user'];
                if (existingUsers.includes(username)) {
                    resolve({
                        success: false,
                        message: 'Username already exists'
                    });
                    return;
                }
                
                resolve({
                    success: true,
                    message: 'User registered successfully'
                });
            }, 1500);
        });
    }

    // Logout method
    logout() {
        if (this.isAuthenticated()) {
            // Call logout API if needed
            this.callLogoutAPI();
        }
        
        this.clearAuthData();
        this.updateUI();
        this.showToast('You have been logged out.', 'info');
        
        // Redirect to login page if not already there
        if (!window.location.href.includes('login.html') && !window.location.href.includes('index.html')) {
            setTimeout(() => {
                window.location.href = 'index.html';
            }, 1000);
        }
    }

    // Call logout API (if needed)
    async callLogoutAPI() {
        try {
            // In a real application, you might want to call your logout API
            // await fetch('/api/logout', {
            //     method: 'POST',
            //     headers: {
            //         'Authorization': `Bearer ${this.token}`
            //     }
            // });
        } catch (error) {
            console.error('Logout API error:', error);
        }
    }

    // Store authentication data in localStorage
    storeAuthData() {
        localStorage.setItem('authToken', this.token);
        localStorage.setItem('tokenExpiry', this.tokenExpiry);
        localStorage.setItem('userData', JSON.stringify(this.currentUser));
        localStorage.setItem('lastActivity', new Date().getTime().toString());
    }

    // Clear authentication data
    clearAuthData() {
        this.token = null;
        this.tokenExpiry = null;
        this.currentUser = null;
        
        localStorage.removeItem('authToken');
        localStorage.removeItem('tokenExpiry');
        localStorage.removeItem('userData');
        localStorage.removeItem('lastActivity');
        
        // Also clear any session storage if used
        sessionStorage.clear();
    }

    // Check if user is authenticated
    isAuthenticated() {
        if (!this.token || !this.currentUser) {
            return false;
        }

        // Check token expiry
        if (this.tokenExpiry) {
            const now = new Date().getTime();
            if (now > parseInt(this.tokenExpiry)) {
                this.logout();
                return false;
            }
        }

        return true;
    }

    // Check user role
    hasRole(role) {
        return this.isAuthenticated() && this.currentUser.role === role;
    }

    // Check if user has any of the specified roles
    hasAnyRole(roles) {
        return this.isAuthenticated() && roles.includes(this.currentUser.role);
    }

    // Get current user
    getCurrentUser() {
        return this.currentUser;
    }

    // Get auth token
    getToken() {
        return this.token;
    }

    // Update UI based on authentication state
    updateUI() {
        const isAuthenticated = this.isAuthenticated();
        
        // Update user display in navbar
        const userDisplay = $('.user-display, .navbar .user-info');
        if (userDisplay.length && this.currentUser) {
            userDisplay.html(`
                <i class="fas fa-user-circle"></i> ${this.currentUser.name}
            `);
        }

        // Show/hide login/logout buttons
        $('.auth-login').toggle(!isAuthenticated);
        $('.auth-logout').toggle(isAuthenticated);
        $('.auth-register').toggle(!isAuthenticated);

        // Update protected content visibility
        $('.protected-content').toggle(isAuthenticated);
        $('.public-content').toggle(!isAuthenticated);

        // Update role-based content
        if (isAuthenticated) {
            $('.admin-only').toggle(this.hasRole('administrator'));
            $('.user-only').toggle(this.hasRole('user'));
        } else {
            $('.admin-only, .user-only').hide();
        }

        // Update page title with user info
        if (isAuthenticated && this.currentUser) {
            document.title = `${this.currentUser.name} - AdminLTE`;
        }
    }

    // Check for auto logout based on inactivity
    checkAutoLogout() {
        const lastActivity = localStorage.getItem('lastActivity');
        if (lastActivity) {
            const now = new Date().getTime();
            const inactiveTime = now - parseInt(lastActivity);
            const maxInactiveTime = 30 * 60 * 1000; // 30 minutes
            
            if (inactiveTime > maxInactiveTime && this.isAuthenticated()) {
                this.showToast('You have been logged out due to inactivity.', 'warning');
                this.logout();
            }
        }

        // Update last activity time
        this.updateLastActivity();
        
        // Set up activity listeners
        this.setupActivityListeners();
    }

    // Update last activity time
    updateLastActivity() {
        localStorage.setItem('lastActivity', new Date().getTime().toString());
    }

    // Setup activity listeners to track user activity
    setupActivityListeners() {
        const events = ['mousedown', 'mousemove', 'keypress', 'scroll', 'touchstart'];
        
        events.forEach(event => {
            document.addEventListener(event, () => {
                this.updateLastActivity();
            }, { passive: true });
        });
    }

    // Handle unauthorized access (401 responses)
    handleUnauthorized() {
        this.showToast('Your session has expired. Please login again.', 'warning');
        this.logout();
    }

    // Password reset request
    async requestPasswordReset(email) {
        try {
            this.showLoading('Sending reset instructions...');
            
            // Mock API call - replace with actual implementation
            const response = await this.mockPasswordResetRequest(email);
            
            if (response.success) {
                this.showToast('Password reset instructions sent to your email.', 'success');
                return { success: true };
            } else {
                throw new Error(response.message || 'Password reset request failed');
            }
        } catch (error) {
            console.error('Password reset error:', error);
            this.showToast(error.message || 'Failed to send reset instructions.', 'error');
            return { success: false, message: error.message };
        } finally {
            this.hideLoading();
        }
    }

    // Mock password reset request
    async mockPasswordResetRequest(email) {
        return new Promise((resolve) => {
            setTimeout(() => {
                // Simple email validation
                const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
                if (!emailRegex.test(email)) {
                    resolve({
                        success: false,
                        message: 'Please enter a valid email address'
                    });
                    return;
                }
                
                resolve({
                    success: true,
                    message: 'Reset instructions sent'
                });
            }, 1500);
        });
    }

    // Change password
    async changePassword(currentPassword, newPassword) {
        try {
            this.showLoading('Changing password...');
            
            // Mock API call - replace with actual implementation
            const response = await this.mockChangePassword(currentPassword, newPassword);
            
            if (response.success) {
                this.showToast('Password changed successfully!', 'success');
                return { success: true };
            } else {
                throw new Error(response.message || 'Password change failed');
            }
        } catch (error) {
            console.error('Password change error:', error);
            this.showToast(error.message || 'Failed to change password.', 'error');
            return { success: false, message: error.message };
        } finally {
            this.hideLoading();
        }
    }

    // Mock change password
    async mockChangePassword(currentPassword, newPassword) {
        return new Promise((resolve) => {
            setTimeout(() => {
                // In a real app, verify current password and update to new password
                if (newPassword.length < 6) {
                    resolve({
                        success: false,
                        message: 'New password must be at least 6 characters long'
                    });
                    return;
                }
                
                resolve({
                    success: true,
                    message: 'Password changed successfully'
                });
            }, 1500);
        });
    }

    // UI Helper Methods
    showLoading(message = 'Loading...') {
        // Remove existing loading overlay
        this.hideLoading();
        
        const loadingHtml = `
            <div class="loading-overlay" style="
                position: fixed;
                top: 0;
                left: 0;
                width: 100%;
                height: 100%;
                background: rgba(0,0,0,0.5);
                display: flex;
                justify-content: center;
                align-items: center;
                z-index: 9999;
                color: white;
                font-size: 18px;
            ">
                <div class="text-center">
                    <i class="fas fa-spinner fa-spin fa-2x mb-2"></i>
                    <div>${message}</div>
                </div>
            </div>
        `;
        
        $('body').append(loadingHtml);
    }

    hideLoading() {
        $('.loading-overlay').remove();
    }

    showToast(message, type = 'info') {
        // Remove existing toasts
        $('.auth-toast').remove();
        
        const typeClasses = {
            success: 'bg-success',
            error: 'bg-danger',
            warning: 'bg-warning',
            info: 'bg-info'
        };
        
        const toastHtml = `
            <div class="auth-toast toast align-items-center text-white ${typeClasses[type] || 'bg-info'} border-0" 
                 style="position: fixed; top: 20px; right: 20px; z-index: 9999; min-width: 250px;">
                <div class="d-flex">
                    <div class="toast-body">
                        <i class="fas ${this.getToastIcon(type)} me-2"></i>
                        ${message}
                    </div>
                    <button type="button" class="btn-close btn-close-white me-2 m-auto" data-bs-dismiss="toast"></button>
                </div>
            </div>
        `;
        
        $('body').append(toastHtml);
        const toastElement = $('.auth-toast');
        
        // Initialize and show toast
        toastElement.toast({ delay: 4000 });
        toastElement.toast('show');
        
        // Remove toast after hide
        toastElement.on('hidden.bs.toast', function () {
            $(this).remove();
        });
    }

    getToastIcon(type) {
        const icons = {
            success: 'fa-check-circle',
            error: 'fa-exclamation-circle',
            warning: 'fa-exclamation-triangle',
            info: 'fa-info-circle'
        };
        return icons[type] || 'fa-info-circle';
    }

    // Validate token on page load
    validateTokenOnLoad() {
        if (this.isAuthenticated()) {
            // You could make an API call here to validate the token
            console.log('Token validated for user:', this.currentUser.username);
        }
    }
}

// Global auth instance
window.authManager = new AuthManager();

// Global helper functions
function logout() {
    if (window.authManager) {
        window.authManager.logout();
    } else {
        // Fallback logout
        localStorage.clear();
        window.location.href = 'index.html';
    }
}

function isAuthenticated() {
    return window.authManager ? window.authManager.isAuthenticated() : false;
}

function getCurrentUser() {
    return window.authManager ? window.authManager.getCurrentUser() : null;
}

function hasRole(role) {
    return window.authManager ? window.authManager.hasRole(role) : false;
}

// Initialize auth when document is ready
$(document).ready(function() {
    // Check authentication on page load
    if (window.authManager) {
        window.authManager.validateTokenOnLoad();
        
        // Protect routes that require authentication
        const protectedPages = ['countries.html', 'operators.html', 'dashboard.html'];
        const currentPage = window.location.pathname.split('/').pop();
        
        if (protectedPages.includes(currentPage) && !window.authManager.isAuthenticated()) {
            window.location.href = 'index.html';
            return;
        }
    }
});

// Export for module usage (if using ES6 modules)
if (typeof module !== 'undefined' && module.exports) {
    module.exports = { AuthManager, authManager: window.authManager };
}