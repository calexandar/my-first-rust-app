import '../css/app.css';

import { createInertiaApp } from '@inertiajs/react';
import AppLayout from './layouts/AppLayout';

createInertiaApp({
  pages: './pages',
  layout: () => AppLayout,
  title: (title) => (title ? `${title} · Rust Inertia App` : 'Rust Inertia App'),
  progress: { color: '#2563eb', delay: 150 },
});
