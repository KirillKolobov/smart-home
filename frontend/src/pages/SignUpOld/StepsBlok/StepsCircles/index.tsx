import { Typography } from "@mui/material";
import classes from "./styles.module.scss";

type CircleProps = {
  number: number;
  active: boolean;
};

type StepsCirclesProps = {
  activeStep: number;
};

export const StepsCircles = ({ activeStep }: StepsCirclesProps) => (
  <div className={classes.steps}>
    <div className={classes.step}>
      <Circle number={1} active={activeStep === 1} />
    </div>
    <div className={classes.separator}></div>
    <div className={classes.step}>
      <Circle number={2} active={activeStep === 2} />
    </div>
  </div>
);

const Circle = ({ number, active }: CircleProps) => {
  return (
    <span className={`${classes.circle} ${active ? classes.circleActive : ""}`}>
      <Typography
        variant="p1"
        color={active ? "primary" : "neutralInfo"}
        className={classes.stepNumber}
      >
        {number}
      </Typography>
    </span>
  );
};
