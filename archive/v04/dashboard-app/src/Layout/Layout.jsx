import React, { useState } from 'react';
import { Box, CssBaseline, Toolbar } from '@mui/material';
import Navbar from './Navbar';
import Sidebar from './Sidebar';
import { ThemeProvider } from '@mui/material/styles';

const drawerWidth = 240;

const Layout = ({ children, theme, darkMode, setDarkMode }) => {
  const [mobileOpen, setMobileOpen] = useState(false);

  const handleDrawerToggle = () => {
    setMobileOpen(!mobileOpen);
  };

  return (
    <ThemeProvider theme={theme}>
      <Box sx={{ display: 'flex' }}>
        <CssBaseline />
        <Navbar 
          handleDrawerToggle={handleDrawerToggle} 
          darkMode={darkMode}
          setDarkMode={setDarkMode}
        />
        <Sidebar 
          drawerWidth={drawerWidth}
          mobileOpen={mobileOpen}
          handleDrawerToggle={handleDrawerToggle}
        />
        <Box
          component="main"
          sx={{ 
            flexGrow: 1, 
            p: 3, 
            width: { sm: `calc(100% - ${drawerWidth}px)` },
            backgroundColor: theme.palette.background.default,
            minHeight: '100vh'
          }}
        >
          <Toolbar />
          {children}
        </Box>
      </Box>
    </ThemeProvider>
  );
};

export default Layout;