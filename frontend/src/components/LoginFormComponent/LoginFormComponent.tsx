import { Button, TextField } from '@mui/material';
import React, { FC } from 'react';


interface LoginFormComponentProps { }

const LoginFormComponent: FC<LoginFormComponentProps> = () => (
  <div data-testid="LoginFormComponent">
    <TextField id="username" label="Username" variant="outlined" />
    <br />
    <TextField id="password" label="Password" variant="outlined" />
    <br />
    <Button variant="contained">Login</Button>
  </div>
);

export default LoginFormComponent;
