export * from './types';
export * from './mocks';

export function formatDate(dateString: string): string {
  return new Date(dateString).toLocaleDateString();
}