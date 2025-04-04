import fs from "fs-extra";
import path from "path";
import { test } from "uvu";
// eslint-disable-next-line no-unused-vars
import { equal, is, ok, throws, type, not, match } from "uvu/assert";
import { execa } from "execa";
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

test("01 - help output mode", async () => {
  let reportedVersion1 = await execa(roastExe, ["-h"]);
  match(reportedVersion1.stdout, /Usage/, "01.01");
  match(reportedVersion1.stdout, /Arguments/, "01.02");
  match(reportedVersion1.stdout, /Options/, "01.03");

  let reportedVersion2 = await execa(roastExe, ["--help"]);
  match(reportedVersion2.stdout, /Usage/, "01.04");
  match(reportedVersion2.stdout, /Arguments/, "01.05");
  match(reportedVersion2.stdout, /Options/, "01.06");
});

test("02 - help flag trumps silent flag", async () => {
  let unsortedFile = '{\n  "z": 1,\n  "a": 2\n}\n';

  let tempFolder = temporaryDirectory();
  // const tempFolder = "temp";
  fs.ensureDirSync(path.resolve(tempFolder));
  fs.writeFileSync(path.join(tempFolder, "sortme.json"), unsortedFile);

  let output = await execa(roastExe, [tempFolder, "-h", "--silent"]).catch(
    (err) => {
      throw new Error(err);
    }
  );

  match(output.stdout, /Usage/, "02.01");
  match(output.stdout, /Arguments/, "02.02");
  equal(output.exitCode, 0, "02.03");
  equal(
    fs.readFileSync(path.join(tempFolder, "sortme.json"), "utf8"),
    unsortedFile,
    "02.04"
  );
});

test.run();
