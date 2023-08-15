import React from 'react';
import { render, screen } from '@testing-library/react';
import '@testing-library/jest-dom/extend-expect';
import LoginFormComponent from './LoginFormComponent';

describe('<LoginFormComponent />', () => {
  test('it should mount', () => {
    render(<LoginFormComponent />);
    
    const loginFormComponent = screen.getByTestId('LoginFormComponent');

    expect(loginFormComponent).toBeInTheDocument();
  });
});