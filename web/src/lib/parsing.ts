import type { EntityFields } from "src/bindings/EntityFields";

const knownFieldsToDelete = ["create_time", "update_time", "priority"];

export const prepareEdit = (
  entity: { text: string },
  entityClass: {
    fields_definitions: () => EntityFields[];
  },
) => {
  const parsed = JSON.parse(entity.text)?.value;
  const extraFields = { ...parsed };
  const fieldDefinitions = entityClass.fields_definitions();

  fieldDefinitions.forEach((definition) => {
    const { name: key } = definition;

    if (!parsed[key]) {
      return;
    }

    if (key === "plugins") {
      parsed.plugins = Object.entries(parsed.plugins).reduce(
        (acc, [pluginName, pluginValue]) => {
          const newPluginValue = Object.entries(
            pluginValue as Record<string, unknown>,
          ).reduce(
            (acc2, [pluginOption, pluginOptionValue]) => {
              if (typeof pluginOptionValue === "object") {
                acc2[pluginOption] = JSON.stringify(pluginOptionValue);
              } else if (typeof pluginOptionValue === "boolean") {
                acc2[pluginOption] = pluginOptionValue.toString();
              } else {
                acc2[pluginOption] = pluginOptionValue;
              }

              return acc2;
            },
            {
              enabled: "true",
            } as Record<string, unknown>,
          );

          acc[pluginName] = newPluginValue;

          if (extraFields.plugins?.[pluginName]) {
            delete extraFields.plugins[pluginName];
          }

          return acc;
        },
        {} as Record<string, Record<string, unknown>>,
      );

      if (Object.keys(extraFields.plugins).length === 0) {
        delete extraFields.plugins;
      }
    } else {
      if (typeof parsed[key] === "object") {
        parsed[key] = JSON.stringify(parsed[key]);
      } else if (typeof parsed[key] === "boolean") {
        parsed[key] = parsed[key].toString();
      }

      delete extraFields[key];
    }
  });

  knownFieldsToDelete.forEach((field) => {
    if (typeof extraFields[field] !== "undefined") {
      delete extraFields[field];
    }
  });

  if (Object.keys(extraFields).length !== 0) {
    parsed.extra = JSON.stringify(extraFields);
  }

  return parsed;
};
