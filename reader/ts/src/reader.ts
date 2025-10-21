import * as fs from 'fs';
import * as path from 'path';
import {Meta, MetaType} from "./types.ts";

export const Reader = (rootPath: string): Meta[] => {
    const result: Meta[] = [];

    try {
      const features = fs.readdirSync(rootPath, { withFileTypes: true });

      for (const featureDirent of features) {
        if (!featureDirent.isDirectory()) continue;

        const featurePath = path.join(rootPath, featureDirent.name);
        const featureName = featureDirent.name;

        const typeDirs = fs.readdirSync(featurePath, { withFileTypes: true });

        for (const typeDirent of typeDirs) {
          if (!typeDirent.isDirectory()) continue;

          const metaType = matchMetaType(typeDirent.name);
          if (!metaType) continue;

          const typePath = path.join(featurePath, typeDirent.name);
          const definitions = fs.readdirSync(typePath, { withFileTypes: true });

          for (const def of definitions) {
            const defPath = path.join(typePath, def.name);

            if (def.isFile()) {
                if (!defPath.endsWith('.json')) continue;
                try {
                    const content = fs.readFileSync(defPath, 'utf-8');
                    const meta: Meta = {name:  featureName, type: metaType, data: content};
                    result.push(meta);
                } catch (err) {
                    console.error(`Error reading file: ${defPath}`, err);
                }
            } else if (def.isDirectory()) {
                const subDefinitions = fs.readdirSync(defPath, { withFileTypes: true });

              for (const subDef of subDefinitions) {
                const subPath = path.join(defPath, subDef.name);

                  if (!subPath.endsWith('.json')) continue;
                  if (!subDef.isFile()) continue;

                  try {
                      const content = fs.readFileSync(subPath, 'utf-8');
                      const meta: Meta = {name:  featureName, type: metaType, data: content};
                      result.push(meta);
                  } catch (err) {
                      console.error(`Error reading file: ${subPath}`, err);
                  }
              }
            }
          }
        }
      }

      return result
    } catch (err) {
      console.error(`Error reading path ${rootPath}:`, err);
      return [];
    }
}

function matchMetaType(name: string): MetaType | null {
  switch (name) {
    case 'flow_type':
      return MetaType.FlowType;
    case 'data_type':
      return MetaType.DataType;
    case 'runtime_definition':
      return MetaType.RuntimeFunction;
    default:
      return null;
  }
}
