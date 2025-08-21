import classes from "./styles.module.scss";
import { Box, Step, StepLabel, Stepper, Typography } from "@mui/material";

const steps = ["1", "2"];

type SignUpStepsProps = {
  activeStep: number;
};

export const SignUpSteps = ({ activeStep }: SignUpStepsProps) => {
  return (
    <Box className={classes.stepperContainer}>
      <Stepper activeStep={activeStep} className={classes.stepper}>
        {steps.map((label) => (
          <Step key={label}>
            <StepLabel></StepLabel>
          </Step>
        ))}
      </Stepper>
      <Box className={classes.stepLabelContainer}>
        <Typography
          className={classes.stepLabel}
          fontSize={12}
          color="text.secondary"
        >
          Личные данные
        </Typography>
        <Typography
          className={classes.stepLabel}
          fontSize={12}
          color="text.secondary"
        >
          Безопасность
        </Typography>
      </Box>
    </Box>
  );
};
