import React from 'react';
import { Grid, Paper, Typography } from '@mui/material';
import StatCard from '../components/cards/StatCard';
import LineChart from '../components/charts/LineChart';
import PieChart from '../components/charts/PieChart';

const Dashboard = () => {
  return (
    <>
      <Typography variant="h4" gutterBottom>
        Dashboard Overview
      </Typography>
      
      <Grid container spacing={3} sx={{ mb: 3 }}>
        <Grid item xs={12} sm={6} md={3}>
          <StatCard 
            title="Total Revenue" 
            value="$24,345" 
            change="12% from last month" 
            positive 
          />
        </Grid>
        <Grid item xs={12} sm={6} md={3}>
          <StatCard 
            title="Active Users" 
            value="1,234" 
            change="8% from last month" 
            positive 
          />
        </Grid>
        <Grid item xs={12} sm={6} md={3}>
          <StatCard 
            title="New Orders" 
            value="356" 
            change="3% from last month" 
            positive={false} 
          />
        </Grid>
        <Grid item xs={12} sm={6} md={3}>
          <StatCard 
            title="Bounce Rate" 
            value="24%" 
            change="2% from last month" 
            positive={false} 
          />
        </Grid>
      </Grid>

      <Grid container spacing={3}>
        <Grid item xs={12} md={8}>
          <Paper sx={{ p: 2 }}>
            <Typography variant="h6" gutterBottom>
              Revenue Overview
            </Typography>
            <LineChart />
          </Paper>
        </Grid>
        <Grid item xs={12} md={4}>
          <Paper sx={{ p: 2 }}>
            <Typography variant="h6" gutterBottom>
              Traffic Sources
            </Typography>
            <PieChart />
          </Paper>
        </Grid>
      </Grid>
    </>
  );
};

export default Dashboard;