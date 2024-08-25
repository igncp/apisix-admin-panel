import { WasmPluginDefinitions } from "pkg/apisix_admin_panel_lib";
import { memo } from "react";

import FormControl from "@mui/material/FormControl";
import InputLabel from "@mui/material/InputLabel";
import MenuItem from "@mui/material/MenuItem";
import Select from "@mui/material/Select";

import type { PluginDefinition } from "../bindings/PluginDefinition";
import type { PluginEntities } from "../bindings/PluginEntities";
import type { PluginOption } from "../bindings/PluginOption";

import Button from "./ui/Button";
import { Checkbox, Input } from "./ui/Input";
import { Text } from "./ui/Text";
import { DeleteIcon, IconButton, OpenInNewIcon } from "./ui/icons/Icons";

export type PluginsState = null | Record<
  string,
  Record<string, string | string[] | undefined>
>;

type Props = {
  entity: PluginEntities;
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

                case "Value": {
                  pluginVal[pluginOption] = (() => {
                    try {
                      return JSON.parse(pluginOptionValue as string);
                    } catch {
                      return undefined;
                    }
                  })();

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
                    if (pluginDefOpt.property_type.List === "Value") {
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

const PluginsFieldBase = ({ entity, plugins, setPlugins }: Props) => {
  const entityPlugins = pluginsDefinitions.filter((plugin) =>
    plugin.entities.includes(entity),
  );

  if (!entityPlugins.length) {
    return null;
  }

  return (
    <div
      className="flex max-h-[250px] w-full flex-col gap-[12px] overflow-y-auto p-[8px]"
      style={{ border: "1px solid #555" }}
    >
      <div className="px-[8px]">Plugins</div>
      {entityPlugins
        .slice(0)
        .sort((a, b) => a.name.localeCompare(b.name))
        .map((plugin) => {
          const { name } = plugin;

          const existingPluginValues = plugins?.[name];

          const isEnabled = existingPluginValues?.enabled === "true";

          const getSortVal = (a: PluginOption) => {
            if (a.is_required) {
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
                    .map((field) => {
                      const existingOptionValue =
                        existingPluginValues[field.name] || "";

                      if (
                        typeof field.property_type === "object" &&
                        "List" in field.property_type
                      ) {
                        const parsedItems =
                          (existingOptionValue as string[] | undefined) || [];

                        const isJSON = field.property_type.List === "Value";

                        return (
                          <div
                            className="flex flex-col gap-[12px]"
                            key={field.name}
                          >
                            <div className="flex flex-row items-baseline gap-[12px]">
                              <Text>{field.name}</Text>
                              <Button
                                onClick={() => {
                                  setPlugins({
                                    ...plugins,
                                    [name]: {
                                      ...existingPluginValues,
                                      [field.name]: [...parsedItems, ""],
                                    },
                                  });
                                }}
                              >
                                +
                              </Button>
                            </div>
                            {parsedItems.map((value, index) => {
                              const fieldLabel = `#${index + 1}${isJSON ? " (JSON)" : ""}`;

                              const isValid = (() => {
                                if (!isJSON || !value) {
                                  return true;
                                }

                                try {
                                  JSON.parse(value);

                                  return true;
                                } catch {
                                  return false;
                                }
                              })();

                              return (
                                <div
                                  className="flex flex-row gap-[12px]"
                                  key={index}
                                >
                                  <Input
                                    error={!isValid}
                                    label={fieldLabel}
                                    multiline={isJSON}
                                    onChange={(e) => {
                                      setPlugins({
                                        ...plugins,
                                        [name]: {
                                          ...existingPluginValues,
                                          [field.name]: parsedItems.map(
                                            (v, i) =>
                                              i === index ? e.target.value : v,
                                          ),
                                        },
                                      });
                                    }}
                                    placeholder={fieldLabel}
                                    value={value}
                                  />
                                  <IconButton
                                    aria-label="Clear"
                                    edge="end"
                                    onClick={() => {
                                      let newList: string[] | undefined =
                                        parsedItems.filter(
                                          (_, i) => i !== index,
                                        );

                                      if (newList.length === 0) {
                                        newList = undefined;
                                      }

                                      setPlugins({
                                        ...plugins,
                                        [name]: {
                                          ...existingPluginValues,
                                          [field.name]: newList,
                                        },
                                      });
                                    }}
                                  >
                                    <DeleteIcon />
                                  </IconButton>
                                </div>
                              );
                            })}
                          </div>
                        );
                      }

                      if (
                        typeof field.property_type === "object" &&
                        "Enum" in field.property_type
                      ) {
                        const fieldId = [entity, name, field.name].join("-");
                        const labelId = `${fieldId}-label`;

                        return (
                          <div
                            className="flex flex-col gap-[4px]"
                            key={field.name}
                          >
                            <FormControl>
                              <InputLabel
                                className="pl-[8px]"
                                htmlFor={fieldId}
                                id={labelId}
                                variant="standard"
                              >
                                {field.name}
                              </InputLabel>
                              <Select
                                id={fieldId}
                                labelId={labelId}
                                onChange={(e) => {
                                  setPlugins({
                                    ...(plugins || {}),
                                    [name]: {
                                      ...existingPluginValues,
                                      [field.name]: e.target.value,
                                    },
                                  });
                                }}
                                value={existingOptionValue}
                              >
                                {field.property_type.Enum.map((option) => (
                                  <MenuItem key={option} value={option}>
                                    {option}
                                  </MenuItem>
                                ))}
                              </Select>
                            </FormControl>
                            {field.description && (
                              <Text className="pl-[4px] text-[#aaa]">
                                {field.description}
                              </Text>
                            )}
                          </div>
                        );
                      }

                      if (field.property_type === "Boolean") {
                        const fieldId = [
                          entity,
                          name,
                          field.name,
                          "enabled",
                        ].join("-");

                        return (
                          <div
                            className="flex flex-col gap-[12px] pl-[8px]"
                            key={field.name}
                          >
                            <div>
                              <label
                                className="cursor-pointer"
                                htmlFor={fieldId}
                              >
                                {field.name}
                              </label>
                              <Checkbox
                                checked={existingOptionValue === "true"}
                                id={fieldId}
                                onChange={() => {
                                  setPlugins({
                                    ...(plugins || {}),
                                    [name]: {
                                      ...existingPluginValues,
                                      [field.name]: String(
                                        existingOptionValue !== "true",
                                      ),
                                    },
                                  });
                                }}
                              />
                            </div>
                            {field.description && (
                              <Text className="pl-[4px] text-[#aaa]">
                                {field.description}
                              </Text>
                            )}
                          </div>
                        );
                      }

                      const isJSON = field.property_type === "Value";

                      const isValid = (() => {
                        if (!!existingOptionValue && isJSON) {
                          try {
                            JSON.parse(existingOptionValue as string);

                            return true;
                          } catch {
                            return false;
                          }
                        }

                        return true;
                      })();

                      return (
                        <Input
                          error={!isValid}
                          helperText={field.description}
                          key={field.name}
                          label={field.name + (isJSON ? " (JSON)" : "")}
                          multiline={isJSON}
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
              )}
            </div>
          );
        })}
    </div>
  );
};

export const PluginsField = memo(PluginsFieldBase);
