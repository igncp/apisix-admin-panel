import { memo } from "react";
import type { EntityFields } from "src/bindings/EntityFields";
import type { PluginEntities } from "src/bindings/PluginEntities";

import { parsePlugins, PluginsField } from "./PluginsField";
import type { PluginsState } from "./PluginsField";
import Button from "./ui/Button";
import { Input } from "./ui/Input";
import { Text } from "./ui/Text";
import { DeleteIcon, IconButton } from "./ui/icons/Icons";

export type EntityFieldsItems = null | Record<
  string,
  PluginsState | string | string[] | undefined
>;

export const parseEntityFields = <
  A extends {
    fields_definitions: () => EntityFields[];
    new (): {
      set_field: (key: string, value: unknown) => void;
    };
    plugin_entity: string;
  },
>(
  entityClass: A,
  items: EntityFieldsItems | undefined,
): InstanceType<A> => {
  const fieldDefinitions = entityClass.fields_definitions();
  const entity = new entityClass() as unknown as InstanceType<A>;

  fieldDefinitions.forEach((fieldDefinition) => {
    const itemValue = items?.[fieldDefinition.name];

    if (
      fieldDefinition.property_type !== "Plugins" &&
      fieldDefinition.is_required !== "False" &&
      !fieldDefinition.hidden &&
      (itemValue === undefined || itemValue === "")
    ) {
      throw `Required field ${fieldDefinition.name} is empty`;
    }

    if (!itemValue) {
      return;
    }

    const { name: itemName } = fieldDefinition;

    switch (fieldDefinition.property_type) {
      case "Value": {
        const value = (() => {
          try {
            return JSON.parse(itemValue as string);
          } catch {
            return undefined;
          }
        })();

        entity.set_field(itemName, value);
        break;
      }

      case "Number": {
        const value = Number(itemValue);

        if (isNaN(value)) {
          throw `Invalid number value for ${itemName}`;
        }

        entity.set_field(itemName, value);
        break;
      }

      case "Boolean": {
        const value = itemValue === "true";

        entity.set_field(itemName, value);
        break;
      }

      case "String": {
        entity.set_field(itemName, itemValue);
        break;
      }

      case "Plugins": {
        if (!itemValue) {
          break;
        }

        const parsedPlugins = parsePlugins(
          entityClass.plugin_entity as PluginEntities,
          itemValue as PluginsState | undefined,
        );

        entity.set_field(itemName, parsedPlugins);
        break;
      }

      default: {
        if (
          typeof fieldDefinition.property_type === "object" &&
          "List" in fieldDefinition.property_type &&
          fieldDefinition.property_type.List === "Value"
        ) {
          if (!itemValue) {
            break;
          }

          const parsedItems = (itemValue as string[])
            .map((item) => {
              try {
                return JSON.parse(item);
              } catch {
                return undefined;
              }
            })
            .filter(Boolean);

          entity.set_field(itemName, parsedItems);
          break;
        }

        entity.set_field(itemName, itemValue);
      }
    }
  });

  return entity;
};

type Props = {
  entity: {
    fields_definitions: () => EntityFields[];
    plugin_entity: string;
  };
  items: EntityFieldsItems;
  setItems: (items: EntityFieldsItems) => void;
};

const EntityFieldBase = ({ entity, items, setItems }: Props) => {
  const fieldsDefinitions = entity.fields_definitions();

  const getSortVal = (a: EntityFields) => {
    if (a.is_required !== "False") {
      return 0;
    }

    if (typeof a.property_type !== "object" && a.property_type !== "Plugins") {
      return 1;
    }

    if (a.property_type !== "Plugins") {
      return 2;
    }

    return 3;
  };

  return (
    <div className="flex w-full flex-col gap-[12px]">
      {fieldsDefinitions
        .sort((a, b) => getSortVal(a) - getSortVal(b))
        .map((fieldDefinition) => {
          const { name } = fieldDefinition;

          if (fieldDefinition.hidden) {
            return null;
          }

          if (fieldDefinition.property_type === "Plugins") {
            return (
              <PluginsField
                entity={entity.plugin_entity as PluginEntities}
                key={name}
                plugins={items?.[name] as PluginsState | undefined}
                setPlugins={(val) => {
                  setItems({
                    ...items,
                    [name]: val,
                  });
                }}
              />
            );
          }

          const fieldValue = items?.[name];

          if (
            typeof fieldDefinition.property_type === "object" &&
            "List" in fieldDefinition.property_type
          ) {
            const parsedItems = (fieldValue as string[] | undefined) || [];
            const isJSON = fieldDefinition.property_type.List === "Value";

            return (
              <div className="flex flex-col gap-[12px]" key={name}>
                <div className="flex flex-row items-baseline gap-[12px]">
                  <Text>{name}</Text>
                  <Button
                    onClick={() => {
                      setItems({
                        ...items,
                        [name]: [...parsedItems, ""],
                      });
                    }}
                  >
                    +
                  </Button>
                </div>
                {fieldDefinition.description && (
                  <Text className="text-[#aaa]">
                    {fieldDefinition.description}
                  </Text>
                )}
                {parsedItems.map((value, index) => {
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
                    <div className="flex flex-row gap-[12px]" key={index}>
                      <Input
                        error={!isValid}
                        multiline={isJSON}
                        onChange={(e) => {
                          setItems({
                            ...items,
                            [name]: parsedItems.map((v, i) =>
                              i === index ? e.target.value : v,
                            ),
                          });
                        }}
                        placeholder={`${name} #${index + 1}${isJSON ? " (JSON)" : ""}`}
                        value={value}
                      />
                      <IconButton
                        aria-label="Clear"
                        edge="end"
                        onClick={() => {
                          let newList: string[] | undefined =
                            parsedItems.filter((_, i) => i !== index);

                          if (newList.length === 0) {
                            newList = undefined;
                          }

                          setItems({
                            ...items,
                            [name]: newList,
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

          const hasError = (() => {
            if (fieldDefinition.property_type !== "Value" || !fieldValue) {
              return false;
            }

            try {
              JSON.parse(fieldValue as string);

              return false;
            } catch {
              return true;
            }
          })();

          const fieldText =
            name +
            (fieldDefinition.is_required !== "False" ? "*" : "") +
            (fieldDefinition.property_type === "Value" ? " (JSON)" : "");

          return (
            <Input
              error={hasError}
              helperText={fieldDefinition.description || undefined}
              key={name}
              label={fieldText}
              multiline={fieldDefinition.property_type === "Value"}
              onChange={(e) => {
                setItems({
                  ...items,
                  [name]: e.target.value,
                });
              }}
              placeholder={
                (fieldDefinition.example
                  ? `Example: ${fieldDefinition.example}`
                  : "") || fieldText
              }
              type="text"
              value={fieldValue || ""}
            />
          );
        })}
    </div>
  );
};

export const EntityField = memo(EntityFieldBase);
