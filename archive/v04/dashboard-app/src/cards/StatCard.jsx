import React from 'react';
import { Card, CardContent, Typography, Stack } from '@mui/material';
import TrendingUpIcon from '@mui/icons-material/TrendingUp';
import TrendingDownIcon from '@mui/icons-material/TrendingDown';

const StatCard = ({ title, value, change, positive }) => {
  return (
    <Card sx={{ minWidth: 200 }}>
      <CardContent>
        <Typography color="text.secondary" gutterBottom>
          {title}
        </Typography>
        <Typography variant="h5" component="div">
          {value}
        </Typography>
        <Stack direction="row" alignItems="center" spacing={0.5}>
          {positive ? (
            <TrendingUpIcon color="success" />
          ) : (
            <TrendingDownIcon color="error" />
          )}
          <Typography variant="body2" color={positive ? 'success.main' : 'error.main'}>
            {change}
          </Typography>
        </Stack>
      </CardContent>
    </Card>
  );
};

export default StatCard;