import { Head, Link } from '@inertiajs/react';

type Props = {
  message: string;
};

export default function Home({ message }: Props) {
  return (
    <>
      <Head title="Home" />
      <section className="space-y-4">
        <h1 className="text-3xl font-bold tracking-tight">Rust + Inertia</h1>
        <p className="text-lg text-stone-700">{message}</p>
        <p className="text-sm text-stone-500">
          This page was routed by Axum and hydrated by React.
        </p>
        <Link
          href="/contact"
          className="inline-flex items-center justify-center rounded-lg bg-blue-600 px-4 py-2 text-sm font-semibold text-white transition hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-300 focus:ring-offset-2"
        >
          Try the contact form
        </Link>
      </section>
    </>
  );
}
