import React from 'react'
import { MenuItemType, SubMenuType, MenuItemGroupType } from 'antd/es/menu/hooks/useItems'
import Link from 'next/link'

type MenuItem = MenuItemType | SubMenuType | MenuItemGroupType

function getItem(
  label: React.ReactNode,
  key: React.Key,
  icon?: React.ReactNode,
  children?: MenuItem[]
): MenuItem {
  return {
    key,
    icon,
    children,
    label
  } as MenuItem
}

export const navLandingPage = [
  getItem(<Link href="/">Beranda</Link>, '/', null),
  getItem(<Link href="/program">Program</Link>, '/program', null),
  getItem(<Link href="/download">Unduh</Link>, '/download', null),
  getItem(<Link href="/about">Tentang</Link>, '/about', null)
]

export const navHomePage = [
  getItem(<Link href={'/dashboard'}>Beranda</Link>, '/dashboard', null),
  getItem(<Link href={'/dashboard/account'}>Akun</Link>, '/dashboard/account', null),
  getItem(<Link href={'/dashboard/classroom'}>Kelas</Link>, '/dashboard/classroom', null),
  getItem(
    <Link href={'/dashboard/question-bank'}>Bank Soal</Link>,
    '/dashboard/question-bank',
    null
  ),
  getItem(
    <Link href={'/dashboard/study-results'}>Hasil Studi</Link>,
    '/dashboard/study-results',
    null
  )
]
