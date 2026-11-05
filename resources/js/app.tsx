import '../css/app.css';

import { createInertiaApp } from '@inertiajs/react';
import AppLayout from './layouts/AppLayout';

createInertiaApp({
  pages: './pages',
  layout: (name) => (name === 'Home' ? null : AppLayout),
  title: (title) => (title ? `${title} · Rust Inertia App` : 'Rust Inertia App'),
  progress: { color: '#4f8ef7', delay: 150 },
});
