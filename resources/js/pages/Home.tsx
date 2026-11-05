import { Head } from '@inertiajs/react';

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
      </section>
    </>
  );
}
