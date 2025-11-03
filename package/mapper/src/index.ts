import {DefinitionMapper, Feature} from "./definition/mapper.js";
import * as fs from "node:fs";
import * as path from "node:path";

DefinitionMapper("../../definitions").then((value: Feature[]) => {
    const functions = value.flatMap(v => v.runtimeFunctions);
    const types = value.flatMap(v => v.dataTypes);
    const flows = value.flatMap(v => v.flowTypes);

    const outDir = path.resolve("./export");
    fs.mkdirSync(outDir, {recursive: true});

    fs.writeFileSync(path.join(outDir, "functions.json"), JSON.stringify(functions));
    fs.writeFileSync(path.join(outDir, "dataTypes.json"), JSON.stringify(types));
    fs.writeFileSync(path.join(outDir, "flowTypes.json"), JSON.stringify(flows));
});
