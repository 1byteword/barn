// src/lib/utils.js
export function cn(...classes) {
    return classes.filter(Boolean).join(' ');
  }
  
  export function isEmpty(value) {
    return value == null || !(Object.keys(value) || value).length;
  }
  