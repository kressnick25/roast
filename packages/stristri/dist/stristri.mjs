/**
 * stristri
 * Extracts or deletes HTML, CSS, text and/or templating tags from string
 * Version: 3.0.0
 * Author: Roy Revelt, Codsen Ltd
 * License: MIT
 * Homepage: https://codsen.com/os/stristri/
 */

import{tokenizer as t}from"codsen-tokenizer";import{collapse as e}from"string-collapse-white-space";import{rApply as s}from"ranges-apply";import{detectLang as r}from"detect-templating-language";const n={html:!0,css:!0,text:!1,templatingTags:!1,reportProgressFunc:null,reportProgressFuncFrom:0,reportProgressFuncTo:100};const o="3.0.0";function i(t,e,s,r){if(4!==arguments.length)throw new Error(`stristri/returnHelper(): should be 3 input args but ${arguments.length} were given!`);if("string"!=typeof t)throw new Error("stristri/returnHelper(): first arg missing!");if("object"!=typeof e)throw new Error("stristri/returnHelper(): second arg missing!");return{log:{timeTakenInMilliseconds:Date.now()-r},result:t,applicableOpts:e,templatingLang:s}}function a(o,a){const g=Date.now();if("string"!=typeof o)throw new Error(`stristri: [THROW_ID_01] the first input arg must be string! It was given as ${JSON.stringify(o,null,4)} (${typeof o})`);if(a&&"object"!=typeof a)throw new Error(`stristri: [THROW_ID_02] the second input arg must be a plain object! It was given as ${JSON.stringify(a,null,4)} (${typeof a})`);const l={...n,...a},p={html:!1,css:!1,text:!1,templatingTags:!1};o||i("",p,r(o),g);const c=[];let m=!1,u=!1,h=!1,f=!1;return t(o,{tagCb:t=>{"comment"===t.type?h?(p.css||(p.css=!0),l.css&&c.push([t.start,t.end," "])):(p.html||(p.html=!0),t.closing||u||m?t.closing&&m&&(m=!1):m=!0,l.html&&c.push([t.start,t.end," "])):"tag"===t.type?(p.html||(p.html=!0),l.html&&c.push([t.start,t.end," "]),"style"!==t.tagName||t.closing?h&&"style"===t.tagName&&t.closing&&(h=!1):h=!0,"xml"===t.tagName&&(t.closing||u||m?t.closing&&u&&(u=!1):u=!0),"script"!==t.tagName||t.closing?f&&"script"===t.tagName&&t.closing&&(f=!1):f=!0):["at","rule"].includes(t.type)?(p.css||(p.css=!0),l.css&&c.push([t.start,t.end," "])):"text"===t.type?(h||m||u||f||p.text||!t.value.trim()||(p.text=!0),(h&&l.css||(m||f)&&l.html||!h&&!m&&!u&&!f&&l.text)&&(t.value.includes("\n")?c.push([t.start,t.end,"\n"]):c.push([t.start,t.end," "]))):"esp"===t.type&&(p.templatingTags||(p.templatingTags=!0),l.templatingTags&&c.push([t.start,t.end," "]))},reportProgressFunc:l.reportProgressFunc,reportProgressFuncFrom:l.reportProgressFuncFrom,reportProgressFuncTo:.95*l.reportProgressFuncTo}),i(e(s(o,c),{trimLines:!0,removeEmptyLines:!0,limitConsecutiveEmptyLinesTo:1}).result,p,r(o),g)}export{n as defaults,a as stri,o as version};