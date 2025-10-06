/**
 * Dashboard Controller
 * Handles dashboard-specific functionality including charts and data updates
 */

class Dashboard {
    static app = null;
    static performanceChart = null;
    static currentTimeFilter = 7;

    static async init(app) {
        this.app = app;
        await this.loadDashboardData();
        this.setupEventListeners();
        this.initializeCharts();
        this.startAutoRefresh();
    }

    /**
     * Load all dashboard data from API
     */
    static async loadDashboardData() {
        try {
            // Simulate API calls with mock data
            const dashboardData = await this.getMockDashboardData();
            
            this.updateMainCards(dashboardData.mainCards);
            this.updateStatCards(dashboardData.statCards);
            this.updateSystemStatus(dashboardData.systemStatus);
            this.updateRecentActivity(dashboardData.recentActivity);
            this.updatePerformanceChart(dashboardData.performanceData);

        } catch (error) {
            console.error('Error loading dashboard data:', error);
            this.setDefaultValues();
            this.app.showError('Failed to load dashboard data. Using demo data.');
        }
    }

    /**
     * Update main KPI cards
     */
    static updateMainCards(data) {
        document.getElementById('roam-in-count').textContent = this.formatNumber(data.roamIn.count);
        document.getElementById('roam-in-change').textContent = data.roamIn.change;
        
        document.getElementById('roam-out-count').textContent = this.formatNumber(data.roamOut.count);
        document.getElementById('roam-out-change').textContent = data.roamOut.change;
        
        document.getElementById('alerts-count').textContent = this.formatNumber(data.alerts.count);
        document.getElementById('critical-alerts').textContent = data.alerts.critical;
        
        document.getElementById('notifications-count').textContent = this.formatNumber(data.notifications.count);
        document.getElementById('new-notifications').textContent = data.notifications.new;
    }

    /**
     * Update stat cards
     */
    static updateStatCards(data) {
        document.getElementById('countries-count').textContent = this.formatNumber(data.countries);
        document.getElementById('operators-count').textContent = this.formatNumber(data.operators);
        document.getElementById('plans-count').textContent = this.formatNumber(data.plans);
        document.getElementById('users-count').textContent = this.formatNumber(data.users);
    }

    /**
     * Update system status
     */
    static updateSystemStatus(status) {
        document.getElementById('api-status').textContent = status.api;
        document.getElementById('db-status').textContent = status.database;
        document.getElementById('partners-status').textContent = status.partners;
        document.getElementById('last-sync').textContent = status.lastSync;

        // Update badge colors based on status
        this.updateStatusBadge('api-status', status.api);
        this.updateStatusBadge('db-status', status.database);
    }

    /**
     * Update status badge colors
     */
    static updateStatusBadge(elementId, status) {
        const element = document.getElementById(elementId);
        const badgeClass = status.toLowerCase() === 'online' ? 'badge-success' : 
                          status.toLowerCase() === 'offline' ? 'badge-danger' : 'badge-warning';
        
        element.className = `badge ${badgeClass}`;
    }

    /**
     * Update recent activity table
     */
    static updateRecentActivity(activities) {
        const tbody = document.getElementById('recent-activity');
        if (!tbody) return;

        tbody.innerHTML = activities.map(activity => `
            <tr>
                <td>
                    <span class="badge badge-${this.getActivityBadgeClass(activity.type)}">
                        ${activity.type}
                    </span>
                </td>
                <td>${activity.description}</td>
                <td>${activity.user}</td>
                <td>${this.formatTime(activity.timestamp)}</td>
                <td>
                    <span class="status-indicator status-${activity.status}"></span>
                    ${activity.status}
                </td>
            </tr>
        `).join('') || '<tr><td colspan="5" class="text-center text-muted">No recent activity</td></tr>';
    }

    /**
     * Get badge class based on activity type
     */
    static getActivityBadgeClass(type) {
        const typeMap = {
            'success': 'success',
            'error': 'danger',
            'warning': 'warning',
            'info': 'info',
            'update': 'primary',
            'create': 'success',
            'delete': 'danger'
        };
        return typeMap[type.toLowerCase()] || 'secondary';
    }

    /**
     * Initialize charts
     */
    static initializeCharts() {
        this.createPerformanceChart();
    }

