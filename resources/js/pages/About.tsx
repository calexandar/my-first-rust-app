import { Head } from '@inertiajs/react';

type Props = {
  message: string;
};

export default function About({ message }: Props) {
  return (
    <>
      <Head title="About" />
      <section className="space-y-4">
        <h1 className="text-3xl font-bold tracking-tight">About</h1>
        <p className="text-lg text-stone-700">{message}</p>
        <p className="text-sm text-stone-500">
          Navigation between pages is handled by the Inertia client, without a
          client-side router.
        </p>
      </section>
    </>
  );
}
