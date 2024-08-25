import { WasmPluginDefinitions } from "pkg/apisix_admin_panel_lib";
import { memo } from "react";
import type { PluginDefinition } from "src/bindings/PluginDefinition";
import type { PluginEntities } from "src/bindings/PluginEntities";

import OpenInNew from "@mui/icons-material/OpenInNew";

import { Checkbox, Input } from "./ui/Input";

export type PluginsState = null | Record<string, Record<string, string>>;

type Props = {
  entity: PluginEntities;
  plugins: unknown;
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
      if (plugin.enabled) {
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

              if (pluginDefOpt.is_required && !pluginOptionValue) {
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

                default: {
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

const PluginsFieldBase = ({ entity, plugins, setPlugins }: Props) => {
  const entityPlugins = pluginsDefinitions.filter((plugin) =>
    plugin.entities.includes(entity),
  );

  if (!entityPlugins.length) {
    return null;
  }

  return (
    <div className="p-[8px]" style={{ border: "1px solid #555" }}>
      <div>Plugins</div>
      {entityPlugins.map((plugin) => {
        const { name } = plugin;

        const existingPluginValues =
          (plugins?.[name as keyof typeof plugins] as
            | Record<string, string>
            | undefined) || {};

        const isEnabled = !!existingPluginValues.enabled;

        return (
          <div className="border-[1px] border-[#333]" key={name}>
            <div className="flex flex-row items-center justify-between gap-[12px]">
              <span>{name}</span>
              <a
                className="text-blue-200"
                href={`https://apisix.apache.org/docs/apisix/plugins/${name}`}
                rel="noreferrer"
                target="_blank"
              >
                <OpenInNew />
              </a>
              <span>
                Enable:{" "}
                <Checkbox
                  checked={isEnabled}
                  onChange={() => {
                    setPlugins({
                      ...(plugins || {}),
                      [name]: {
                        ...existingPluginValues,
                        enabled: !isEnabled,
                      },
                    });
                  }}
                />
              </span>
            </div>
            <div className="flex flex-col gap-[12px]">
              {!!isEnabled &&
                plugin.options.map((field) => {
                  const existingOptionValue =
                    existingPluginValues[field.name] || "";

                  return (
                    <Input
                      key={field.name}
                      label={field.name}
                      onChange={(e) => {
                        setPlugins({
                          ...(plugins || {}),
                          [name]: {
                            ...existingPluginValues,
                            [field.name]: e.target.value,
                          },
                        });
                      }}
                      placeholder={
                        field.is_required
                          ? `* Required (${field.property_type})`
                          : field.default_value || ""
                      }
                      type="text"
                      value={existingOptionValue}
                    />
                  );
                })}
            </div>
          </div>
        );
      })}
    </div>
  );
};

export const PluginsField = memo(PluginsFieldBase);
