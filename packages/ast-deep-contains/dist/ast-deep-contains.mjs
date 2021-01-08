/**
 * ast-deep-contains
 * Like t.same assert on array of objects, where element order doesn't matter.
 * Version: 2.0.1
 * Author: Roy Revelt, Codsen Ltd
 * License: MIT
 * Homepage: https://codsen.com/os/ast-deep-contains/
 */

import t from"object-path";import{traverse as r}from"ast-monkey-traverse";import e from"@sindresorhus/is";var n="2.0.1";function a(t,r){return Array.from(t).filter(((t,e)=>e!==r))}const i={skipContainers:!0,arrayStrictComparison:!1};function s(n,o,l,c,f){const p={...i,...f};e(n)!==e(o)?c(`the first input arg is of a type ${e(n).toLowerCase()} but the second is ${e(o).toLowerCase()}. Values are - 1st:\n${JSON.stringify(n,null,4)}\n2nd:\n${JSON.stringify(o,null,4)}`):r(o,((r,i,o,f)=>{const h=void 0!==i?i:r,{path:u}=o;if(t.has(n,u))if(!p.arrayStrictComparison&&e.plainObject(h)&&"array"===o.parentType&&o.parent.length>1){f.now=!0;const r=Array.from(o.path.includes(".")?t.get(n,function(t){if(t.includes("."))for(let r=t.length;r--;)if("."===t[r])return t.slice(0,r);return t}(u)):n);if(o.parent.length>r.length)c(`the first array: ${JSON.stringify(r,null,4)}\nhas less objects than array we're matching against, ${JSON.stringify(o.parent,null,4)}`);else{const t=o.parent,n=r.map(((t,r)=>r)),i=(t.map(((t,r)=>r)),[]);for(let t=0,r=n.length;r>t;t++){const r=[],e=n[t],s=a(n,t);r.push(e),s.forEach((t=>{i.push(Array.from(r).concat(t))}))}const f=i.map((t=>t.map(((t,r)=>[r,t]))));let h=0;for(let n=0,a=f.length;a>n;n++){let a=0;f[n].forEach((n=>{e.plainObject(t[n[0]])&&e.plainObject(r[n[1]])&&Object.keys(t[n[0]]).forEach((e=>{Object.keys(r[n[1]]).includes(e)&&(a+=1,r[n[1]][e]===t[n[0]][e]&&(a+=5))}))})),f[n].push(a),a>h&&(h=a)}for(let e=0,n=f.length;n>e;e++)if(f[e][2]===h){f[e].forEach(((n,a)=>{f[e].length-1>a&&s(r[n[1]],t[n[0]],l,c,p)}));break}}}else{const r=t.get(n,u);p.skipContainers&&(e.plainObject(r)||Array.isArray(r))||l(r,h,u)}else c(`the first input: ${JSON.stringify(n,null,4)}\ndoes not have the path "${u}", we were looking, would it contain a value ${JSON.stringify(h,null,0)}.`);return h}))}export{s as deepContains,i as defaults,n as version};