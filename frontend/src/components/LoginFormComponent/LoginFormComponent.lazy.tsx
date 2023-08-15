import React, { lazy, Suspense } from 'react';

const LazyLoginFormComponent = lazy(() => import('./LoginFormComponent'));

const LoginFormComponent = (props: JSX.IntrinsicAttributes & { children?: React.ReactNode; }) => (
  <Suspense fallback={null}>
    <LazyLoginFormComponent {...props} />
  </Suspense>
);

export default LoginFormComponent;
