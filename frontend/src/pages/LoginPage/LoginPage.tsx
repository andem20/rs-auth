import React, { FC } from 'react';
import LoginFormComponent from '../../components/LoginFormComponent/LoginFormComponent.lazy';
import { Grid } from '@mui/material';


interface LoginPageProps { }

const LoginPage: FC<LoginPageProps> = () => (
  <div data-testid="LoginPage">
    <Grid
      container
      spacing={0}
      direction="column"
      alignItems="center"
      justifyContent="center"
      sx={{ minHeight: '100vh' }}
    >
      <Grid item xs={3}>
        <LoginFormComponent />
      </Grid>
    </Grid>
  </div>
);

export default LoginPage;
