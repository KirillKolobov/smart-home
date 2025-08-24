import classes from "./styles.module.scss";
import { Link, Paper, Typography } from "@mui/material";
import { FormProvider, useForm } from "react-hook-form";

import { FirstStep } from "./FirstStep";
import { SecondStep } from "./SecondStep";
import type { IFormData } from "../types";

type SignUpFormProps = {
  activeStep: number;
  handleChangeStep: () => void;
};

export const SignUpForm = ({
  activeStep,
  handleChangeStep,
}: SignUpFormProps) => {
  const form = useForm<IFormData>();
  return (
    <FormProvider {...form}>
      <Paper
        className={classes.formContainer}
        component={"form"}
        elevation={0}
        onSubmit={form.handleSubmit((data) => console.log(data))}
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
