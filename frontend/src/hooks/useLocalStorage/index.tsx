// import { useCallback, useState } from "react";

// export const useLocalStorage = <T,>(key: string, initialValue: T) => {
//   const [storedValue, setStoredValue] = useState<T>(() => {
//     try {
//       const item = window.localStorage.getItem(key);
//       return item ? JSON.parse(item) : initialValue;
//     } catch {
//       return initialValue;
//     }
//   });

//   const setValue = useCallback(
//     (value: T) => {
//       setStoredValue(value);
//       window.localStorage.setItem(key, JSON.stringify(value));
//     },
//     [key]
//   );

//   return [storedValue, setValue] as const;
// };

// import { LocalStorage } from '@novakid/frontend-core';
// import { type SetStateAction, useCallback, useLayoutEffect, useState } from 'react';

// import { LocalStorageKeys, StoredStateByKey } from 'src/types';

// const listeners = new Map<string, Set<(arg: unknown) => void>>();

// export const useLocalStorage = <K extends LocalStorageKeys, T extends StoredStateByKey[K]>(
//   key: K,
//   defaultValue?: T,
// ) => {
//   const [state, _setState] = useState((LocalStorage.getItem(key) as T | undefined) ?? defaultValue);

//   const setState = useCallback(
//     (value: SetStateAction<T | undefined>) => {
//       const newVal = typeof value === 'function' ? value(LocalStorage.getItem(key) as T | undefined) : value;
//       _setState(newVal);
//       LocalStorage.setItem(key, newVal);
//       listeners.get(key)?.forEach((listener) => {
//         listener(newVal);
//       });
//     },
//     [key],
//   );

//   useLayoutEffect(() => {
//     if (!listeners.has(key)) {
//       listeners.set(key, new Set());
//     }
//     listeners.get(key)?.add(_setState as (arg: unknown) => void);
//     return () => {
//       listeners.get(key)?.delete(_setState as (arg: unknown) => void);
//       if (listeners.get(key)?.size === 0) {
//         listeners.delete(key);
//       }
//     };
//   }, [key]);

//   return [state, setState] as const;
// };