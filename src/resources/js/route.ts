import { usePage } from '@inertiajs/react';

export function useRoute() {
  const { props } = usePage<any>();
  const routes = (props.routes || {}) as Record<string, string>;
  const appUrl = (props.app_url || '').replace(/\/$/, '');

  return (name: string, params?: Record<string, string | number>) => {
    let path = routes[name];
    if (!path) {
      console.warn(`Route named "${name}" not found.`);
      return name;
    }

    if (params) {
      const remainingParams = { ...params };
      Object.entries(params).forEach(([key, val]) => {
        const placeholder1 = `:${key}`;
        const placeholder2 = `{${key}}`;
        let replaced = false;

        if (path.includes(placeholder1)) {
          path = path.replace(placeholder1, String(val));
          replaced = true;
        }
        if (path.includes(placeholder2)) {
          path = path.replace(placeholder2, String(val));
          replaced = true;
        }

        if (replaced) {
          delete remainingParams[key];
        }
      });

      // Append remaining parameters as query string
      const queryKeys = Object.keys(remainingParams);
      if (queryKeys.length > 0) {
        const queryString = queryKeys
          .map(k => `${encodeURIComponent(k)}=${encodeURIComponent(String(remainingParams[k]))}`)
          .join('&');
        path += (path.includes('?') ? '&' : '?') + queryString;
      }
    }

    const cleanPath = path.startsWith('/') ? path : `/${path}`;
    return `${appUrl}${cleanPath}`;
  };
}
