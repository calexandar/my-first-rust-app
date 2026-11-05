import type { PageProps } from '@inertiajs/core';
import { Link, usePage } from '@inertiajs/react';
import type { PropsWithChildren } from 'react';

type SharedProps = PageProps & {
  appName?: string;
};

export default function AppLayout({ children }: PropsWithChildren) {
  const { props } = usePage<SharedProps>();
  const appName = props.appName ?? 'Rust Inertia App';

  return (
    <div className="min-h-screen bg-stone-50 text-stone-900 antialiased">
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
          </nav>
        </div>
      </header>
      <main className="mx-auto max-w-3xl px-6 py-10">{children}</main>
    </div>
  );
}
