import { createContext, useContext } from 'react';
import { en, type Messages } from './en';
import { vi } from './vi';
import { zhCN } from './zh-CN';

export type Locale = 'en' | 'zh-CN' | 'vi';

export const locales: Locale[] = ['en', 'zh-CN', 'vi'];

const messagesByLocale: Record<Locale, Messages> = { en, 'zh-CN': zhCN, vi };

const pathPrefixes: Record<Locale, string> = { en: '', 'zh-CN': '/zh', vi: '/vi' };

export function isLocale(value: string): value is Locale {
  return (locales as string[]).includes(value);
}

export function messagesFor(locale: Locale): Messages {
  return messagesByLocale[locale];
}

export function localePath(locale: Locale, path: string): string {
  return `${pathPrefixes[locale]}${path}`;
}

export interface PageInfo {
  locale: Locale;
  path: string;
}

export const PageContext = createContext<PageInfo>({ locale: 'en', path: '/' });

export function usePage(): PageInfo {
  return useContext(PageContext);
}

export function useMessages(): Messages {
  return messagesFor(usePage().locale);
}
