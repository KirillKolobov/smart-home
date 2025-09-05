import {
  type BaseTextFieldProps,
  InputAdornment,
  TextField,
  Typography,
} from "@mui/material";
import classes from "./styles.module.scss";
import { type RegisterOptions, useController } from "react-hook-form";
import type { ReactNode } from "react";

interface IProps extends BaseTextFieldProps {
  name: string;
  label?: string;
  rules?: RegisterOptions;
  multiline?: boolean;
  rows?: number;
  testId?: string;
  adornmentImg?: ReactNode;
}

export const InputText = ({
  placeholder,
  label,
  name,
  rules,
  multiline,
  rows,
  adornmentImg,
  testId,
  ...props
}: IProps) => {
  const {
    field: { onChange, value },
    fieldState: { error },
  } = useController({ name, rules, defaultValue: "" });

  return (
    <label className={classes.inputContainer}>
      <Typography>{label}</Typography>
      <TextField
        slotProps={{ htmlInput: { "data-testid": testId } }}
        name={name}
        value={value}
        onChange={onChange}
        helperText={error?.message}
        error={!!error}
        multiline={multiline}
        rows={rows}
        {...props}
        className={classes.inputText}
        placeholder={placeholder ?? label}
        {...(adornmentImg && {
          slotProps: {
            input: {
              endAdornment: (
                <InputAdornment position="end">{adornmentImg}</InputAdornment>
              ),
            },
          },
        })}
      />
    </label>
  );
};