    /**
     * Create performance chart
     */
    static createPerformanceChart() {
        const ctx = document.getElementById('performanceChart');
        if (!ctx) return;

        // Destroy existing chart if it exists
        if (this.performanceChart) {
            this.performanceChart.destroy();
        }

        this.performanceChart = new Chart(ctx.getContext('2d'), {
            type: 'line',
            data: {
                labels: [],
                datasets: [
                    {
                        label: 'Roam IN',
                        data: [],
                        borderColor: '#007bff',
                        backgroundColor: 'rgba(0, 123, 255, 0.1)',
                        tension: 0.4,
                        fill: true,
                        borderWidth: 2
                    },
                    {
                        label: 'Roam OUT',
                        data: [],
                        borderColor: '#28a745',
                        backgroundColor: 'rgba(40, 167, 69, 0.1)',
                        tension: 0.4,
                        fill: true,
                        borderWidth: 2
                    }
                ]
            },
            options: {
                responsive: true,
                maintainAspectRatio: false,
                plugins: {
                    legend: {
                        position: 'top',
                        labels: {
                            padding: 20,
                            usePointStyle: true
                        }
                    },
                    tooltip: {
                        mode: 'index',
                        intersect: false
                    }
                },
                scales: {
                    x: {
                        grid: {
                            display: false
                        }
                    },
                    y: {
                        beginAtZero: true,
                        grid: {
                            borderDash: [2, 2]
                        },
                        ticks: {
                            callback: function(value) {
                                return value >= 1000 ? (value/1000) + 'k' : value;
                            }
                        }
                    }
                },
                interaction: {
                    mode: 'nearest',
                    axis: 'x',
                    intersect: false
                }
            }
        });
    }

    /**
     * Update performance chart with new data
     */
    static updatePerformanceChart(data) {
        if (!this.performanceChart) return;

        this.performanceChart.data.labels = data.labels;
        this.performanceChart.data.datasets[0].data = data.roamIn;
        this.performanceChart.data.datasets[1].data = data.roamOut;
        this.performanceChart.update();
    }

    /**
     * Setup event listeners
     */
    static setupEventListeners() {
        // Time filter dropdown
        document.addEventListener('click', (e) => {
            if (e.target.classList.contains('time-filter')) {
                e.preventDefault();
                this.handleTimeFilter(e.target);
            }
        });

        // Refresh activity button
        const refreshBtn = document.getElementById('refresh-activity');
        if (refreshBtn) {
            refreshBtn.addEventListener('click', () => {
                this.refreshActivity();
            });
        }

        // View alerts button
        const viewAlertsBtn = document.getElementById('view-alerts-btn');
        if (viewAlertsBtn) {
            viewAlertsBtn.addEventListener('click', (e) => {
                e.preventDefault();
                this.showAlertsModal();
            });
        }

        // View notifications button
        const viewNotificationsBtn = document.getElementById('view-notifications-btn');
        if (viewNotificationsBtn) {
            viewNotificationsBtn.addEventListener('click', (e) => {
                e.preventDefault();
                this.app.showInfo('Notifications feature coming soon!');
            });
        }
    }

    /**
     * Handle time filter selection
     */
    static handleTimeFilter(target) {
        // Remove active class from all filters
        document.querySelectorAll('.time-filter').forEach(item => {
            item.classList.remove('active');
        });

        // Add active class to clicked filter
        target.classList.add('active');

        // Update chart data based on selected period
        const period = parseInt(target.getAttribute('data-period'));
        this.currentTimeFilter = period;
        this.refreshChartData();
    }

    /**
     * Refresh chart data based on current filter
     */
    static async refreshChartData() {
        try {
            // Simulate API call for filtered data
            const filteredData = await this.getFilteredPerformanceData(this.currentTimeFilter);
            this.updatePerformanceChart(filteredData);
        } catch (error) {
            console.error('Error refreshing chart data:', error);
        }
    }

    /**
     * Refresh activity data
     */
    static async refreshActivity() {
        const refreshBtn = document.getElementById('refresh-activity');
        if (refreshBtn) {
            refreshBtn.querySelector('i').classList.add('fa-spin');
        }

        try {
            // Simulate API call
            await new Promise(resolve => setTimeout(resolve, 1000));
            const newActivity = await this.getMockRecentActivity();
            this.updateRecentActivity(newActivity);
            this.app.showSuccess('Activity data refreshed');
        } catch (error) {
            console.error('Error refreshing activity:', error);
            this.app.showError('Failed to refresh activity data');
        } finally {
            if (refreshBtn) {
                refreshBtn.querySelector('i').classList.remove('fa-spin');
            }
        }
    }

    /**
     * Show alerts modal
     */
    static async showAlertsModal() {
        try {
            const alerts = await this.getMockAlerts();
            this.populateAlertsModal(alerts);
            $('#alertsModal').modal('show');
        } catch (error) {
            console.error('Error loading alerts:', error);
            this.app.showError('Failed to load alerts');
        }
    }

    /**
     * Populate alerts modal
     */
    static populateAlertsModal(alerts) {
        const container = document.getElementById('alerts-list');
        if (!container) return;

        container.innerHTML = alerts.map(alert => `
            <div class="alert alert-${alert.severity} alert-dismissible">
                <h5><i class="icon fas fa-${alert.icon}"></i> ${alert.title}</h5>
                <p>${alert.message}</p>
                <small class="text-muted">${this.formatTime(alert.timestamp)}</small>
            </div>
        `).join('') || '<p class="text-muted text-center">No active alerts</p>';
    }

