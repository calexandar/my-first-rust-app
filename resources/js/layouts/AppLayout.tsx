import { Link, router, usePage } from '@inertiajs/react';
import type { PropsWithChildren } from 'react';

import type { SharedProps } from '../types';

export default function AppLayout({ children }: PropsWithChildren) {
  const { props, flash } = usePage<SharedProps>();
  const appName = props.appName ?? 'Rust Inertia App';
  const environment = props.environment ?? 'local';
  const user = props.auth?.user ?? null;
  const csrfToken = props.csrfToken ?? '';
  const success = typeof flash.success === 'string' ? flash.success : null;

  const signOut = () => {
    router.post(
      '/logout',
      {},
      { headers: { 'X-CSRF-TOKEN': csrfToken } },
    );
  };

  return (
    <div className="flex min-h-screen flex-col bg-stone-50 text-stone-900 antialiased">
      <header className="border-b border-stone-200 bg-white">
        <div className="mx-auto flex max-w-3xl flex-wrap items-center justify-between gap-3 px-6 py-4">
          <Link href="/" className="font-semibold tracking-tight">
            {appName}
          </Link>
          <nav className="flex flex-wrap items-center gap-5 text-sm text-stone-600">
            <Link href="/" className="transition hover:text-blue-600">
              Home
            </Link>
            <Link href="/about" className="transition hover:text-blue-600">
              About
            </Link>
            <Link href="/contact" className="transition hover:text-blue-600">
              Contact
            </Link>
            {user ? (
              <>
                <Link
                  href="/dashboard"
                  className="transition hover:text-blue-600"
                >
                  Dashboard
                </Link>
                {user.is_admin && (
                  <Link
                    href="/admin/users"
                    className="transition hover:text-blue-600"
                  >
                    Admin
                  </Link>
                )}
                <span className="text-stone-400">|</span>
                <span className="font-medium text-stone-700">{user.name}</span>
                <button
                  type="button"
                  onClick={signOut}
                  className="rounded-lg border border-stone-300 px-3 py-1.5 text-sm font-medium text-stone-700 transition hover:border-stone-400 hover:bg-stone-50 focus:outline-none focus:ring-2 focus:ring-blue-300 focus:ring-offset-2"
                >
                  Sign out
                </button>
              </>
            ) : (
              <>
                <Link
                  href="/login"
                  className="transition hover:text-blue-600"
                >
                  Log in
                </Link>
                <Link
                  href="/register"
                  className="rounded-lg bg-blue-600 px-3 py-1.5 text-sm font-semibold text-white transition hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-300 focus:ring-offset-2"
                >
                  Register
                </Link>
              </>
            )}
          </nav>
        </div>
      </header>
      <main className="mx-auto w-full max-w-3xl flex-1 px-6 py-10">
        {success && (
          <div
            role="status"
            aria-live="polite"
            className="mb-6 rounded-xl border border-emerald-200 bg-emerald-50 px-4 py-3 text-sm text-emerald-800"
          >
            {success}
          </div>
        )}
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
