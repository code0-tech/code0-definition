import * as fs from 'fs';
import * as path from 'path';

export enum MetaType {
  FlowType = 'FlowType',
  DataType = 'DataType',
  RuntimeFunction = 'RuntimeFunction',
}

export interface Meta {
  name: string;
  type: MetaType;
  data: string[];
}

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
              const meta = MetaReader(featureName, metaType, defPath);
              if (meta) result.push(meta);
            } else if (def.isDirectory()) {
              const subDefinitions = fs.readdirSync(defPath, { withFileTypes: true });

              for (const subDef of subDefinitions) {
                const subPath = path.join(defPath, subDef.name);
                if (!subDef.isFile()) continue;

                const meta = MetaReader(featureName, metaType, subPath);
                if (meta) result.push(meta);
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

const MetaReader = (name: string, type: MetaType, filePath: string): Meta | null => {
    let content: string;

    try {
      content = fs.readFileSync(filePath, 'utf-8');
    } catch (err) {
      console.error(`Error reading file: ${filePath}`, err);
      return null;
    }

    const lines = content.split('\n');
    let insideCode = false;
    const currentBlock: string[] = [];
    const codeSnippets: string[] = [];

    for (const line of lines) {
      if (line.includes('```')) {
        insideCode = !insideCode;

        if (!insideCode) {
          codeSnippets.push(currentBlock.join(' '));
          currentBlock.length = 0;
        }
        continue;
      }

      if (insideCode) {
        currentBlock.push(line);
      }
    }

    return { name, type, data: codeSnippets };
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
