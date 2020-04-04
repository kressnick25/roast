/**
 * string-process-comma-separated
 * Extracts chunks from possibly comma or whatever-separated string
 * Version: 1.2.3
 * Author: Roy Revelt, Codsen Ltd
 * License: MIT
 * Homepage: https://gitlab.com/codsen/codsen/tree/master/packages/string-process-comma-separated
 */

(function (global, factory) {
  typeof exports === 'object' && typeof module !== 'undefined' ? module.exports = factory() :
  typeof define === 'function' && define.amd ? define(factory) :
  (global = global || self, global.stringProcessCommaSeparated = factory());
}(this, (function () { 'use strict';

  function _typeof(obj) {
    "@babel/helpers - typeof";

    if (typeof Symbol === "function" && typeof Symbol.iterator === "symbol") {
      _typeof = function (obj) {
        return typeof obj;
      };
    } else {
      _typeof = function (obj) {
        return obj && typeof Symbol === "function" && obj.constructor === Symbol && obj !== Symbol.prototype ? "symbol" : typeof obj;
      };
    }

    return _typeof(obj);
  }

  function processCommaSeparated(str, originalOpts) {
    // insurance:
    if (typeof str !== "string") {
      throw new Error("string-process-comma-separated: [THROW_ID_01] input must be string! It was given as ".concat(_typeof(str), ", equal to:\n").concat(JSON.stringify(str, null, 4)));
    } else if (!str.length || !originalOpts.cb && !originalOpts.errCb) {
      // if input str is empty or there are no callbacks, exit early
      return;
    } // opts preparation:


    var defaults = {
      from: 0,
      to: str.length,
      offset: 0,
      leadingWhitespaceOK: false,
      trailingWhitespaceOK: false,
      oneSpaceAfterCommaOK: false,
      innerWhitespaceAllowed: false,
      separator: ",",
      cb: null,
      errCb: null
    };
    var opts = Object.assign({}, defaults, originalOpts); // patch from/to values, they might have been given as nulls etc.

    if (!Number.isInteger(originalOpts.from)) {
      opts.from = 0;
    }

    if (!Number.isInteger(originalOpts.to)) {
      opts.to = str.length;
    }

    if (!Number.isInteger(originalOpts.offset)) {
      opts.offset = 0;
    } // action:


    var chunkStartsAt = null;
    var whitespaceStartsAt = null;
    var firstNonwhitespaceNonseparatorCharFound = false;
    var separatorsArr = []; // needed to catch trailing separators

    var lastNonWhitespaceCharAt = null;
    var fixable = true;

    for (var i = opts.from; i < opts.to; i++) {
      // catch the last nonwhitespace char
      if (str[i].trim().length && str[i] !== opts.separator) {
        lastNonWhitespaceCharAt = i;
      } // catch the beginning of a chunk


      if (chunkStartsAt === null && str[i].trim().length && (!opts.separator || str[i] !== opts.separator)) {
        if (!firstNonwhitespaceNonseparatorCharFound) {
          firstNonwhitespaceNonseparatorCharFound = true;
        } // if there was only one separator up to now, wipe it


        if (separatorsArr.length) {
          if (separatorsArr.length > 1) {
            separatorsArr.forEach(function (separatorsIdx, orderNumber) {
              if (orderNumber) {
                opts.errCb([[separatorsIdx + opts.offset, separatorsIdx + 1 + opts.offset]], "Remove separator.", fixable);
              }
            });
          }

          separatorsArr = [];
        }

        chunkStartsAt = i;
      } // catch the ending of a chunk


      if (Number.isInteger(chunkStartsAt) && (i > chunkStartsAt && opts.separator && str[i] === opts.separator || i + 1 === opts.to)) {
        var chunk = str.slice(chunkStartsAt, i + 1 === opts.to && str[i] !== opts.separator && str[i].trim().length ? i + 1 : i); // ping the cb

        if (typeof opts.cb === "function") {
          opts.cb(chunkStartsAt + opts.offset, (i + 1 === opts.to && str[i] !== opts.separator && str[i].trim().length ? i + 1 : lastNonWhitespaceCharAt + 1) + opts.offset);
        } // reset


        chunkStartsAt = null;
      } // catch the beginning of a whitespace


      if (!str[i].trim().length && whitespaceStartsAt === null) {
        whitespaceStartsAt = i;
      } // catch the ending of a whitespace


      if (whitespaceStartsAt !== null && (str[i].trim().length || i + 1 === opts.to)) {
        if (whitespaceStartsAt === opts.from) {
          if (!opts.leadingWhitespaceOK && typeof opts.errCb === "function") {
            opts.errCb([[whitespaceStartsAt + opts.offset, (i + 1 === opts.to ? i + 1 : i) + opts.offset]], "Remove whitespace.", fixable);
          } // else - fine

        } else if (!str[i].trim().length && i + 1 === opts.to) {
          // if it's trailing whitespace, we're on the last character
          // (right before opts.to)
          if (!opts.trailingWhitespaceOK && typeof opts.errCb === "function") {
            opts.errCb([[whitespaceStartsAt + opts.offset, i + 1 + opts.offset]], "Remove whitespace.", fixable);
          } // else - fine

        } else if ((!opts.oneSpaceAfterCommaOK || !(str[i].trim().length && i > opts.from + 1 && str[i - 1] === " " && str[i - 2] === ",")) && (!opts.innerWhitespaceAllowed || !(firstNonwhitespaceNonseparatorCharFound && str[whitespaceStartsAt - 1] && str[i].trim().length && str[i] !== opts.separator && str[whitespaceStartsAt - 1] !== opts.separator))) {
          // exclude single space after a comma, with condition that something
          // non-whitespacey follows
          // maybe opts.oneSpaceAfterCommaOK is on?
          var startingIdx = whitespaceStartsAt;
          var endingIdx = i;

          if (i + 1 === opts.to && str[i] !== opts.separator && !str[i].trim().length) {
            endingIdx++;
          } // i + 1 === opts.to && str[i] !== opts.separator && str[i].trim().length
          //   ? i + 1
          //   : i;


          var whatToAdd = "";

          if (opts.oneSpaceAfterCommaOK) {
            if (str[whitespaceStartsAt] === " " && str[whitespaceStartsAt - 1] === opts.separator) {
              // if first whitespace chunk's character is a space, leave it
              startingIdx++;
            } else if (str[whitespaceStartsAt] !== " ") {
              // if first whitespace chunk's character is not a space,
              // replace whole chunk with a space
              whatToAdd = " ";
            }
          }

          var message = "Remove whitespace."; // What if there's a space in the middle of a value, for example, URL?
          // <input accept="abc,def ghi,jkl">
          //                       ^
          //                     here.
          // We identify it by checking, is there a separator in front.

          if (!opts.innerWhitespaceAllowed && firstNonwhitespaceNonseparatorCharFound && str[whitespaceStartsAt - 1] && str[i].trim().length && str[i] !== opts.separator && str[whitespaceStartsAt - 1] !== opts.separator) {
            fixable = false;
            message = "Bad whitespace.";
          }

          if (whatToAdd.length) {
            opts.errCb([[startingIdx + opts.offset, endingIdx + opts.offset, whatToAdd]], message, fixable);
          } else {
            opts.errCb([[startingIdx + opts.offset, endingIdx + opts.offset]], message, fixable);
          } // reset fixable


          fixable = true;
        } // reset


        whitespaceStartsAt = null;
      } // catch the separator


      if (str[i] === opts.separator) {
        if (!firstNonwhitespaceNonseparatorCharFound) {
          opts.errCb([[i + opts.offset, i + 1 + opts.offset]], "Remove separator.", fixable);
        } else {
          separatorsArr.push(i);
        }
      } //                                  |
      //                                  |
      //                                  |
      //                                  |
      //                                  |
      //                                  |
      //                                  |
      //                                  |
      //                                  |
      //                            BOTTOM RULES
      //                                  |
      //                                  |
      //                                  |
      //                                  |
      //                                  |
      //                                  |
      //                                  |
      //                                  |
      //                                  |
      // catch the end of the string


      if (i + 1 === opts.to) {
        separatorsArr.forEach(function (separatorsIdx) {
          opts.errCb([[separatorsIdx + opts.offset, separatorsIdx + 1 + opts.offset]], "Remove separator.", fixable);
        });
      } // logging

    }
  }

  return processCommaSeparated;

})));