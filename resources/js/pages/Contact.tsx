import { Head, useForm, usePage } from '@inertiajs/react';
import type { FormEvent } from 'react';

import type { SharedProps } from '../types';

type ContactMessage = {
  id: number;
  name: string;
  email: string;
  message: string;
  created_at: string;
};

type Props = {
  headline: string;
  supportEmail: string;
  messages: ContactMessage[];
};

type ContactForm = {
  name: string;
  email: string;
  message: string;
};

const fieldClass =
  'w-full rounded-lg border border-stone-300 bg-white px-3 py-2 text-sm text-stone-900 shadow-sm outline-none transition focus:border-blue-500 focus:ring-2 focus:ring-blue-200 disabled:cursor-not-allowed disabled:bg-stone-100';

export default function Contact({
  headline,
  supportEmail,
  messages,
}: Props) {
  const { flash, props } = usePage<SharedProps>();
  const csrfToken = props.csrfToken ?? '';
  const form = useForm<ContactForm>({ name: '', email: '', message: '' });
  const success = typeof flash.success === 'string' ? flash.success : null;

  const submit = (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();

    form.post('/contact', {
      headers: { 'X-CSRF-TOKEN': csrfToken },
      preserveScroll: true,
      onSuccess: () => form.reset(),
    });
  };

  return (
    <>
      <Head title="Contact" />
      <section className="space-y-6">
        <header className="space-y-2">
          <h1 className="text-3xl font-bold tracking-tight">Contact</h1>
          <p className="text-lg text-stone-700">{headline}</p>
          <p className="text-sm text-stone-500">
            This form is validated on the server. Field errors and the success
            message come back through Inertia, and saved messages are stored in
            PostgreSQL.
          </p>
        </header>

        {success && (
          <div
            role="status"
            aria-live="polite"
            className="rounded-xl border border-emerald-200 bg-emerald-50 px-4 py-3 text-sm text-emerald-800"
          >
            {success}
          </div>
        )}

        <form
          onSubmit={submit}
          noValidate
          className="space-y-5 rounded-2xl border border-stone-200 bg-white p-6 shadow-sm"
        >
          <div className="space-y-2">
            <label htmlFor="name" className="block text-sm font-medium">
              Name
            </label>
            <input
              id="name"
              name="name"
              type="text"
              autoComplete="name"
              value={form.data.name}
              onChange={(event) => form.setData('name', event.target.value)}
              disabled={form.processing}
              aria-invalid={Boolean(form.errors.name) || undefined}
              aria-describedby={form.errors.name ? 'name-error' : undefined}
              className={fieldClass}
            />
            {form.errors.name && (
              <p id="name-error" role="alert" className="text-sm text-red-600">
                {form.errors.name}
              </p>
            )}
          </div>

          <div className="space-y-2">
            <label htmlFor="email" className="block text-sm font-medium">
              Email
            </label>
            <input
              id="email"
              name="email"
              type="email"
              autoComplete="email"
              value={form.data.email}
              onChange={(event) => form.setData('email', event.target.value)}
              disabled={form.processing}
              aria-invalid={Boolean(form.errors.email) || undefined}
              aria-describedby={form.errors.email ? 'email-error' : undefined}
              className={fieldClass}
            />
            {form.errors.email && (
              <p id="email-error" role="alert" className="text-sm text-red-600">
                {form.errors.email}
              </p>
            )}
          </div>

          <div className="space-y-2">
            <label htmlFor="message" className="block text-sm font-medium">
              Message
            </label>
            <textarea
              id="message"
              name="message"
              rows={5}
              value={form.data.message}
              onChange={(event) => form.setData('message', event.target.value)}
              disabled={form.processing}
              aria-invalid={Boolean(form.errors.message) || undefined}
              aria-describedby={form.errors.message ? 'message-error' : undefined}
              className={fieldClass}
            />
            {form.errors.message && (
              <p
                id="message-error"
                role="alert"
                className="text-sm text-red-600"
              >
                {form.errors.message}
              </p>
            )}
          </div>

          <div className="flex flex-wrap items-center justify-between gap-4">
            <p className="text-xs text-stone-500">
              Questions? Email {supportEmail}. Saved messages are only shown in
              this browser session.
            </p>
            <button
              type="submit"
              disabled={form.processing}
              className="inline-flex items-center justify-center rounded-lg bg-blue-600 px-4 py-2 text-sm font-semibold text-white transition hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-300 focus:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-60"
            >
              {form.processing ? 'Sending...' : 'Send message'}
            </button>
          </div>
        </form>

        <section aria-labelledby="messages-heading" className="space-y-3">
          <div className="flex flex-wrap items-baseline justify-between gap-2">
            <h2 id="messages-heading" className="text-lg font-semibold">
              Recent messages from this browser
            </h2>
            <span className="text-xs text-stone-500">
              {messages.length} saved
            </span>
          </div>

          {messages.length === 0 ? (
            <p className="rounded-xl border border-dashed border-stone-300 bg-white px-4 py-6 text-sm text-stone-500">
              No messages yet. Submit the form to save one.
            </p>
          ) : (
            <ul className="space-y-3">
              {messages.map((message) => (
                <li
                  key={message.id}
                  className="rounded-xl border border-stone-200 bg-white p-4 shadow-sm"
                >
                  <div className="flex flex-wrap items-baseline justify-between gap-2">
                    <p className="font-medium">{message.name}</p>
                    <time
                      dateTime={message.created_at}
                      className="text-xs text-stone-500"
                    >
                      {new Date(message.created_at).toLocaleString()}
                    </time>
                  </div>
                  <p className="mt-1 text-sm text-stone-500">{message.email}</p>
                  <p className="mt-3 whitespace-pre-wrap text-sm text-stone-700">
                    {message.message}
                  </p>
                </li>
              ))}
            </ul>
          )}
        </section>
      </section>
    </>
  );
}