    /**
     * Start auto-refresh for dashboard data
     */
    static startAutoRefresh() {
        // Refresh data every 2 minutes
        setInterval(() => {
            this.loadDashboardData();
        }, 120000);
    }

    /**
     * Format number with K/M suffixes
     */
    static formatNumber(num) {
        if (num >= 1000000) {
            return (num / 1000000).toFixed(1) + 'M';
        }
        if (num >= 1000) {
            return (num / 1000).toFixed(1) + 'K';
        }
        return num.toString();
    }

    /**
     * Format timestamp to relative time
     */
    static formatTime(timestamp) {
        const now = new Date();
        const time = new Date(timestamp);
        const diffInMinutes = Math.floor((now - time) / (1000 * 60));
        
        if (diffInMinutes < 1) return 'Just now';
        if (diffInMinutes < 60) return `${diffInMinutes}m ago`;
        if (diffInMinutes < 1440) return `${Math.floor(diffInMinutes / 60)}h ago`;
        return `${Math.floor(diffInMinutes / 1440)}d ago`;
    }

    /**
     * Set default values when API fails
     */
    static setDefaultValues() {
        const defaultData = this.getMockDashboardData();
        this.updateMainCards(defaultData.mainCards);
        this.updateStatCards(defaultData.statCards);
        this.updateSystemStatus(defaultData.systemStatus);
        this.updateRecentActivity(defaultData.recentActivity);
    }

    /**
     * Mock data for demonstration
     */
    static async getMockDashboardData() {
        // Simulate API delay
        await new Promise(resolve => setTimeout(resolve, 500));

        return {
            mainCards: {
                roamIn: { count: 12542, change: '12.5%' },
                roamOut: { count: 8945, change: '5.2%' },
                alerts: { count: 23, critical: 3 },
                notifications: { count: 156, new: 8 }
            },
            statCards: {
                countries: 84,
                operators: 245,
                plans: 56,
                users: 42
            },
            systemStatus: {
                api: 'Online',
                database: 'Online',
                partners: '24/25',
                lastSync: 'Just now'
            },
            recentActivity: this.getMockRecentActivity(),
            performanceData: this.getMockPerformanceData(7)
        };
    }

    static getMockRecentActivity() {
        return [
            {
                type: 'success',
                description: 'Roaming agreement updated with Operator A',
                user: 'admin',
                timestamp: new Date(Date.now() - 5 * 60000).toISOString(),
                status: 'completed'
            },
            {
                type: 'warning',
                description: 'High latency detected in Region B',
                user: 'system',
                timestamp: new Date(Date.now() - 15 * 60000).toISOString(),
                status: 'investigating'
            },
            {
                type: 'info',
                description: 'New country added to roaming network',
                user: 'admin',
                timestamp: new Date(Date.now() - 45 * 60000).toISOString(),
                status: 'completed'
            },
            {
                type: 'error',
                description: 'API connection timeout with Partner C',
                user: 'system',
                timestamp: new Date(Date.now() - 2 * 3600000).toISOString(),
                status: 'resolved'
            },
            {
                type: 'success',
                description: 'Monthly performance report generated',
                user: 'system',
                timestamp: new Date(Date.now() - 4 * 3600000).toISOString(),
                status: 'completed'
            }
        ];
    }

    static getMockPerformanceData(days = 7) {
        const labels = [];
        const roamIn = [];
        const roamOut = [];
        
        for (let i = days - 1; i >= 0; i--) {
            const date = new Date();
            date.setDate(date.getDate() - i);
            labels.push(date.toLocaleDateString('en', { month: 'short', day: 'numeric' }));
            
            roamIn.push(Math.floor(Math.random() * 2000) + 1000);
            roamOut.push(Math.floor(Math.random() * 1500) + 800);
        }
        
        return { labels, roamIn, roamOut };
    }

    static async getFilteredPerformanceData(days) {
        await new Promise(resolve => setTimeout(resolve, 300));
        return this.getMockPerformanceData(days);
    }

    static async getMockAlerts() {
        await new Promise(resolve => setTimeout(resolve, 300));
        
        return [
            {
                severity: 'danger',
                icon: 'exclamation-triangle',
                title: 'Critical: Service Outage',
                message: 'Roaming services temporarily unavailable in Region X',
                timestamp: new Date(Date.now() - 30 * 60000).toISOString()
            },
            {
                severity: 'warning',
                icon: 'clock',
                title: 'High Latency',
                message: 'Increased response time detected with multiple operators',
                timestamp: new Date(Date.now() - 2 * 3600000).toISOString()
            },
            {
                severity: 'info',
                icon: 'info-circle',
                title: 'Maintenance Scheduled',
                message: 'Planned maintenance for database optimization',
                timestamp: new Date(Date.now() - 6 * 3600000).toISOString()
            }
        ];
    }
}

export default Dashboard;