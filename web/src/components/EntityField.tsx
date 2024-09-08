import { memo } from "react";
import type { EntityFields } from "src/bindings/EntityFields";
import type { PluginEntities } from "src/bindings/PluginEntities";

import { MultiField } from "./MultiField";
import { parsePlugins, PluginsField } from "./PluginsField";
import type { PluginsState } from "./PluginsField";

export type EntityFieldsItems = null | Record<
  string,
  PluginsState | string | string[] | undefined
>;

const extraField: EntityFields = {
  default_value: null,
  description:
    "You can add any extra fields here which will be merged with the entity",
  example: '{ "foo": "bar" }',
  hidden: false,
  is_editable: true,
  is_required: "False",
  name: "extra",
  property_type: "JSON",
};

export const parseEntityFields = <
  A extends {
    fields_definitions: () => EntityFields[];
    new (): {
      add_extra_json: (value: unknown) => void;
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
      case "JSON": {
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
          fieldDefinition.property_type.List === "JSON"
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

  const extraFields = (() => {
    if (!items?.extra) {
      return {};
    }

    try {
      return JSON.parse(items.extra as string);
    } catch {
      return {};
    }
  })();

  entity.add_extra_json(extraFields);

  return entity;
};

type Props = {
  entity: {
    fields_definitions: () => EntityFields[];
    plugin_entity: string;
  };
  isEditing: boolean;
  items: EntityFieldsItems;
  setItems: (items: EntityFieldsItems) => void;
};

const EntityFieldBase = ({ entity, isEditing, items, setItems }: Props) => {
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
        .concat(extraField)
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

          return (
            <MultiField
              definition={fieldDefinition}
              isEditing={isEditing}
              key={name}
              prefix={name}
              setState={setItems}
              state={items as Record<string, string | string[] | undefined>}
            />
          );
        })}
    </div>
  );
};

export const EntityField = memo(EntityFieldBase);
