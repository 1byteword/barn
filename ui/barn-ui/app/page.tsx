import Image from "next/image";
import styles from "./page.module.css";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import Link from "next/link";
// Make sure to properly import or define these icon components
import { Input } from "@/components/ui/input"
import { LayoutDashboardIcon } from "@/components/icons/layout-dashboard";


import { cn } from "@/lib/utils"

import {
  LayoutDashboardIcon,
  DatabaseIcon,
  FolderSyncIcon,
  SettingsIcon,
  MenuIcon,
  SearchIcon,
  BellIcon,
  CircleUserIcon
} from "@/components/icons";

export default function Home() {
  return (
    <main className={styles.main}>
      <div className={styles.description}>
        <p>
          Get started by editing&nbsp;
          <code className={styles.code}>app/page.tsx</code>
        </p>
        <div>
          <a
            href="https://vercel.com?utm_source=create-next-app&utm_medium=default-template&utm_campaign=create-next-app"
            target="_blank"
            rel="noopener noreferrer"
          >
            Powered by <Image src="/vercel.svg" alt="Vercel Logo" className={styles.vercelLogo} width={72} height={16} />
          </a>
        </div>
      </div>

      {/* New Component Integration */}
      <div className="flex h-screen bg-gray-100">
        <nav className="flex flex-col w-64 h-full px-4 py-8 bg-gray-900 border-r">
          <h2 className="text-xs font-semibold text-gray-500 uppercase tracking-wider">Vault</h2>
          <div className="flex flex-col justify-between flex-1 mt-6">
            <aside>
              <ul>
                <li>
                  <a href="#" className="flex items-center px-4 py-2 mt-5 text-gray-200 bg-gray-700 rounded-md">
                    <LayoutDashboardIcon className="w-5 h-5" />
                    <span className="mx-4 font-medium">Dashboard</span>
                  </a>
                </li>
                <li>
                  <a href="#" className="flex items-center px-4 py-2 mt-5 text-gray-200 rounded-md hover:bg-gray-700">
                    <DatabaseIcon className="w-5 h-5" />
                    <span className="mx-4 font-medium">Secrets Engines</span>
                  </a>
                </li>
                <li>
                  <a href="#" className="flex items-center px-4 py-2 mt-5 text-gray-200 rounded-md hover:bg-gray-700">
                    <FolderSyncIcon className="w-5 h-5" />
                    <span className="mx-4 font-medium">Secrets Sync</span>
                    <Badge variant="secondary" className="ml-auto">Enterprise</Badge>
                  </a>
                </li>
              </ul>
            </aside>
            <div className="px-4 -mx-4">
              <Button variant="ghost" className="w-full">Seal Vault</Button>
            </div>
          </div>
        </nav>
        <div className="flex flex-col flex-1 overflow-hidden">
          <header className="flex items-center justify-between p-4 border-b bg-white">
            <div className="flex items-center">
              <MenuIcon className="w-6 h-6 text-gray-500" />
              <h1 className="ml-4 text-lg font-semibold text-gray-800">Vault v1.16.3</h1>
            </div>
            <div className="flex items-center">
              <SearchIcon className="w-6 h-6 text-gray-500" />
              <BellIcon className="w-6 h-6 ml-4 text-gray-500" />
              <CircleUserIcon className="w-6 h-6 ml-4 text-gray-500" />
            </div>
          </header>
        </div>
      </div>

      {/* End of New Component Integration */}

      {/* Additional content from original Home component */}
    </main>
  );
}
