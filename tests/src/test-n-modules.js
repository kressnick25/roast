import fs from "fs-extra";
import path from "path";
import { test } from "uvu";
// eslint-disable-next-line no-unused-vars
import { equal, is, ok, throws, type, not, match } from "uvu/assert";
import { execa, execaCommand } from "execa";
import { temporaryDirectory } from "tempy";
// import pMap from "p-map";
// import pack from "../package.json";
// import {
//   testFileContents,
//   sortedTestFileContents,
//   testFilePaths,
//   sortedTabbedTestFileContents,
//   minifiedContents,
//   prettifiedContents,
// } from "./util/data.js";
import { roastExe } from "./util/data.js";

// -----------------------------------------------------------------------------

test("01 - only node_modules with one file, flag disabled", async () => {
  let tempFolder = temporaryDirectory();
  // const tempFolder = "temp";
  fs.ensureDirSync(path.resolve(tempFolder));
  fs.ensureDirSync(path.resolve(path.join(tempFolder, "/node_modules/")));
  let pathOfTheTestfile = path.join(tempFolder, "/node_modules/sortme.json");
  let originalContents = '{\n  "z": 1,\n  "a": 2\n}\n';

  let processedFilesContents = fs
    .writeFile(pathOfTheTestfile, originalContents)
    .then(() => execa(roastExe, ["-s", tempFolder]))
    .then(() => fs.readFile(pathOfTheTestfile, "utf8"))
    .then((testFile) =>
      execaCommand(`rm -rf ${tempFolder}`)
        .then(() => testFile)
        .catch((err) => {
          throw new Error(err);
        })
    )
    .catch((err) => {
      throw new Error(err);
    });

  equal(await processedFilesContents, originalContents, "01.01");
});

test("02- files inside and outside node_modules, flag disabled", async () => {
  let originalContents = '{\n  "z": 1,\n  "a": 2\n}\n';
  let sortedContents = '{\n  "a": 2,\n  "z": 1\n}\n';

  let tempFolder = temporaryDirectory();
  // const tempFolder = "temp";
  fs.ensureDirSync(path.resolve(tempFolder));
  fs.ensureDirSync(path.resolve(path.join(tempFolder, "/node_modules/dir1/")));
  let pathOfTestFile1 = path.join(tempFolder, "/node_modules/dir1/sortme.json");
  let pathOfTestFile2 = path.join(tempFolder, "sortme.json");

  fs.writeFileSync(pathOfTestFile1, originalContents);
  fs.writeFileSync(pathOfTestFile2, originalContents);

  await execa(roastExe, ["-s", tempFolder]).catch((err) => {
    throw new Error(err);
  });

  equal(fs.readFileSync(pathOfTestFile1, "utf8"), originalContents, "02.01");
  equal(fs.readFileSync(pathOfTestFile2, "utf8"), sortedContents, "02.02");

  await execaCommand(`rm -rf ${tempFolder}`).catch((err) => {
    throw new Error(err);
  });
});

test.run();
