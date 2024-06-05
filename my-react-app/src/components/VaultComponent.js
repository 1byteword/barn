// src/components/VaultComponent.js
import React from 'react';
import { Button } from 'components/ui/button';
import { Select, SelectTrigger, SelectValue } from 'components/ui/select';

export default function VaultComponent() {
  return (
    <div className="flex h-screen bg-gray-100">
      <nav className="w-64 flex-shrink-0 bg-black p-4">
        <div className="flex items-center space-x-2 text-white mb-6">
          <VaultIcon className="h-8 w-8" />
          <span className="font-medium text-xl">Vault</span>
        </div>
        <ul className="space-y-2">
          <li>
            <Button className="w-full text-left text-white bg-gray-700">Dashboard</Button>
          </li>
          <li>
            <Button className="w-full text-left text-white">Secrets Engines</Button>
          </li>
          <li>
            <Button className="w-full text-left text-white">Access</Button>
          </li>
          <li>
            <Button className="w-full text-left text-white">Policies</Button>
          </li>
          <li>
            <Button className="w-full text-left text-white">Tools</Button>
          </li>
          <li>
            <Button className="w-full text-left text-white">Monitoring</Button>
          </li>
          <li>
            <Button className="w-full text-left text-white">Client Count</Button>
          </li>
          <li>
            <Button className="w-full text-left text-white">Seal Vault</Button>
          </li>
        </ul>
      </nav>
      <div className="flex-1 p-10">
        <h1 className="text-3xl font-bold mb-8">Vault v1.16.3</h1>
        <div className="grid grid-cols-3 gap-8">
          <div className="col-span-2">
            <div className="bg-white p-6 rounded-lg shadow mb-8">
              <div className="flex justify-between items-center mb-4">
                <h2 className="text-xl font-semibold">Secrets engines</h2>
                <Button variant="outline">Details</Button>
              </div>
              <div className="space-y-4">
                <div className="flex items-center justify-between">
                  <div>
                    <DatabaseIcon className="h-6 w-6 text-gray-600 mr-2 inline" />
                    <span className="font-medium">cubbyhole/</span>
                    <p className="text-sm text-gray-500">cubbyhole_c00102c per-token private secret storage</p>
                  </div>
                  <Button variant="outline">View</Button>
                </div>
                <div className="flex items-center justify-between">
                  <div>
                    <KeyIcon className="h-6 w-6 text-gray-600 mr-2 inline" />
                    <span className="font-medium">secret/</span>
                    <p className="text-sm text-gray-500">kv_39fa3b1e key/value secret storage</p>
                  </div>
                  <Button variant="outline">View</Button>
                </div>
              </div>
            </div>
            <div className="bg-white p-6 rounded-lg shadow">
              <h2 className="text-xl font-semibold mb-4">Learn more</h2>
              <ul className="space-y-2">
                <li>
                  <a href="#" className="flex items-center text-blue-600">
                    <BookIcon className="h-6 w-6 mr-2" />
                    Secrets Management
                  </a>
                </li>
                <li>
                  <a href="#" className="flex items-center text-blue-600">
                    <PenToolIcon className="h-6 w-6 mr-2" />
                    Monitor & Troubleshooting
                  </a>
                </li>
                <li>
                  <a href="#" className="flex items-center text-blue-600">
                    <BadgeIcon className="h-6 w-6 mr-2" />
                    Build your own Certificate Authority (CA)
                  </a>
                </li>
              </ul>
            </div>
          </div>
          <div className="space-y-8">
            <div className="bg-white p-6 rounded-lg shadow">
              <h2 className="text-xl font-semibold mb-4">Quick actions</h2>
              <p className="text-sm mb-4">Secrets engines</p>
              <p className="text-sm text-gray-500 mb-4">Supported engines include databases, KV version 2, and PKI.</p>
              <Select>
                <SelectTrigger className="mb-4">
                  <SelectValue placeholder="Type to select a mount" />
                </SelectTrigger>
              </Select>
              <div className="border-t pt-4">
                <h3 className="text-lg font-semibold mb-2">No mount selected</h3>
                <p className="text-sm text-gray-500">Select a mount above to get started.</p>
              </div>
            </div>
            <div className="bg-white p-6 rounded-lg shadow">
              <h2 className="text-xl font-semibold mb-4">Configuration details</h2>
              <div className="space-y-4">
                <div className="flex justify-between">
                  <span>API_ADDR</span>
                  <span>None</span>
                </div>
                <div className="flex justify-between">
                  <span>Default lease TTL</span>
                  <span>0</span>
                </div>
                <div className="flex justify-between">
                  <span>Max lease TTL</span>
                  <span>0</span>
                </div>
                <div className="flex justify-between">
                  <span>TLS</span>
                  <span>Disabled</span>
                </div>
                <div className="flex justify-between">
                  <span>Log format</span>
                  <span>None</span>
                </div>
                <div className="flex justify-between">
                  <span>Log level</span>
                  <span>None</span>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

function BadgeIcon(props) {
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
      <path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z" />
    </svg>
  );
}

function BookIcon(props) {
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
      <path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H20v20H6.5a2.5 2.5 0 0 1 0-5H20" />
    </svg>
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

function PenToolIcon(props) {
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
      <path d="M15.707 21.293a1 1 0 0 1-1.414 0l-1.586-1.586a1 1 0 0 1 0-1.414l5.586-5.586a1 1 0 0 1 1.414 0l1.586 1.586a1 1 0 0 1 0 1.414z" />
      <path d="m18 13-1.375-6.874a1 1 0 0 0-.746-.776L3.235 2.028a1 1 0 0 0-1.207 1.207L5.35 15.879a1 1 0 0 0 .776.746L13 18" />
      <path d="m2.3 2.3 7.286 7.286" />
      <circle cx="11" cy="11" r="2" />
    </svg>
  );
}

function VaultIcon(props) {
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
      <rect width="18" height="18" x="3" y="3" rx="2" />
      <circle cx="7.5" cy="7.5" r=".5" fill="currentColor" />
      <path d="m7.9 7.9 2.7 2.7" />
      <circle cx="16.5" cy="7.5" r=".5" fill="currentColor" />
      <path d="m13.4 10.6 2.7-2.7" />
      <circle cx="7.5" cy="16.5" r=".5" fill="currentColor" />
      <path d="m7.9 16.1 2.7-2.7" />
      <circle cx="16.5" cy="16.5" r=".5" fill="currentColor" />
      <path d="m13.4 13.4 2.7 2.7" />
      <circle cx="12" cy="12" r="2" />
    </svg>
  );
}
