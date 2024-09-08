import { WasmPluginDefinitions } from "pkg/apisix_admin_panel_lib";
import { memo } from "react";

import type { PluginDefinition } from "../bindings/PluginDefinition";
import type { PluginEntities } from "../bindings/PluginEntities";
import type { PluginOption } from "../bindings/PluginOption";

import { MultiField } from "./MultiField";
import { Checkbox } from "./ui/Input";
import { OpenInNewIcon, CodeIcon } from "./ui/icons/Icons";

export type PluginsState = null | Record<
  string,
  Record<string, string | string[] | undefined>
>;

type Props = {
  entity: PluginEntities;
  isEditing?: boolean;
  plugins: PluginsState | undefined;
  setPlugins: (plugins: PluginsState) => void;
};

const pluginsDefinitions = (() => {
  if (typeof window === "undefined") {
    return [];
  }

  // eslint-disable-next-line @typescript-eslint/no-unnecessary-type-assertion
  return WasmPluginDefinitions.print() as PluginDefinition[];
})();

export const parsePlugins = (
  entity: PluginEntities,
  plugins: PluginsState | undefined,
) =>
  Object.entries(plugins || {}).reduce(
    (finalPlugins, [pluginName, plugin]) => {
      if (plugin.enabled === "true") {
        const pluginDefinition = pluginsDefinitions.find(
          (p) => p.name === pluginName && p.entities.includes(entity),
        );

        if (!pluginDefinition) {
          return finalPlugins;
        }

        const newFinalPlugins = {
          ...finalPlugins,
          [pluginName]: pluginDefinition.options.reduce(
            (pluginVal, pluginDefOpt) => {
              const { name: pluginOption } = pluginDefOpt;
              const { [pluginOption]: pluginOptionValue } = plugin;

              if (pluginDefOpt.is_required !== "False" && !pluginOptionValue) {
                throw `Required field ${pluginName}.${pluginOption} is empty`;
              }

              if (!pluginOptionValue) {
                return pluginVal;
              }

              switch (pluginDefOpt.property_type) {
                case "Number": {
                  pluginVal[pluginOption] = Number(pluginOptionValue);
                  break;
                }

                case "Boolean": {
                  pluginVal[pluginOption] = pluginOptionValue === "true";
                  break;
                }

                case "String": {
                  pluginVal[pluginOption] = pluginOptionValue;
                  break;
                }

                case "JSON": {
                  pluginVal[pluginOption] = (() => {
                    try {
                      return JSON.parse(pluginOptionValue as string);
                    } catch {
                      return undefined;
                    }
                  })();

                  break;
                }

                // Unexpected case
                case "Plugins": {
                  break;
                }

                default: {
                  if (
                    typeof pluginDefOpt.property_type === "object" &&
                    "Enum" in pluginDefOpt.property_type
                  ) {
                    pluginVal[pluginOption] = pluginOptionValue;
                    break;
                  }

                  if (
                    typeof pluginDefOpt.property_type === "object" &&
                    "List" in pluginDefOpt.property_type
                  ) {
                    if (pluginDefOpt.property_type.List === "JSON") {
                      pluginVal[pluginOption] = (() => {
                        try {
                          return (pluginOptionValue as string[])
                            .map((i) => {
                              try {
                                return JSON.parse(i);
                              } catch {
                                return undefined;
                              }
                            })
                            .filter(Boolean);
                        } catch {
                          return undefined;
                        }
                      })();
                    } else {
                      pluginVal[pluginOption] = pluginOptionValue;
                    }

                    break;
                  }

                  pluginDefOpt.property_type satisfies never;
                  pluginVal[pluginOption] = pluginOptionValue;
                }
              }

              return pluginVal;
            },
            {} as Record<string, unknown>,
          ),
        };

        return newFinalPlugins;
      }

      return finalPlugins;
    },
    undefined as Record<string, unknown> | undefined,
  );

const PluginsFieldBase = ({
  entity,
  isEditing,
  plugins,
  setPlugins,
}: Props) => {
  const entityPlugins = pluginsDefinitions.filter((plugin) =>
    plugin.entities.includes(entity),
  );

  if (!entityPlugins.length) {
    return null;
  }

  const { length: enabledPlugins } = Object.values(plugins || {}).filter(
    (plugin) => plugin.enabled === "true",
  );

  return (
    <>
      <div className="px-[8px]">
        Plugins. Enabled: {enabledPlugins} / {entityPlugins.length}.
      </div>
      <div
        className="flex max-h-[250px] w-full flex-col gap-[12px] overflow-y-auto p-[8px]"
        style={{ border: "1px solid #555" }}
      >
        {entityPlugins
          .slice(0)
          .sort((a, b) => a.name.localeCompare(b.name))
          .map((plugin) => {
            const { name } = plugin;

            const existingPluginValues = plugins?.[name];

            const isEnabled = existingPluginValues?.enabled === "true";

            const getSortVal = (a: PluginOption) => {
              if (a.is_required !== "False") {
                return 0;
              }

              if (typeof a.property_type !== "object") {
                return 1;
              }

              return 2;
            };

            const checkboxId = [entity, name, "enabled"].join("-");

            return (
              <div
                className="px-[8px] py-[8px]"
                key={name}
                style={isEnabled ? { border: "1px solid #555" } : {}}
              >
                <div className="flex flex-row items-center justify-between gap-[12px]">
                  <span>{name}</span>
                  <a
                    className="text-blue-200"
                    href={`https://apisix.apache.org/docs/apisix/plugins/${name}`}
                    rel="noreferrer"
                    target="_blank"
                  >
                    <OpenInNewIcon />
                  </a>
                  <a
                    className="text-blue-200"
                    href={`https://github.com/apache/apisix/blob/master/apisix/plugins/${name}.lua`}
                    rel="noreferrer"
                    target="_blank"
                  >
                    <CodeIcon />
                  </a>
                  <div className="flex-1" />
                  <span>
                    <label className="cursor-pointer" htmlFor={checkboxId}>
                      Enable:
                    </label>{" "}
                    <Checkbox
                      checked={isEnabled}
                      id={checkboxId}
                      onChange={() => {
                        setPlugins({
                          ...(plugins || {}),
                          [name]: {
                            ...existingPluginValues,
                            enabled: String(!isEnabled),
                          },
                        } as PluginsState);
                      }}
                    />
                  </span>
                </div>
                {!!isEnabled && (
                  <div className="flex flex-col gap-[12px]">
                    {plugin.options
                      .slice(0)
                      .sort((a, b) => getSortVal(a) - getSortVal(b))
                      .map((field) => (
                        <MultiField
                          definition={field}
                          isEditing={isEditing}
                          key={field.name}
                          prefix={name}
                          setState={(optionState) => {
                            setPlugins({
                              ...(plugins || {}),
                              [name]: optionState,
                            } as PluginsState);
                          }}
                          state={existingPluginValues}
                        />
                      ))}
                  </div>
                )}
              </div>
            );
          })}
      </div>
    </>
  );
};

export const PluginsField = memo(PluginsFieldBase);
