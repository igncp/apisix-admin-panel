import { useRef } from "react";
import type { PropertyType } from "src/bindings/PropertyType";
import type { Required } from "src/bindings/Required";

import FormControl from "@mui/material/FormControl";
import InputLabel from "@mui/material/InputLabel";
import MenuItem from "@mui/material/MenuItem";
import Select from "@mui/material/Select";

import { JSONField } from "./JSONField";
import Button from "./ui/Button";
import { Checkbox, Input } from "./ui/Input";
import { Text } from "./ui/Text";
import { DeleteIcon, IconButton } from "./ui/icons/Icons";

type State = null | Record<string, string | string[] | undefined>;

type Definition = {
  default_value: null | string;
  description?: string;
  example?: null | string;
  is_editable?: boolean;
  is_required: Required;
  name: string;
  property_type: PropertyType;
};

type Props = {
  definition: Definition;
  isEditing?: boolean;
  prefix: string;
  setState: (state: State) => void;
  state: State;
};

export const MultiField = ({
  definition,
  isEditing,
  prefix,
  setState,
  state,
}: Props) => {
  const {
    default_value: defaultValue,
    description,
    example,
    is_required: isRequired,
    name,
    property_type: propertyType,
  } = definition;

  const existingOptionValue = state?.[name] || "";
  const setContentWrap = useRef<{ fn: (v: string) => void }>({ fn: () => {} });

  if (typeof propertyType === "object" && "List" in propertyType) {
    const parsedItems = (() => {
      if (typeof existingOptionValue === "string") {
        try {
          return JSON.parse(existingOptionValue).map((v: unknown) =>
            typeof v === "string" ? v : JSON.stringify(v),
          );
        } catch {
          return [];
        }
      }

      return (existingOptionValue as string[] | undefined) || [];
    })() as string[];

    const isJSON = propertyType.List === "JSON";

    return (
      <div className="flex flex-col gap-[12px]">
        <div className="flex flex-row items-baseline gap-[12px]">
          <Text>{name}</Text>
          <Button
            onClick={() => {
              setState({
                ...state,
                [name]: [...parsedItems, ""],
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
            <div className="flex flex-row gap-[12px]" key={index}>
              <Input
                error={!isValid}
                label={fieldLabel}
                multiline={isJSON}
                onChange={(e) => {
                  setState({
                    ...state,
                    [name]: parsedItems.map((v, i) =>
                      i === index ? e.target.value : v,
                    ),
                  });
                }}
                placeholder={fieldLabel}
                value={value}
              />
              <IconButton
                aria-label="Clear"
                edge="end"
                onClick={() => {
                  let newList: string[] | undefined = parsedItems.filter(
                    (_, i) => i !== index,
                  );

                  if (newList.length === 0) {
                    newList = undefined;
                  }

                  setState({
                    ...state,
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

  if (typeof propertyType === "object" && "Enum" in propertyType) {
    const fieldId = [prefix, name].join("-");
    const labelId = `${fieldId}-label`;

    return (
      <div className="flex flex-col gap-[4px]">
        <div className="items-between flex flex-row gap-[12px]">
          <FormControl className="flex-1">
            <InputLabel
              className="pl-[8px]"
              htmlFor={fieldId}
              id={labelId}
              variant="standard"
            >
              {name}
            </InputLabel>
            <Select
              disabled={isEditing && !definition.is_editable}
              id={fieldId}
              labelId={labelId}
              onChange={(e) => {
                setState({
                  ...state,
                  [name]: e.target.value,
                });
              }}
              value={existingOptionValue}
            >
              {propertyType.Enum.map((option) => (
                <MenuItem key={option} value={option}>
                  {option}
                </MenuItem>
              ))}
            </Select>
          </FormControl>
          {definition.is_required === "False" && (
            <IconButton
              aria-label="Clear"
              edge="end"
              onClick={(e) => {
                e.stopPropagation();

                if (!state?.[name]) {
                  return;
                }

                const newState = { ...state };

                delete newState[name];

                setState(newState);
              }}
            >
              <DeleteIcon />
            </IconButton>
          )}
        </div>
        {description && (
          <Text className="ml-[16px] text-[12px] text-[rgba(255,255,255,0.7)]">
            {description}
          </Text>
        )}
      </div>
    );
  }

  if (propertyType === "Boolean") {
    const fieldId = [prefix, name, "enabled"].join("-");

    return (
      <div className="flex flex-col gap-[4px] pl-[8px]">
        <div>
          <label className="cursor-pointer" htmlFor={fieldId}>
            {name}
          </label>
          <Checkbox
            checked={existingOptionValue === "true"}
            disabled={isEditing && !definition.is_editable}
            id={fieldId}
            onChange={() => {
              setState({
                ...state,
                [name]: String(existingOptionValue !== "true"),
              });
            }}
          />
        </div>
        {description && (
          <Text className="mt-[-15px] pl-[4px] text-[12px] text-[#aaa]">
            {description}
          </Text>
        )}
      </div>
    );
  }

  const isJSON = propertyType === "JSON";

  if (propertyType === "JSON") {
    setContentWrap.current.fn = (val: string) => {
      setState({
        ...state,
        [name]: val,
      });
    };

    return (
      <JSONField
        content={existingOptionValue as string}
        key={name}
        name={name}
        setContentWrap={setContentWrap.current}
      />
    );
  }

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
      disabled={isEditing && !definition.is_editable}
      error={!isValid}
      helperText={description}
      key={name}
      label={name + (isJSON ? " (JSON)" : "")}
      multiline={isJSON}
      onChange={(e) => {
        setState({
          ...state,
          [name]: e.target.value,
        });
      }}
      placeholder={
        isRequired !== "False" ? "* Required" : example || defaultValue || ""
      }
      type="text"
      value={existingOptionValue}
    />
  );
};
