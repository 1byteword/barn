import React from 'react';
import { Button } from 'components/ui/button';
import { Input } from 'components/ui/input';

export default function VaultComponent() {
  return (
    <div className="flex h-screen bg-[#f5f5f5] dark:bg-[#1a1a1a]">
      <nav className="w-64 px-8 py-4 bg-[#333] dark:bg-[#111]">
        <div className="flex items-center justify-center h-12">
          <TreesIcon className="h-6 w-6 text-[#e6e6e6]" />
        </div>
        <div className="mt-8">
          <h2 className="text-xs font-semibold text-[#b3b3b3] uppercase tracking-wide">Barn</h2>
          <div className="mt-2 -mx-3">
            <a
              href="#"
              className="block rounded px-3 py-2 text-base font-medium text-[#e6e6e6] bg-[#444]"
            >
              Dashboard
            </a>
            <a
              href="#"
              className="mt-1 block rounded px-3 py-2 text-base font-medium text-[#b3b3b3] hover:bg-[#555] hover:text-[#e6e6e6]"
            >
              Secrets Engines
            </a>
            <a
              href="#"
              className="mt-1 block rounded px-3 py-2 text-base font-medium text-[#b3b3b3] hover:bg-[#555] hover:text-[#e6e6e6]"
            >
              Access
            </a>
            <a
              href="#"
              className="mt-1 block rounded px-3 py-2 text-base font-medium text-[#b3b3b3] hover:bg-[#555] hover:text-[#e6e6e6]"
            >
              Policies
            </a>
            <a
              href="#"
              className="mt-1 block rounded px-3 py-2 text-base font-medium text-[#b3b3b3] hover:bg-[#555] hover:text-[#e6e6e6]"
            >
              Tools
            </a>
            <a
              href="#"
              className="mt-1 block rounded px-3 py-2 text-base font-medium text-[#b3b3b3] hover:bg-[#555] hover:text-[#e6e6e6]"
            >
              Monitoring
            </a>
          </div>
        </div>
      </nav>
      <div className="flex-1 px-10 py-8">
        <div className="flex justify-between">
          <h1 className="text-2xl font-semibold text-[#333] dark:text-[#e6e6e6]">Barn v0.0.3</h1>
          <div>
            <SettingsIcon className="h-5 w-5 text-[#666] dark:text-[#b3b3b3]" />
          </div>
        </div>
        <div className="grid grid-cols-3 gap-8 mt-8">
          <div className="col-span-2">
            <div className="bg-[#fff] dark:bg-[#222] p-6 rounded-lg shadow">
              <h2 className="text-lg font-semibold text-[#333] dark:text-[#e6e6e6]">Secrets engines</h2>
              <div className="mt-4">
                <div className="flex items-center justify-between p-4 bg-[#f0f0f0] dark:bg-[#2a2a2a] rounded-lg">
                  <div className="flex items-center">
                    <DatabaseIcon className="h-6 w-6 text-[#666] dark:text-[#b3b3b3] mr-2" />
                    <div>
                      <h3 className="font-semibold text-[#333] dark:text-[#e6e6e6]">cubbyhole/</h3>
                      <p className="text-sm text-[#666] dark:text-[#b3b3b3]">
                        cubbyhole_c00102c per-token private secret storage
                      </p>
                    </div>
                  </div>

                  <Button
                    variant="outline"
                    className="ml-auto bg-[#6b705c] text-[#f5f5f5] hover:bg-[#8a9470] dark:bg-[#a7a59b] dark:text-[#1a1a1a] dark:hover:bg-[#92967e]"
                  >
                    View
                  </Button>
                </div>
                <div className="flex items-center justify-between p-4 bg-[#f0f0f0] dark:bg-[#2a2a2a] rounded-lg mt-4">
                  <div className="flex items-center">
                    <KeyIcon className="h-6 w-6 text-[#666] dark:text-[#b3b3b3] mr-2" />
                    <div>
                      <h3 className="font-semibold text-[#333] dark:text-[#e6e6e6]">secret/</h3>
                      <p className="text-sm text-[#666] dark:text-[#b3b3b3]">kv_39fa3b1e key/value secret storage</p>
                    </div>
                  </div>
                  
                  <Button
                    variant="outline"
                    className="ml-auto bg-[#6b705c] text-[#f5f5f5] hover:bg-[#8a9470] dark:bg-[#a7a59b] dark:text-[#1a1a1a] dark:hover:bg-[#92967e]"
                  >
                    View
                  </Button>
                </div>
              </div>
            </div>
            <div className="bg-[#fff] dark:bg-[#222] p-6 rounded-lg shadow mt-8">
              <h2 className="text-lg font-semibold text-[#333] dark:text-[#e6e6e6]">Learn more</h2>
              <div className="mt-4">
                <a href="#" className="block text-[#8c7853] hover:underline dark:text-[#b8a786]">
                  Secrets Management
                </a>
                <a href="#" className="block text-[#8c7853] hover:underline mt-2 dark:text-[#b8a786]">
                  Monitor & Troubleshooting
                </a>
                <a href="#" className="block text-[#8c7853] hover:underline mt-2 dark:text-[#b8a786]">
                  Build your own Certificate Authority (CA)
                </a>
              </div>
            </div>
          </div>
          <div>
            <div className="bg-[#fff] dark:bg-[#222] p-6 rounded-lg shadow">
              <h2 className="text-lg font-semibold text-[#333] dark:text-[#e6e6e6]">Quick actions</h2>
              <p className="text-sm text-[#666] dark:text-[#b3b3b3] mt-1">
                Supported engines include databases, KV version 2, and PKI.
              </p>
              <Input
                placeholder="Type to select a mount"
                className="mt-4 bg-[#d3d3c3] text-[#333] dark:bg-[#3a3a3a] dark:text-[#e6e6e6] border border-[#6b705c] dark:border-[#a7a59b]"
              />
              <div className="mt-4 p-4 bg-[#f0f0f0] dark:bg-[#2a2a2a] rounded-lg text-center">
                <p className="text-lg font-semibold text-[#333] dark:text-[#969696]">No mount selected</p>
                <p className="text-sm text-[#666] dark:text-[#b3b3b3] mt-2">Select a mount above to get started.</p>
              </div>
            </div>
            <div className="bg-[#fff] dark:bg-[#222] p-6 rounded-lg shadow mt-8">
              <h2 className="text-lg font-semibold text-[#333] dark:text-[#e6e6e6]">Configuration details</h2>
              <div className="mt-4">
                <div className="flex justify-between py-2">
                  <span className="text-sm text-[#666] dark:text-[#b3b3b3]">API_ADDR</span>
                  <span className="text-sm text-[#333] dark:text-[#e6e6e6]">None</span>
                </div>
                <div className="flex justify-between py-2">
                  <span className="text-sm text-[#666] dark:text-[#b3b3b3]">Default lease TTL</span>
                  <span className="text-sm text-[#333] dark:text-[#e6e6e6]">0</span>
                </div>
                <div className="flex justify-between py-2">
                  <span className="text-sm text-[#666] dark:text-[#b3b3b3]">Max lease TTL</span>
                  <span className="text-sm text-[#333] dark:text-[#e6e6e6]">0</span>
                </div>
                <div className="flex justify-between py-2">
                  <span className="text-sm text-[#666] dark:text-[#b3b3b3]">TLS</span>
                  <span className="text-sm text-[#333] dark:text-[#e6e6e6]">Disabled</span>
                </div>
                <div className="flex justify-between py-2">
                  <span className="text-sm text-[#666] dark:text-[#b3b3b3]">Log format</span>
                  <span className="text-sm text-[#333] dark:text-[#e6e6e6]">None</span>
                </div>
                <div className="flex justify-between py-2">
                  <span className="text-sm text-[#666] dark:text-[#b3b3b3]">Log level</span>
                  <span className="text-sm text-[#333] dark:text-[#e6e6e6]">None</span>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

function DatabaseIcon(props) {
  return (
    <svg
      {...props}
      xmlns="http://www.w3.org/2000/svg"
      width="24"
      height="24"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth="2"
      strokeLinecap="round"
      strokeLinejoin="round"
    >
      <ellipse cx="12" cy="5" rx="9" ry="3" />
      <path d="M3 5V19A9 3 0 0 0 21 19V5" />
      <path d="M3 12A9 3 0 0 0 21 12" />
    </svg>
  );
}

function KeyIcon(props) {
  return (
    <svg
      {...props}
      xmlns="http://www.w3.org/2000/svg"
      width="24"
      height="24"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth="2"
      strokeLinecap="round"
      strokeLinejoin="round"
    >
      <path d="m15.5 7.5 2.3 2.3a1 1 0 0 0 1.4 0l2.1-2.1a1 1 0 0 0 0-1.4L19 4" />
      <path d="m21 2-9.6 9.6" />
      <circle cx="7.5" cy="15.5" r="5.5" />
    </svg>
  );
}

function SettingsIcon(props) {
  return (
    <svg
      {...props}
      xmlns="http://www.w3.org/2000/svg"
      width="24"
      height="24"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth="2"
      strokeLinecap="round"
      strokeLinejoin="round"
    >
      <path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z" />
      <circle cx="12" cy="12" r="3" />
    </svg>
  );
}
  function TreesIcon(props) {
    return (
      <svg
        {...props}
        xmlns="http://www.w3.org/2000/svg"
        width="24"
        height="24"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        strokeWidth="2"
        strokeLinecap="round"
        strokeLinejoin="round"
      >
        <path d="M10 10v.2A3 3 0 0 1 8.9 16v0H5v0h0a3 3 0 0 1-1-5.8V10a3 3 0 0 1 6 0Z" />
        <path d="M7 16v6" />
        <path d="M13 19v3" />
        <path d="M12 19h8.3a1 1 0 0 0 .7-1.7L18 14h.3a1 1 0 0 0 .7-1.7L16 9h.2a1 1 0 0 0 .8-1.7L13 3l-1.4 1.5" />
      </svg>
    );
  }
