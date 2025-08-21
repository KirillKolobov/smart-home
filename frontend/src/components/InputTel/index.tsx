import {
  type BaseTextFieldProps,
  InputAdornment,
  Typography,
} from "@mui/material";
import classes from "./styles.module.scss";
import { type RegisterOptions, useController } from "react-hook-form";
import { MuiTelInput } from "mui-tel-input";

interface IProps extends BaseTextFieldProps {
  name: string;
  label?: string;
  rules?: RegisterOptions;
  rows?: number;
  testId?: string;
  adornmentImg?: string;
}

export const InputTel = ({
  placeholder,
  label,
  name,
  rules,
  rows,
  adornmentImg,
  ...props
}: IProps) => {
  const {
    field: { onChange, value },
    fieldState: { error },
  } = useController({ name, rules, defaultValue: "" });
  return (
    <label className={classes.inputContainer}>
      <Typography>{label}</Typography>
      <MuiTelInput
        // slotProps={{ htmlInput: { "data-testid": testId } }}
        name={name}
        value={value}
        onChange={onChange}
        helperText={error?.message}
        error={!!error}
        rows={rows}
        {...props}
        className={classes.inputText}
        placeholder={placeholder ?? label}
        defaultCountry="RU"
        onlyCountries={["RU", "KZ", "BY", "US"]}
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
