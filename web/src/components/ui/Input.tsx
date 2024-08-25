import type { ComponentProps } from "react";

import MuiCheckbox from "@mui/material/Checkbox";
import Collapse from "@mui/material/Collapse";
import Switch from "@mui/material/Switch";
import TextField from "@mui/material/TextField";

type Props = ComponentProps<typeof TextField>;

export const Input = (props: Props) => (
  <TextField variant="outlined" {...props} />
);

type CheckboxProps = ComponentProps<typeof MuiCheckbox>;

export const Checkbox = (props: CheckboxProps) => <MuiCheckbox {...props} />;

export { Collapse, Switch };
