import React from 'react';
import { 
  Drawer, 
  List, 
  ListItem, 
  ListItemIcon, 
  ListItemText, 
  Divider,
  Toolbar 
} from '@mui/material';
import DashboardIcon from '@mui/icons-material/Dashboard';
import AnalyticsIcon from '@mui/icons-material/Analytics';
import SettingsIcon from '@mui/icons-material/Settings';

const Sidebar = ({ drawerWidth, mobileOpen, handleDrawerToggle }) => {
  return (
    <>
      <Drawer
        variant="temporary"
        open={mobileOpen}
        onClose={handleDrawerToggle}
        ModalProps={{ keepMounted: true }}
        sx={{
          display: { xs: 'block', sm: 'none' },
          '& .MuiDrawer-paper': { boxSizing: 'border-box', width: drawerWidth },
        }}
      >
        <SidebarContent drawerWidth={drawerWidth} />
      </Drawer>
      <Drawer
        variant="permanent"
        sx={{
          display: { xs: 'none', sm: 'block' },
          '& .MuiDrawer-paper': { boxSizing: 'border-box', width: drawerWidth },
        }}
        open
      >
        <SidebarContent drawerWidth={drawerWidth} />
      </Drawer>
    </>
  );
};

const SidebarContent = ({ drawerWidth }) => (
  <div>
    <Toolbar />
    <Divider />
    <List>
      {[
        { text: 'Dashboard', icon: <DashboardIcon />, path: '/' },
        { text: 'Analytics', icon: <AnalyticsIcon />, path: '/analytics' },
        { text: 'Settings', icon: <SettingsIcon />, path: '/settings' },
      ].map((item) => (
        <ListItem button key={item.text} component="a" href={item.path}>
          <ListItemIcon>{item.icon}</ListItemIcon>
          <ListItemText primary={item.text} />
        </ListItem>
      ))}
    </List>
  </div>
);

export default Sidebar;