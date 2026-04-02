import type { MetaFunction } from '@remix-run/node';

export const meta: MetaFunction = () => {
  return [{ title: 'Hello Remix' }];
};

export default function Index() {
  return (
    <main style={{ fontFamily: 'sans-serif', maxWidth: '600px', margin: '4rem auto', padding: '0 1rem' }}>
      <h1>Hello from Remix</h1>
      <p>
        This app is running on <a href="https://partiri.com">Partiri</a>.
      </p>
    </main>
  );
}
