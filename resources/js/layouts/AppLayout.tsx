import { Link, usePage } from '@inertiajs/react';
import type { PropsWithChildren } from 'react';

import type { SharedProps } from '../types';

export default function AppLayout({ children }: PropsWithChildren) {
  const { props } = usePage<SharedProps>();
  const appName = props.appName ?? 'Rust Inertia App';
  const environment = props.environment ?? 'local';

  return (
    <div className="flex min-h-screen flex-col bg-stone-50 text-stone-900 antialiased">
      <header className="border-b border-stone-200 bg-white">
        <div className="mx-auto flex max-w-3xl items-center justify-between px-6 py-4">
          <Link href="/" className="font-semibold tracking-tight">
            {appName}
          </Link>
          <nav className="flex gap-5 text-sm text-stone-600">
            <Link href="/" className="transition hover:text-blue-600">
              Home
            </Link>
            <Link href="/about" className="transition hover:text-blue-600">
              About
            </Link>
            <Link href="/contact" className="transition hover:text-blue-600">
              Contact
            </Link>
          </nav>
        </div>
      </header>
      <main className="mx-auto w-full max-w-3xl flex-1 px-6 py-10">
        {children}
      </main>
      <footer className="border-t border-stone-200 bg-white">
        <div className="mx-auto flex max-w-3xl items-center justify-between px-6 py-4 text-xs text-stone-500">
          <span>{appName}</span>
          <span className="rounded-full bg-stone-100 px-2.5 py-1 font-medium text-stone-600">
            {environment}
          </span>
        </div>
      </footer>
    </div>
  );
}
