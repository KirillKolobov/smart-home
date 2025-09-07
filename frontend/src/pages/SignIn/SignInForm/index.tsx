import classes from "./styles.module.scss";
import {
  Box,
  Button,
  IconButton,
  Link,
  Paper,
  Typography,
} from "@mui/material";
import { FormProvider, useForm, type SubmitHandler } from "react-hook-form";
import type { AuthResponse, IFormData, ServerValidationErrors } from "../types";
import { useMutation } from "@tanstack/react-query";
import { AxiosError } from "axios";
import { InputText } from "../../../components/InputText";
import {
  DEFAULT_MAX_LENGTH_MSG,
  DEFAULT_REQUIRED_MSG,
} from "../../../constants";
import { useState } from "react";
import { InputCheckbox } from "../../../components/InputCheckbox";
import { signInUser } from "../../../services/authService";
import { setToken } from "../../../utils/localStorage";
import { useNavigate } from "react-router";

export const SignInForm = () => {
  const [showPassword, setShowPassword] = useState(false);
  const handleClickShowPassword = () => setShowPassword((show) => !show);
  const navigate = useNavigate();

  const form = useForm<IFormData>();
  const { mutate } = useMutation<
    AuthResponse,
    AxiosError<ServerValidationErrors>,
    IFormData
  >({
    mutationFn: signInUser,
    onError: (error) => {
      const errors = error.response?.data.errors;
      if (errors) {
        Object.entries(errors).forEach(([key, value]) => {
          form.setError(key as keyof IFormData, { message: value });
        });
      }
    },
    onSuccess: (data) => {
      setToken(data.token);
      navigate("/");
    },
  });
  const onSubmit: SubmitHandler<IFormData> = (data) => mutate(data);

  return (
    <FormProvider {...form}>
      <Paper
        className={classes.formContainer}
        component={"form"}
        elevation={0}
        onSubmit={form.handleSubmit(onSubmit)}
      >
        <InputText
          name="email"
          label="Email адрес"
          rules={{
            required: DEFAULT_REQUIRED_MSG,
            maxLength: { message: "Too many symbols", value: 20 },
          }}
          type="email"
          adornmentImg={"📧"}
        />
        <InputText
          testId="password"
          name="password"
          label="Пароль"
          rules={{
            required: DEFAULT_REQUIRED_MSG,
            maxLength: { message: DEFAULT_MAX_LENGTH_MSG, value: 20 },
          }}
          placeholder="Минимум 8 символов"
          type={showPassword ? "text" : "password"}
          adornmentImg={
            <IconButton
              onClick={handleClickShowPassword}
              edge="end"
              size="small"
            >
              {showPassword ? "👁️" : "👁️‍🗨️"}
            </IconButton>
          }
        />
        <Box className={classes.rememberContainer}>
          <InputCheckbox name="remember" label="Запомнить меня" />
          <Link>Забыли пароль?</Link>
        </Box>
        <Button type="submit" variant="contained">
          Войти
        </Button>
        <Typography className={classes.signInText} color="text.secondary">
          Нет аккаунта? <Link>Зарегистрироваться</Link>
        </Typography>
      </Paper>
    </FormProvider>
  );
};
