import { Checkbox, type CheckboxProps, FormControlLabel } from "@mui/material";
import { type RegisterOptions, useController } from "react-hook-form";

interface IProps extends CheckboxProps {
  name: string;
  label?: React.ReactNode;
  rules?: RegisterOptions;
}

export const InputCheckbox = ({ label, name, rules, ...props }: IProps) => {
  const {
    field: { onChange, value },
  } = useController({ name, rules, defaultValue: false });
  return (
    <FormControlLabel
      control={<Checkbox {...props} />}
      label={label}
      name={name}
      onChange={onChange}
      checked={value}
    />
  );
};
