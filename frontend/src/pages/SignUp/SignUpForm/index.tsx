import classes from "./styles.module.scss";
import { Link, Paper, Typography } from "@mui/material";
import { FormProvider, useForm, type SubmitHandler } from "react-hook-form";
import { FirstStep } from "./FirstStep";
import { SecondStep } from "./SecondStep";
import type { IFormData, ServerValidationErrors } from "../types";
import { useMutation } from "@tanstack/react-query";
import type { IUser } from "../../../types";
import { AxiosError } from "axios";
import { signUpUser } from "../../../services/authService";

type SignUpFormProps = {
  activeStep: number;
  handleChangeStep: () => void;
};

export const SignUpForm = ({
  activeStep,
  handleChangeStep,
}: SignUpFormProps) => {
  const form = useForm<IFormData>();
  const { mutate } = useMutation<
    IUser,
    AxiosError<ServerValidationErrors>,
    IFormData
  >({
    mutationFn: signUpUser,
    onError: (error) => {
      const errors = error.response?.data.errors;
      if (errors) {
        Object.entries(errors).forEach(([key, value]) => {
          form.setError(key as keyof IFormData, { message: value });
        });
      }
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
        {activeStep === 0 ? (
          <FirstStep handleChangeStep={handleChangeStep} />
        ) : (
          <SecondStep handleChangeStep={handleChangeStep} />
        )}
        <Typography className={classes.signInText} color="text.secondary">
          Уже есть аккаунт? <Link>Войти</Link>
        </Typography>
      </Paper>
    </FormProvider>
  );
};
