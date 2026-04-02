import { json } from '@remix-run/node';
import type { LoaderFunctionArgs } from '@remix-run/node';

export const loader = (_args: LoaderFunctionArgs) => {
  return json({ status: 'ok' });
};
