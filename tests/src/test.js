import fs from "fs-extra";
import path from "path";
import { test } from "uvu";
// eslint-disable-next-line no-unused-vars
import { equal, is, ok, throws, type, not, match } from "uvu/assert";
import { execa, execaCommand } from "execa";
import { temporaryDirectory } from "tempy";
import pMap from "p-map";

// import pack from "../package.json";
import {
  testFileContents,
  sortedTestFileContents,
  testFilePaths,
  // sortedTabbedTestFileContents,
  minifiedContents,
  prettifiedContents,
  roastExe
} from "./util/data.js";

// -----------------------------------------------------------------------------

test("01 - default sort, called on the whole folder", async () => {
  // 1. fetch us an empty, random, temporary folder:

  // Re-route the test files into `temp/` folder instead for easier access when
  // troubleshooting. Just comment out one of two:
  let tempFolder = temporaryDirectory();
  // const tempFolder = "temp";

  // The temp folder needs subfolders. Those have to be in place before we start
  // writing the files:
  fs.ensureDirSync(path.join(tempFolder, "test1"));
  fs.ensureDirSync(path.join(tempFolder, "test1/folder1"));
  fs.ensureDirSync(path.join(tempFolder, "test2"));

  // 2. asynchronously write all test files

  let processedFileContents = pMap(
    testFilePaths,
    (oneOfTestFilePaths, testIndex) =>
      fs.writeJson(
        path.join(tempFolder, oneOfTestFilePaths),
        testFileContents[testIndex]
      )
  )
    .then(() =>
      fs.writeFile(
        path.join(tempFolder, "test1/.something.yml"), //  - dotfile in yml with yml extension
        "foo:\n  bar"
      )
    )
    .then(() =>
      fs.writeFile(
        path.join(tempFolder, "test1/.somethinginyml"), // - dotfile in yml without yml extension
        "foo:\n  bar"
      )
    )
    .then(() => execa(roastExe, [tempFolder]))
    .then(() =>
      pMap(testFilePaths, (oneOfPaths) =>
        fs.readJson(path.join(tempFolder, oneOfPaths), "utf8")
      ).then((contentsArray) => {
        return pMap(contentsArray, (oneOfArrays) =>
          JSON.stringify(oneOfArrays, null, 2)
        );
      })
    )
    .then((received) =>
      // execaCommand(`rm -rf ${path.join(path.resolve(), "../temp")}`)
      execaCommand(`rm -rf ${tempFolder}`).then(() => received)
    )
    .catch((err) => {
      throw new Error(err);
    });

  equal(await processedFileContents, sortedTestFileContents, "01.01");
});

test("02 - sort, there's a broken JSON among files", async () => {
  // 1. fetch us an empty, random, temporary folder:

  // Re-route the test files into `temp/` folder instead for easier access when
  // troubleshooting. Just comment out one of two:
  let tempFolder = temporaryDirectory();
  // const tempFolder = "temp";

  // The temp folder needs subfolders. Those have to be in place before we start
  // writing the files:
  fs.ensureDirSync(path.join(tempFolder, "test1"));
  fs.ensureDirSync(path.join(tempFolder, "test1/folder1"));
  fs.ensureDirSync(path.join(tempFolder, "test2"));

  // 2. asynchronously write all test files

  let processedFileContents = pMap(
    testFilePaths,
    (oneOfTestFilePaths, testIndex) =>
      fs.writeJson(
        path.join(tempFolder, oneOfTestFilePaths),
        testFileContents[testIndex]
      )
  )
    .then(() =>
      fs.writeFile(
        path.join(tempFolder, "test1/.something.yml"), // - dotfile in yml with yml extension
        "foo:\n  bar"
      )
    )
    .then(() =>
      fs.writeFile(
        path.join(tempFolder, "test1/.somethinginyml"), // - dotfile in yml without yml extension
        "foo:\n  bar"
      )
    )
    .then(() =>
      fs.writeFile(path.join(tempFolder, "test1/broken.json"), '{a": "b"}\n')
    )
    .then(() => execa(roastExe, [tempFolder]))
    .then((receivedStdOut) => {
      match(receivedStdOut.stderr, /broken\.json/);
      return pMap(testFilePaths, (oneOfPaths) =>
        fs.readJson(path.join(tempFolder, oneOfPaths), "utf8")
      ).then((contentsArray) => {
        return pMap(contentsArray, (oneOfArrays) =>
          JSON.stringify(oneOfArrays, null, 2)
        );
      });
    })
    .then((received) =>
      // execaCommand(`rm -rf ${path.join(path.resolve(), "../temp")}`)
      execaCommand(`rm -rf ${tempFolder}`).then(() => received)
    )
    .catch((err) => {
      throw new Error(err);
    });

  equal(await processedFileContents, sortedTestFileContents, "02.01");
});

test("03 - fixes minified dotfiles in JSON format", async () => {
  let tempFolder = temporaryDirectory();
  // const tempFolder = "temp";
  fs.ensureDirSync(path.resolve(tempFolder));
  let pathOfTheTestfile = path.join(tempFolder, ".eslintrc.json");

  let processedFileContents = fs
    .writeFile(pathOfTheTestfile, minifiedContents)
    .then(() => execa(roastExe, [tempFolder, ".eslintrc.json"]))
    .then(() => fs.readFile(pathOfTheTestfile, "utf8"))
    .then((received) =>
      // execaCommand(`rm -rf ${path.join(path.resolve(), "../temp")}`)
      execaCommand(`rm -rf ${tempFolder}`).then(() => received)
    )
    .catch((err) => {
      throw new Error(err);
    });

  equal(await processedFileContents, prettifiedContents, "03.01");
});

test("04 - topmost level is array", async () => {
  let tempFolder = temporaryDirectory();
  // const tempFolder = "temp";
  fs.ensureDirSync(path.resolve(tempFolder));
  let pathOfTheTestfile = path.join(tempFolder, "sortme.json");

  let processedFileContents = fs
    .writeFile(
      pathOfTheTestfile,
      JSON.stringify(
        [
          {
            x: "y",
            a: "b",
          },
          {
            p: "r",
            c: "d",
          },
        ],
        null,
        2
      )
    )
    .then(() => execa(roastExe, [tempFolder, "sortme.json"]))
    .then(() => fs.readFile(pathOfTheTestfile, "utf8"))
    .then((received) =>
      // execaCommand(`rm -rf ${path.join(path.resolve(), "../temp")}`)
      execaCommand(`rm -rf ${tempFolder}`).then(() => received)
    )
    .catch((err) => {
      throw new Error(err);
    });

  equal(
    await processedFileContents,
    `[
\t{
\t\t"a": "b",
\t\t"x": "y"
\t},
\t{
\t\t"c": "d",
\t\t"p": "r"
\t}
]\n`,
    "04.01"
  );
});

test("05 - no files found in the given directory", async () => {
  // fetch us a random temp folder
  let tempFolder = temporaryDirectory();
  // const tempFolder = "temp";

  // call execa on that empty folder
  let stdOutContents = await execa(roastExe, [tempFolder]);
  // CLI will complain no files could be found
  match(
    stdOutContents.stderr,
    /The inputs don't lead to any json files! Exiting./,
    "05.01"
  );
});

test.run();
